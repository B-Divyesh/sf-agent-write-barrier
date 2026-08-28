# Handoff — Agent Write Barrier v0.1.0

## Release decision: PASS

Independent verification of candidate `9b168821aad5f7b6ae8da87f221ea806c53276d9` **PASSed** on 2026-08-28 UTC. The live deployment at <https://agent-write-barrier.sociobot.in/> exactly matches the candidate build. Full evidence, commands, live SHA-256 comparisons, and defect assessment are in `.factory/verification-3.md`.

No Critical, High, Medium, or Low defects were found. The known platform limit is intentional: enforced mode requires Linux Landlock ABI 3+; unsupported systems fail closed unless the user explicitly chooses labelled audit-only fallback.

## Latest verification summary

`npm ci`, `npm test` (8 Playwright tests plus Rust/type checks), `npm run lint`, `npm run build`, `cargo package`, `cargo test --doc`, and high-severity audit all passed. A clean installed package consumer verified normal writes, ignored files, Git metadata receipt coverage, an outside-policy write denial, successful recovery, JSON operations, and invalid-input exit 64 handling. Live browser checks passed on desktop and 390px mobile: keyboard activation/focus, reduced motion, zero serious/critical Axe findings, no errors, same-origin requests, service-worker control, and offline reload. Live headers enforce immutable hashed asset caching, revalidatable worker/HTML, CSP and related response hardening. Mobile Lighthouse: 99 performance / 100 accessibility / 100 best practices / 100 SEO.

## Earlier PWA repair context

## Root cause and repair

Independent verification (`.factory/verification-2.md`) reproduced a stale
existing PWA client after a content-only release: the old worker always used
the `awb-site-v1` cache and served navigations cache-first, so unchanged asset
URLs meant `sw.js` did not change and the prior HTML could persist.

Live verification of the first repair exposed a second installation failure:
Azure Static Web Apps consumes `staticwebapp.config.json` at deploy time and
returns it as 404 at runtime. The worker had included that deployment-only file
in `cache.addAll()`, causing its install event to reject and leaving no active
service worker.

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
- Deployment-only `staticwebapp.config.json` is excluded from the precache, so
  Azure's intentional runtime 404 cannot prevent worker installation.

## Regression coverage

`site/tests/pwa-deploy.spec.ts` uses a real browser and local HTTP server to
install v1, alter only `index.html` to v2 while retaining the exact precache
URL list, regenerate `sw.js`, call `registration.update()`, and assert that
the existing controlled client renders v2. It additionally asserts the exact
Azure cache and response-policy configuration, and writes a mock
`staticwebapp.config.json` to assert that deployment-only metadata is not in
the precache. The regular offline browser test asserts that the site itself
registers and controls the page before an offline reload; it does not manually
install a worker.

## Verification run (2026-08-28 UTC)

All commands below ran from this checkout after a fresh `npm ci` (23 packages;
`npm audit --audit-level=high`: 0 vulnerabilities).

| Check | Result |
| --- | --- |
| `npm test` | PASS — strict TypeScript check; 2 Rust unit tests; 4 CLI integrations, including the 15 attempted escape-write shapes; and 8 Playwright tests. Browser coverage includes desktop semantics/axe, keyboard activation and live status, 390×844 mobile overflow/target sizing, privacy and terms axe, offline reload, and existing-client PWA content-only update. |
| `npm run lint` | PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`. |
| `npm run build` | PASS — produced `dist/site` and `dist/bin/awb`; the site precaches 16 runtime files. |
| `cargo package` | PASS — package verification passed; ready-to-publish archive is `target/package/agent-write-barrier-0.1.0.crate` (29,008 bytes). |
| `cargo test --doc` | PASS — 0 doctests. |
| Packaged consumer | PASS — unpacked the generated `.crate`, installed it into an isolated Cargo prefix, then completed `awb init`, `check --json`, `run --receipt … --json -- sh -c 'printf consumer > consumer.txt'`, and `inspect --json` in a fresh worktree. |
| Live URL smoke | PASS — `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 821 ms with no browser errors, the expected title, `lang=en`, one `h1`, `main`, no missing image alt, and no unlabeled button. Production `index.html` SHA-256 is `744f5e3c7e3769d0fc84d22e202252ffcb5be99be8b73742cbffdcb2e5458c91`, exactly matching `dist/site/index.html`. |
| Live PWA/update | PASS — the production worker excludes `staticwebapp.config.json`; a fresh Chromium profile registered and was controlled by it, then completed an offline reload with the homepage heading and offline bar visible and no console errors. |
| Live accessibility, keyboard, mobile, privacy | PASS — Axe reported 0 serious/critical violations on `/`, `/privacy/`, and `/terms/`; Enter ran the demo and rendered its blocked result; at 390×844, overflow was 0 px and the install control was 49.59 px high. All browser requests were same-origin. |
| Lighthouse 13 mobile | PASS — Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 985 ms, CLS 0.055, TBT 18 ms, transfer 105,017 B. |
| Azure response policy | PASS — hashed JS and hero return `public, max-age=31536000, immutable`; `/sw.js` returns `no-cache`; production supplies the configured same-origin CSP, `X-Frame-Options: DENY`, Permissions-Policy, and Cross-Origin-Resource-Policy. |

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
- Deployment used `/opt/fleet/lib/deploy-static.sh agent-write-barrier
  dist/site`; Azure deployment ID `efb6118a-a164-49a8-9d89-d60343cbb0df`
  completed successfully.
