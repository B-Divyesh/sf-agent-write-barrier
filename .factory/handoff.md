# Handoff — Agent Write Barrier v0.1.0 repair

## Release decision

The failed PWA release candidate has been repaired and is ready for the
factory static deployment. This handoff is for the repair based at
`d21ae74edb5d8aaed603268601703b8c34fa9fea`; the final repair commit records
the verification evidence below.

## Root cause and repair

Independent verification (`.factory/verification-2.md`) reproduced a stale
existing PWA client after a content-only release: the old worker always used
the `awb-site-v1` cache and served navigations cache-first, so unchanged asset
URLs meant `sw.js` did not change and the prior HTML could persist.

- `scripts/build-sw.mjs` now fingerprints every precached URL **and its
  content** into the worker cache name. A content-only HTML release changes
  `sw.js`, installs a new worker, and removes the old cache.
- Navigations are network-first, retaining the cached document only for an
  offline fallback. The registration opts out of the HTTP cache and begins
  immediately instead of depending on a late `load` listener.
- `site/public/staticwebapp.config.json` supplies Azure Static Web Apps-native
  cache rules: hashed assets and the immutable hero use one-year immutable
  caching; HTML revalidates; `sw.js` uses `no-cache`. It also applies the
  same-origin CSP and response hardening policy.

## Regression coverage

`site/tests/pwa-deploy.spec.ts` uses a real browser and local HTTP server to
install v1, alter only `index.html` to v2 while retaining the exact precache
URL list, regenerate `sw.js`, call `registration.update()`, and assert that
the existing controlled client renders v2. It additionally asserts the exact
Azure cache and response-policy configuration. The regular offline browser
test asserts that the site itself registers and controls the page before an
offline reload; it does not manually install a worker.

## Verification run (2026-08-28 UTC)

All commands below ran from this checkout after a fresh `npm ci` (23 packages;
`npm audit --audit-level=high`: 0 vulnerabilities).

| Check | Result |
| --- | --- |
| `npm test` | PASS — strict TypeScript check; 2 Rust unit tests; 4 CLI integrations, including the 15 attempted escape-write shapes; and 8 Playwright tests. Browser coverage includes desktop semantics/axe, keyboard activation and live status, 390×844 mobile overflow/target sizing, privacy and terms axe, offline reload, and existing-client PWA content-only update. |
| `npm run lint` | PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`. |
| `npm run build` | PASS — produced `dist/site` and `dist/bin/awb`; the site precaches 17 files. |
| `cargo package` | PASS — package verification passed; ready-to-publish archive is `target/package/agent-write-barrier-0.1.0.crate` (28,844 bytes). |
| `cargo test --doc` | PASS — 0 doctests. |
| Packaged consumer | PASS — unpacked the generated `.crate`, installed it into an isolated Cargo prefix, then completed `awb init`, `check --json`, `run --receipt … --json -- sh -c 'printf consumer > consumer.txt'`, and `inspect --json` in a fresh worktree. |

## Build, run, and deploy

```sh
npm ci
npm test
npm run lint
npm run build
cargo package

# Factory static deployment (Azure Static Web Apps)
/opt/fleet/lib/deploy-static.sh agent-write-barrier dist/site
```

The factory owns registry credentials; do not publish the crate. `cargo
package` creates the ready-to-publish archive. The deployable static artifact
is `dist/site`.

## Limits and follow-up

- Enforced mode requires Linux Landlock ABI 3+ (roughly Linux 6.2+). Other
  platforms fail closed unless the user explicitly selects the labelled,
  non-enforcing audit fallback.
- The write boundary does not isolate network access, other processes,
  devices, already-open file descriptors, kernel vulnerabilities, or hostile
  same-user processes. Receipts contain local paths and hashes and should be
  reviewed before sharing.
- Post-deployment URL identity, response headers, PWA control, and Lighthouse
  evidence are appended after the factory deployment completes.
