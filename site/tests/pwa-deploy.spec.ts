import { createServer } from 'node:http';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { basename, join, resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { test, expect } from '@playwright/test';
import { buildServiceWorker } from '../../scripts/build-sw.mjs';

const siteRoot = resolve(import.meta.dirname, '..');

test('a content-only release refreshes an existing controlled client', async ({ page }) => {
  const root = await mkdtemp(join(tmpdir(), 'awb-pwa-update-'));
  let version = 'v1';
  await writeFile(join(root, 'index.html'), `<!doctype html><title>${version}</title><h1>${version}</h1>`);
  const first = await buildServiceWorker(root);

  const server = createServer(async (request, response) => {
    const pathname = new URL(request.url ?? '/', 'http://127.0.0.1').pathname;
    const requested = pathname === '/' ? 'index.html' : pathname.slice(1);
    const file = resolve(root, requested);
    if (file !== root && !file.startsWith(`${root}/`)) {
      response.writeHead(403).end();
      return;
    }
    try {
      const body = await readFile(file);
      response.writeHead(200, {
        'Content-Type': basename(file) === 'sw.js' ? 'application/javascript' : 'text/html',
        'Cache-Control': 'no-cache'
      }).end(body);
    } catch {
      response.writeHead(404).end();
    }
  });
  await new Promise<void>((done) => server.listen(0, '127.0.0.1', done));
  const address = server.address();
  if (!address || typeof address === 'string') throw new Error('Test server did not start');
  const origin = `http://127.0.0.1:${address.port}`;

  try {
    await page.goto(origin);
    await page.evaluate(() => navigator.serviceWorker.register('/sw.js', { updateViaCache: 'none' }));
    await page.evaluate(() => navigator.serviceWorker.ready.then(() => true));
    await page.reload();
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('v1');

    version = 'v2';
    await writeFile(join(root, 'index.html'), `<!doctype html><title>${version}</title><h1>${version}</h1>`);
    const second = await buildServiceWorker(root);
    expect(second.assets).toEqual(first.assets);
    expect(second.cacheVersion).not.toBe(first.cacheVersion);

    await page.evaluate(async () => {
      const registration = await navigator.serviceWorker.getRegistration();
      await registration?.update();
    });
    await page.reload();
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('v2');
  } finally {
    await new Promise<void>((done, fail) => server.close((error) => error ? fail(error) : done()));
    await rm(root, { recursive: true, force: true });
  }
});

test('Azure deploy configuration keeps hashed assets immutable and the worker revalidatable', async () => {
  const config = JSON.parse(await readFile(join(siteRoot, 'public/staticwebapp.config.json'), 'utf8'));
  expect(config.globalHeaders['Cache-Control']).toBe('public, max-age=0, must-revalidate');
  expect(config.globalHeaders['Content-Security-Policy']).toContain("default-src 'self'");
  expect(config.globalHeaders['Content-Security-Policy']).toContain("font-src 'self' data:");
  expect(config.globalHeaders['X-Frame-Options']).toBe('DENY');
  expect(config.routes).toEqual(expect.arrayContaining([
    expect.objectContaining({
      route: '/assets/*',
      headers: { 'Cache-Control': 'public, max-age=31536000, immutable' }
    }),
    expect.objectContaining({
      route: '/hero-boundary.webp',
      headers: { 'Cache-Control': 'public, max-age=31536000, immutable' }
    }),
    expect.objectContaining({
      route: '/sw.js',
      headers: { 'Cache-Control': 'no-cache' }
    })
  ]));
});
