# Independent verification 2 — FAIL

**Candidate:** `1475dc1d617ff8e80fdf99695916f5da0517189e`  
**Live URL:** https://agent-write-barrier.sociobot.in/  
**Verified:** 2026-08-28 (UTC)  
**Scope:** clean-checkout CLI, package-consumer, static site, production deployment, accessibility, privacy, performance, response policy, and PWA behaviour.

## Decision

**FAIL.** The CLI and site are otherwise credible and the live files match the candidate exactly, but the PWA does not reliably update after a content-only deploy. Its fixed cache name can serve a stale shell indefinitely. The production host also does not honor the candidate's long-lived immutable-cache policy for hashed assets. These fail the explicit PWA-update and caching checks.

## Clean-checkout evidence

A detached clean worktree at the candidate was used. `npm ci` installed 23 packages from `package-lock.json` and reported 0 vulnerabilities.

| Check | Result |
| --- | --- |
| `npm test` | PASS — 2 Rust unit, 4 CLI integration (including all 15 escape-write shapes), and 6 Playwright tests passed. |
| `cargo fmt --check` | PASS |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| `npm run build` | PASS — produced `dist/site` and `dist/bin/awb` (1.6 MiB). |
| `cargo package` | PASS — produced `target/package/agent-write-barrier-0.1.0.crate` (28 KiB). |
| `cargo test --doc` | PASS (0 doctests present). |
| `npm audit --audit-level=high` | PASS — 0 vulnerabilities. |

There is no standalone TypeScript type-check or JavaScript lint script/configuration in the repository; Vite's production compilation and the available Rust formatter/linter were run.

## CLI and consumer evidence

The packaged crate was installed into a new `cargo install --root` prefix from `target/package/agent-write-barrier-0.1.0`, not from the repository binary. The installed `awb 0.1.0` completed the documented `init`, `check --json`, `run --receipt ... --json -- sh -c ...`, and `inspect --json` flow in a new consumer directory. Its receipt contained normal source, ignored `target/cache.pyc`, and `.git/HEAD` changes.

Independent boundary/recovery exercise on Linux Landlock ABI 3:

- Allowed `permitted.txt` was created inside the policy root.
- An attempted write to a pre-existing `/tmp/awb-boundary-escape.*` file failed with `Permission denied`; its size remained 0.
- Receipt reported `enforcement: "enforced"`, ABI 3, and command exit 0.
- Duplicate `init` and a missing policy each exited 64 with actionable errors; `init --force` recovered successfully.

This agrees with the included 15-shape suite: create, remove, move, copy, hard-link, symlink, chmod, truncate, touch, install, `dd`, nested directory, FIFO, and in-place edit outside policy were blocked, while the normal worktree flow succeeded and recorded ignored plus Git-internal files.

## Live deployment and browser evidence

The live root document SHA-256 is `4e521c85549e917fdef4903377a12d3a7bb64ffaff8b9e369d942d7fb8353c81`, exactly matching the candidate's `dist/site/index.html`. Live `assets/main-BaV5o1we.js` and `assets/style-CmWzlckE.css` also exactly match the candidate artifacts. Therefore this is not a stale/deployment-mismatch false failure.

Fresh Chromium checks against the live URL found:

- Desktop: title, `lang=en`, one `h1`, one `main`, and zero images without `alt`.
- Axe: no serious or critical issues on `/`, `/privacy/`, or `/terms/`.
- Keyboard: Enter activated the receipt simulation; the focused primary button computed to a visible `3px` amber outline.
- Mobile (390 × 844): 0 px horizontal overflow, 16 px body text, and 49.59 px install target height.
- Reduced motion: simulation reached final state within 30 ms; computed animation and transition duration were `0.01ms`.
- No console errors or page errors. Browser requests stayed same-origin (`https://agent-write-barrier.sociobot.in`); the documentation site made no runtime third-party request.
- PWA: the live service worker controlled the page and an offline reload displayed the cached shell plus the offline bar.

Build budgets pass: initial JS 3,462 B, CSS 18,370 B, fonts 72,420 B total, hero WebP 67,018 B. All are below the 200 KB / 50 KB / 120 KB / 300 KB limits.

## Defects

### High — service-worker update can leave content-only releases stale

`scripts/build-sw.mjs` always emits `const CACHE = 'awb-site-v1'`. It uses cache-first navigation handling. A content-only update that does not change the precache URL list leaves `sw.js` byte-identical, so browsers do not install a new worker and keep returning the prior cached `/` response.

This was reproduced with the candidate's actual worker logic in an isolated HTTP deployment simulation: install shell version `v1`, change only the HTML response to `v2`, call `registration.update()`, and reload. The rendered result remained `v1` (`STALE_CONTENT_REPRODUCED`). This violates the requested service-worker update check. Generate a cache version from the precache/content manifest (or use network-first/revalidation for navigations) and verify an existing client receives a content-only deployment.

### Medium — production does not apply the candidate's immutable asset cache policy

The candidate's `site/public/_headers` requests `Cache-Control: public, max-age=31536000, immutable` for `/assets/*` and the hero. Production instead returns `cache-control: public, must-revalidate, max-age=30` for the hashed JS, CSS, service worker, and HTML. The live asset content matches the candidate but its cache policy does not. Configure the deploy host to honor `_headers` (or its native equivalent) and retest the response headers.

### Informational — response-policy hardening opportunity

Production supplies HSTS, `nosniff`, and a strict referrer policy, but no Content-Security-Policy, frame-ancestors/X-Frame-Options, Permissions-Policy, or explicit cross-origin resource policy was observed. This was not used as a release blocker because the product contract does not require a CSP, but it is a worthwhile deployment hardening follow-up.

## Privacy and limits

Source and browser inspection found no telemetry or runtime third-party script/font requests. Fonts are self-hosted. The CLI contains no network client path in its product source and correctly documents Landlock/fallback limits; its receipt deliberately includes local paths and hashes, so users should not share receipts indiscriminately.

## Re-run

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
cargo package
```

Then install `target/package/agent-write-barrier-0.1.0` into a fresh Cargo prefix and exercise `awb init`, `awb check --json`, `awb run`, and `awb inspect --json` from a new consumer worktree. Recheck the public URL and, after fixing the High defect, perform an existing-client service-worker update test before release.
