import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const repository = resolve(import.meta.dirname, '../..');
const binary = resolve(repository, 'target/debug/awb');

function temp(prefix: string): string {
  return mkdtempSync(join(tmpdir(), prefix));
}

function run(args: string[], cwd: string, extraEnv: Record<string, string> = {}) {
  return spawnSync(binary, args, {
    cwd,
    encoding: 'utf8',
    timeout: 15_000,
    env: { ...process.env, ...extraEnv }
  });
}

function demo(cwd: string) {
  const result = run(['demo'], cwd);
  expect(result.status, result.stderr).toBe(0);
  const match = result.stdout.match(/^Receipt: (.+)$/m);
  expect(match).not.toBeNull();
  const receiptPath = match![1].trim();
  return { result, receiptPath, receipt: JSON.parse(readFileSync(receiptPath, 'utf8')) };
}

test('@claim:demo-isolation bundled demo uses an isolated temporary project', async () => {
  const caller = temp('awb-claim-caller-');
  writeFileSync(join(caller, 'keep.txt'), 'developer file\n');
  const before = readdirSync(caller);
  const { result, receiptPath } = demo(caller);
  expect(readdirSync(caller)).toEqual(before);
  expect(readFileSync(join(caller, 'keep.txt'), 'utf8')).toBe('developer file\n');
  expect(receiptPath.startsWith(tmpdir())).toBe(true);
  expect(receiptPath.startsWith(caller)).toBe(false);
  expect(result.stdout).toContain('BLOCKED: ../blocked-agent.conf stayed outside allowed paths');
  const alias = run(['--demo'], caller);
  expect(alias.status, alias.stderr).toBe(0);
  rmSync(caller, { recursive: true, force: true });
  rmSync(resolve(receiptPath, '..'), { recursive: true, force: true });
});

test('@claim:write-boundary-15 Landlock blocks all fifteen documented outside-write shapes', async () => {
  const output = execFileSync('cargo', ['test', 'blocks_fifteen_escape_write_shapes', '--', '--exact'], { cwd: repository, encoding: 'utf8', timeout: 30_000 });
  expect(output).toContain('test blocks_fifteen_escape_write_shapes ... ok');
});

test('@claim:receipt-coverage demo receipt lists content, ignored, deleted, link, and Git metadata changes', async () => {
  const caller = temp('awb-claim-receipt-');
  const { receipt, receiptPath } = demo(caller);
  expect(receipt.schema_version).toBe(1);
  expect(receipt.enforcement).toBe('enforced');
  expect(receipt.landlock_abi).toBeGreaterThanOrEqual(3);
  expect(receipt.command).toEqual(expect.arrayContaining(['sh', '-c']));
  const changes = receipt.changes as Array<{ path: string; change: string; before?: unknown; after?: { sha256?: string; symlink_target?: string; mode?: number } }>;
  expect(changes.some((item) => item.path.endsWith('src/config.rs') && item.change === 'modified' && item.after?.sha256)).toBe(true);
  expect(changes.some((item) => item.path.endsWith('target/config.pyc') && item.change === 'created')).toBe(true);
  expect(changes.some((item) => item.path.endsWith('obsolete.txt') && item.change === 'deleted')).toBe(true);
  expect(changes.some((item) => item.path.endsWith('current-config') && item.after?.symlink_target === 'src/config.rs')).toBe(true);
  expect(changes.some((item) => item.path.endsWith('.git/hooks/pre-commit') && item.change === 'modified')).toBe(true);
  rmSync(caller, { recursive: true, force: true });
  rmSync(resolve(receiptPath, '..'), { recursive: true, force: true });
});

test('@claim:policy-safety policies resolve from their file and never add home or credential paths', async () => {
  const root = temp('awb-claim-policy-');
  const project = join(root, 'project');
  const elsewhere = join(root, 'elsewhere');
  mkdirSync(project); mkdirSync(elsewhere);
  const isolatedHome = join(root, 'home'); mkdirSync(isolatedHome);
  const init = run(['init'], project, { HOME: isolatedHome });
  expect(init.status).toBe(0);
  const policy = JSON.parse(readFileSync(join(project, '.awb-policy.json'), 'utf8'));
  expect(policy).toEqual({ version: 1, allow_write: ['.'], watch: ['.'] });
  expect(JSON.stringify(policy)).not.toContain(isolatedHome);
  const check = run(['check', '--json', '--policy', '../project/.awb-policy.json'], elsewhere, { HOME: isolatedHome });
  expect(check.status, check.stderr).toBe(0);
  expect(JSON.parse(check.stdout).allowed_write).toEqual([project]);
  writeFileSync(join(root, 'root.json'), JSON.stringify({ version: 1, allow_write: ['/'], watch: ['/'] }));
  expect(run(['check', '--policy', join(root, 'root.json')], root, { HOME: isolatedHome }).status).toBe(64);
  writeFileSync(join(root, 'home.json'), JSON.stringify({ version: 1, allow_write: [isolatedHome], watch: [isolatedHome] }));
  expect(run(['check', '--policy', join(root, 'home.json')], root, { HOME: isolatedHome }).status).toBe(64);
  const two = join(root, 'two'); const outside = join(root, 'outside'); mkdirSync(two); mkdirSync(outside);
  writeFileSync(join(root, 'multi.json'), JSON.stringify({ version: 1, allow_write: ['project', 'two'], watch: [root] }));
  const multi = run(['run', '--policy', join(root, 'multi.json'), '--receipt', join(root, 'multi-receipt.json'), '--', 'sh', '-c', 'printf one > project/a; printf two > two/b; (printf no > outside/c) 2>/dev/null || true'], root);
  expect(multi.status, multi.stderr).toBe(0);
  expect(readFileSync(join(project, 'a'), 'utf8')).toBe('one');
  expect(readFileSync(join(two, 'b'), 'utf8')).toBe('two');
  expect(() => readFileSync(join(outside, 'c'))).toThrow();
  rmSync(root, { recursive: true, force: true });
});

test('@claim:process-contract preserves cwd, streams, exit status, reads, children, and cleans private temp data', async () => {
  const root = temp('awb-claim-process-');
  const work = join(root, 'work'); mkdirSync(work);
  writeFileSync(join(root, 'outside.txt'), 'readable\n');
  writeFileSync(join(root, 'policy.json'), JSON.stringify({ version: 1, allow_write: ['work'], watch: ['work'] }));
  const receipt = join(root, 'receipt.json');
  const script = 'pwd; cat ../outside.txt; sh -c "printf nested > nested.txt"; printf child-error >&2; printf temp > "$TMPDIR/probe"; exit 23';
  const result = run(['run', '--policy', join(root, 'policy.json'), '--receipt', receipt, '--', 'sh', '-c', script], work);
  expect(result.status).toBe(23);
  expect(result.stdout).toBe(`${work}\nreadable\n`);
  expect(result.stderr).toContain('child-error');
  expect(readFileSync(join(work, 'nested.txt'), 'utf8')).toBe('nested');
  const body = JSON.parse(readFileSync(receipt, 'utf8'));
  expect(body.command_exit).toBe(23);
  expect(body.enforcement).toBe('enforced');
  const sessionTemp = join(tmpdir(), `agent-write-barrier-${body.session_id}`);
  expect(() => readdirSync(sessionTemp)).toThrow();
  const inspect = run(['inspect', '--json', receipt], root);
  expect(inspect.status).toBe(0);
  expect(JSON.parse(inspect.stdout).command_exit).toBe(23);
  const signalReceipt = join(root, 'signal.json');
  const signaled = run(['run', '--policy', join(root, 'policy.json'), '--receipt', signalReceipt, '--', 'sh', '-c', 'kill -TERM $$'], work);
  expect(signaled.status).toBe(143);
  const signalBody = JSON.parse(readFileSync(signalReceipt, 'utf8'));
  expect(signalBody.command_exit).toBe(143);
  expect(() => readdirSync(join(tmpdir(), `agent-write-barrier-${signalBody.session_id}`))).toThrow();
  rmSync(root, { recursive: true, force: true });
});

test('@claim:enforcement-modes unavailable enforcement fails closed and explicit fallback says audit-only', async () => {
  const root = temp('awb-claim-modes-');
  const work = join(root, 'work'); mkdirSync(work);
  writeFileSync(join(root, 'policy.json'), JSON.stringify({ version: 1, allow_write: ['work'], watch: [root] }));
  const marker = join(root, 'outside.txt');
  const env = { AWB_TEST_DISABLE_LANDLOCK: '1' };
  const closed = run(['run', '--policy', join(root, 'policy.json'), '--', 'sh', '-c', `printf ran > ${marker}`], work, env);
  expect(closed.status).toBe(77);
  expect(() => readFileSync(marker)).toThrow();
  const receipt = join(root, 'audit.json');
  const audit = run(['run', '--allow-unsafe-fallback', '--policy', join(root, 'policy.json'), '--receipt', receipt, '--', 'sh', '-c', `printf observed > ${marker}`], work, env);
  expect(audit.status, audit.stderr).toBe(0);
  expect(readFileSync(marker, 'utf8')).toBe('observed');
  expect(JSON.parse(readFileSync(receipt, 'utf8')).enforcement).toBe('audit-only');
  expect(audit.stderr).toContain('writes are not blocked');
  rmSync(root, { recursive: true, force: true });
});

test('@claim:local-cli package defines one CLI and contains no telemetry or network client', async () => {
  const cargo = readFileSync(join(repository, 'Cargo.toml'), 'utf8');
  const source = readdirSync(join(repository, 'src')).map((name) => readFileSync(join(repository, 'src', name), 'utf8')).join('\n');
  expect((cargo.match(/\[\[bin\]\]/g) ?? [])).toHaveLength(1);
  expect(cargo).not.toMatch(/reqwest|hyper|ureq|telemetry|analytics/i);
  expect(source).not.toMatch(/TcpStream|UdpSocket|openai|analytics|telemetry/i);
  expect(run(['--version'], repository).stdout.trim()).toBe('awb 0.1.0');
});

test('@claim:license-source source, package metadata, sample files, and MIT license agree', async () => {
  const cargo = readFileSync(join(repository, 'Cargo.toml'), 'utf8');
  const license = readFileSync(join(repository, 'LICENSE'), 'utf8');
  expect(cargo).toContain('license = "MIT"');
  expect(cargo).toContain('https://github.com/B-Divyesh/sf-agent-write-barrier');
  expect(license).toContain('MIT License');
  expect(readFileSync(join(repository, 'examples/sample-project/src/config.rs'), 'utf8')).toContain('API_TIMEOUT_SECONDS');
});

test('@claim:site-privacy every route stays same-origin and uses no cookies or non-demo storage', async ({ page, context }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/definitely-missing-review-route']) {
    await page.goto(path);
    await expect(page.locator('h1')).toHaveCount(1);
  }
  expect(requests.every((url) => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
  expect(await context.cookies()).toEqual([]);
  const keys = await page.evaluate(() => Object.keys(localStorage));
  expect(keys.every((key) => key.startsWith('demo:awb:'))).toBe(true);
});

test('@claim:offline-guide cached routes reload offline and privacy control clears caches', async ({ page, context }) => {
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) await page.goto(path);
  await page.goto('/demo/');
  await page.evaluate(() => navigator.serviceWorker.ready.then(() => true));
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  await context.setOffline(true);
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) {
    await page.goto(path);
    await expect(page.locator('h1')).toHaveCount(1);
  }
  await context.setOffline(false);
  await page.goto('/privacy/');
  await page.getByRole('button', { name: 'Clear demo and offline data' }).click();
  await expect(page.locator('#clear-status')).toHaveText('Demo and offline data cleared.');
  expect(await page.evaluate(() => caches.keys())).toEqual([]);
});

test('@claim:routing-metadata routes set titles, metadata, focus, history, and a designed 404', async ({ page }) => {
  await page.goto('/');
  const demo = page.getByRole('link', { name: /Try it with sample data/ });
  await demo.click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page).toHaveTitle('Demo — Agent Write Barrier');
  await expect(page.locator('h1')).toBeFocused();
  await page.goBack();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.locator('h1')).toBeFocused();
  for (const path of ['/', '/demo/', '/privacy/', '/terms/']) {
    await page.goto(path);
    await expect(page.locator('meta[name="description"]')).toHaveAttribute('content', /.+/);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', new RegExp(path === '/' ? '\\/$' : `${path}$`));
    await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /og-boundary\.webp$/);
    await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
  }
  await page.goto('/definitely-missing-review-route');
  await expect(page).toHaveTitle('Page not found — Agent Write Barrier');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Return to a known path');
  const deploy = JSON.parse(readFileSync(join(repository, 'site/public/staticwebapp.config.json'), 'utf8'));
  expect(deploy.responseOverrides['404'].statusCode).toBe(404);
});

test('@claim:mobile-access every route passes serious Axe checks and keeps controls usable at 390px', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) {
    await page.goto(path);
    expect(await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth)).toBeLessThanOrEqual(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
  }
  await page.goto('/');
  for (const target of await page.locator('header a:visible, footer a:visible').all()) {
    const box = await target.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
  }
  const command = page.locator('pre[aria-label="Cargo install command"]');
  await command.focus();
  await expect(command).toBeFocused();
});

test('@claim:limits-consistent site, README, and help state the same security limits', async () => {
  const readme = readFileSync(join(repository, 'README.md'), 'utf8');
  const pages = ['index.html', 'terms/index.html'].map((name) => readFileSync(join(repository, 'site', name), 'utf8')).join('\n');
  const help = run(['--help'], repository).stdout;
  for (const text of [readme, pages, help]) {
    expect(text).toMatch(/does not isolate networks|does not make untrusted code safe|This does not isolate networks/);
  }
  expect(readme).toContain('lasting changes');
  expect(help).toContain('lasting changes');
  expect(readme).not.toContain('every persistent');
  expect(pages).not.toContain('See every write');
});
