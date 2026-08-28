# Independent verification 3 — PASS

**Candidate:** `9b168821aad5f7b6ae8da87f221ea806c53276d9`  
**Live URL:** <https://agent-write-barrier.sociobot.in/>  
**Verified:** 2026-08-28 UTC  
**Scope:** clean-checkout CLI/package consumer, static site, production deployment, PWA, privacy, accessibility, browser policies, and performance.

## Decision

**PASS.** Fresh evidence confirms the candidate implements the brief's useful job: a locally run agent command gets a constrained write surface, outside-policy writes are blocked on supported Linux, and a receipt captures normal, ignored, and Git metadata changes. The live static deployment exactly matches the candidate build and passes the requested browser, policy, PWA, privacy, and bundle checks.

## Clean-checkout quality gates

`git status` was clean at the candidate before verification. A fresh `npm ci` installed 23 packages; `npm audit --audit-level=high` reported 0 vulnerabilities.

| Command | Result |
| --- | --- |
| `npm test` | PASS — TypeScript check; 2 Rust unit tests; 4 CLI integrations; 8 Playwright tests. The CLI suite includes the normal documented workflow and all 15 escape-write shapes. |
| `npm run lint` | PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`. |
| `npm run build` | PASS — builds `dist/site` and `dist/bin/awb`. |
| `cargo package` | PASS — package verification passed; archive is 28.3 KiB compressed. |
| `cargo test --doc` | PASS — 0 doctests. |
| `npx playwright test` | PASS — 8/8 tests in 15.3 s, independently rerun after `npm test`. |

An initial browser-suite attempt encountered an already-running local Vite listener on port 4173, caused by a prior timed command in this verifier container. After that listener exited, the exact clean `npm test` command passed with exit 0; this was environmental contention, not a candidate failure.

## Packaged CLI / end-to-end evidence

I installed the packaged source at `target/package/agent-write-barrier-0.1.0` into a new Cargo install prefix, then used only that installed `awb 0.1.0` in a new consumer directory.

- `awb --help` presents `init`, `check`, `run`, and `inspect`, explicit Linux Landlock/fail-closed limits, and an exit-0 result.
- A policy allowing only `work/` while watching the consumer root produced `check --json` with `valid: true`, `enforcement_available: true`, Landlock ABI 3.
- `awb run --receipt … --json -- sh -c …` created an ordinary file, an ignored-style `work/target/cache.pyc`, and modified `work/.git/HEAD`. The enforced receipt recorded all of these changes with hashes/metadata and command exit 0.
- A representative boundary write to pre-existing `outside/escape.txt` failed with `Permission denied`; no file was created. A following permitted `work/recovered.txt` write succeeded, proving recovery after invalid agent input. Its receipt reported `enforcement: "enforced"` and ABI 3.
- Invalid recovery paths are actionable: duplicate `init` and `check --policy missing.json` both returned documented usage exit 64 with clear messages.

The repository integration suite additionally verifies creation, remove, move, copy, hard-link, symlink, chmod, truncate, touch, install, `dd`, nested-directory, FIFO, and in-place outside-policy attempts. This satisfies the brief's 15-attempt escape success measure on the available Linux Landlock platform.

## Live deployment, browser, privacy, and PWA

Production root SHA-256 is `744f5e3c7e3769d0fc84d22e202252ffcb5be99be8b73742cbffdcb2e5458c91`, identical to `dist/site/index.html`. The live `sw.js`, hero, hashed JS/CSS, privacy page, and terms page also each matched their candidate build SHA-256 exactly.

Fresh Chromium testing against the public URL found:

- Desktop semantics: title is `Agent Write Barrier — See every write. Bound every agent.`, `lang=en`, exactly one `<h1>`, exactly one `<main>`, and no image without `alt`.
- Axe reported no serious or critical issues on `/`, `/privacy/`, or `/terms/`.
- Keyboard-only Enter activated the receipt simulation and produced `BLOCKED · operation not permitted`. Its focused primary button has a visible 3px amber focus outline (`rgb(255, 202, 114)`).
- At 390 × 844, horizontal overflow is 0 px, body text is 16 px, and the install control is 49.59 px high.
- In reduced-motion mode, the demo reached final state in 14.2 ms; transition and animation durations compute to `0.01ms`.
- No console errors or page errors occurred. All observed runtime HTTP(S) requests were same-origin. Source/build inspection found no telemetry, analytics, cookies, or third-party fonts/scripts; fonts are bundled locally. The CLI has no product network client.
- A fresh profile registered and was controlled by the live service worker. While offline, the offline indicator appeared and a reload retained the homepage heading. The candidate's browser regression also passed an existing-client content-only release update: changed HTML with an unchanged precache URL list regenerated the worker/cache identity and the controlled client rendered v2.

## Response policy, cache, and budgets

Live headers match the candidate deployment configuration:

- HTML: `public, max-age=0, must-revalidate`; worker: `no-cache`; hashed JS/CSS and hero: `public, max-age=31536000, immutable`.
- HSTS, `nosniff`, strict referrer policy, `X-Frame-Options: DENY`, same-origin CSP, Permissions-Policy, and Cross-Origin-Resource-Policy are present.
- Candidate output: initial JS 3,450 B (1,660 B gzip), CSS 18,370 B (7,210 B gzip), five self-hosted font subsets total 72,420 B, hero 67,018 B. All meet the 200 KB JS / 50 KB CSS / 120 KB font / 300 KB image budgets.
- Mobile Lighthouse against the production build served locally: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 1,276 ms, CLS 0.057, TBT 96 ms, transfer 106,287 B.

## Defects by severity

- **Critical:** none.
- **High:** none.
- **Medium:** none.
- **Low:** none.
- **Informational:** The security boundary is correctly documented as Linux Landlock ABI 3+ only. On unsupported systems the default is fail-closed; explicit audit fallback is evidence rather than enforcement. This is an intentional, visible limitation aligned with the brief, not a defect.

## Re-run

```sh
npm ci
npm test
npm run lint
npm run build
cargo package
cargo test --doc
```

Then install `target/package/agent-write-barrier-0.1.0` into a clean Cargo prefix and exercise `awb init`, `check --json`, `run --receipt … --json -- sh -c …`, and `inspect --json` in a new consumer directory. Recheck the public URL in a fresh browser profile for service-worker control and offline reload.
