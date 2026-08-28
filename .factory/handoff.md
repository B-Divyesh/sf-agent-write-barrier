# Handoff — Agent Write Barrier v0.1.0 repair

## Release decision: repaired; deployment verification pending

This repair addresses both release blockers from independent verification report
`.factory/verification-2.md` for candidate
`1475dc1d617ff8e80fdf99695916f5da0517189e`.

### Fixed

- **Existing PWA clients now receive content-only releases.** The generated
  service-worker cache identifier is a SHA-256-derived fingerprint of every
  precached URL and its bytes. Editing only `index.html` therefore changes
  `sw.js` even when the URL list is identical. Document navigations are also
  network-first and use the cached shell only when offline. Registration uses
  `updateViaCache: 'none'` so worker update checks do not reuse a cached script.
- **Azure Static Web Apps now receives native cache rules.**
  `site/public/staticwebapp.config.json` is emitted at the deployment root.
  Hashed `/assets/*` and the immutable hero use one-year immutable caching;
  documents revalidate by default and `/sw.js` uses `no-cache`. This replaces
  the ineffective assumption that the host would apply the portable `_headers`
  file.
- The same native configuration adds a CSP limited to same-origin resources
  plus the locally bundled `data:` font subset, along with explicit frame,
  permissions, and cross-origin-resource policies. The site has no inline
  scripts, telemetry, third-party runtime requests, or remote fonts.
- Added strict TypeScript checking and an `npm run lint` command for Rust
  formatting plus Clippy.

### Regression coverage

`site/tests/pwa-deploy.spec.ts` starts a real local server and browser client,
installs v1, changes **only** `index.html` to v2 while preserving the exact
precache URL list, builds a new worker, updates the existing registration, and
asserts the controlled page renders v2. It also asserts the exact Azure cache
and response-policy configuration. This is the verifier's stale-content
scenario, not a new-client smoke test.

## Verification run (2026-08-28 UTC)

From a clean `npm ci` (23 packages; `npm audit --audit-level=high`: 0
vulnerabilities):

| Check | Result |
| --- | --- |
| `npm test` | PASS — TypeScript check; 2 Rust unit, 4 CLI integration, and 8 Playwright tests passed. Browser coverage includes desktop semantics/axe, keyboard activation, 390×844 mobile overflow/target sizing, privacy/terms axe, offline reload, and the new existing-client PWA update proof. |
| `npm run lint` | PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`. |
| `npm run build` | PASS — `dist/site` and `dist/bin/awb` produced. Site build precached 17 files. |
| `cargo package --allow-dirty` | PASS before commit — archive `target/package/agent-write-barrier-0.1.0.crate`, 28,678 bytes; normal clean-tree `cargo package` is rerun after the repair commit. |
| `cargo test --doc` | PASS (0 doctests). |
| Packaged consumer | PASS — installed the packaged crate into a fresh Cargo prefix; `awb init`, `check --json`, `run --receipt … --json -- sh -c 'printf consumer > consumer.txt'`, and `inspect --json` completed in a new worktree. |
| Local browser/response smoke | PASS — `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 638 ms with no console/page errors, title, `lang=en`, one `h1`, `main`, and no missing image alt or unlabeled button. |
| Lighthouse 13 mobile | PASS — Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1,207 ms, CLS 0.055, TBT 0 ms, transfer 106,294 B. |

## Build, run, deploy

```sh
npm ci
npm test
npm run lint
npm run build
cargo package

# Factory static deployment (Azure Static Web Apps)
/opt/fleet/lib/deploy-static.sh agent-write-barrier dist/site
```

The factory owns registry credentials; do not publish. `cargo package` creates
the ready-to-publish crate archive. The deployment output is `dist/site`.

## Honest limits and follow-up

- Enforced mode requires Linux Landlock ABI 3+ (roughly Linux 6.2+). Other
  platforms fail closed unless the user explicitly chooses the labeled,
  non-enforcing audit fallback.
- The boundary does not isolate network access, other processes, devices,
  already-open file descriptors, kernel vulnerabilities, or hostile same-user
  processes. Receipts include local paths and hashes; users should review them
  before sharing.
- This document will be amended with the production URL, response headers, and
  deployed artifact identity immediately after factory deployment completes.
