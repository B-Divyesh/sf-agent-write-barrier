# Handoff — Agent Write Barrier v0.1.0 — independent verification 2

## Release decision: FAIL

Candidate `1475dc1d617ff8e80fdf99695916f5da0517189e` was independently verified on 2026-08-28 UTC against https://agent-write-barrier.sociobot.in/. The live HTML, JS, and CSS are byte-for-byte the candidate build, so the result is based on fresh production evidence, not a stale deployment.

Do not release this candidate as a PWA until the service-worker update defect is fixed and the existing-client update path is retested. See `.factory/verification-2.md` for complete commands and evidence.

### Blocking defect (High)

`scripts/build-sw.mjs` emits the permanently named `awb-site-v1` cache and cache-first navigation handling. An isolated existing-client reproduction with the candidate worker showed a content-only deployment from `v1` to `v2` still rendered `v1` after `registration.update()` and reload (`STALE_CONTENT_REPRODUCED`). This fails the required service-worker update check.

### Deployment defect (Medium)

The candidate requests immutable cache headers for hashed assets in `site/public/_headers`; production returns `Cache-Control: public, must-revalidate, max-age=30` for those assets instead. Configure the host's native header rules and verify them live.

### What passed

- Clean detached checkout: `npm ci`, `npm test` (2 unit + 4 CLI integration + 6 Playwright), `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, exact `npm run build`, `cargo package`, `cargo test --doc`, and high-severity npm audit all passed.
- The packaged crate installed into a fresh consumer prefix and the public `awb 0.1.0` CLI completed init/check/run/inspect. A direct Landlock ABI 3 test allowed an in-policy write and denied an out-of-policy `/tmp` write; invalid policy and duplicate-init recovery paths returned expected exit 64 errors.
- Production matches candidate bytes; desktop/mobile, keyboard focus, reduced motion, offline reload, axe serious/critical, console/page errors, self-hosted assets, and runtime-request privacy checks passed. Initial JS/CSS/fonts/hero are all within budget.

### Known non-blocking hardening follow-up

Live headers include HSTS, nosniff, and strict referrer policy but no CSP, framing policy, Permissions-Policy, or explicit cross-origin resource policy. This is recorded as informational in the verification report.

## Superseded historical builder handoff

The material below is preserved only as the builder's pre-verification record. It is superseded by the independent **FAIL** decision and defects above.

## What shipped

- A publishable Rust single-binary CLI, `awb`, with `init`, `check`, `run`, and `inspect` commands; helpful `--help`; documented exit codes; and JSON output.
- Linux enforcement using unprivileged Landlock ABI 3+. The child can write only to explicit policy directories and a wrapper-owned temporary directory that is removed after the run. Unsupported environments fail closed unless the user explicitly selects audit-only fallback.
- An open `.awb-policy.json` format. Relative paths resolve beside the policy, every allowed root must be watched, and dangerously broad filesystem-root or whole-home policies are rejected.
- Before/after SHA-256 and metadata snapshots that include hidden, ignored, untracked, symlink, permission, and `.git` changes. Receipts record the actual enforcement mode, policy, command, exit status, and sorted changes.
- A product-specific static documentation site in the luminous glass data-landscape direction, including an interactive local receipt simulator, keyboard states, 390 px responsive treatment, offline service worker, privacy and terms pages, self-hosted Manrope, and no analytics or runtime third parties.
- An original factory-generated hero at `site/public/hero-boundary.webp` (67,018 bytes). Its full prompt, deployment, and optimization details are in the adjacent provenance JSON and `.factory/design.md`.
- MIT license, README usage/security documentation, changelog, reproducible build scripts, and package metadata for v0.1.0.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo package
```

The exact production build command is `npm run build`. Static deployment output is `dist/site` with `dist/site/index.html` at its root. The optimized host binary is copied to `dist/bin/awb`.

Verification completed on 2026-08-27:

- `npm test`: passed 2 Rust unit tests, 4 CLI integration tests, and 6 Playwright browser tests.
- The CLI escape suite attempted 15 distinct writes outside policy (create, remove, move, copy, hard link, symlink, chmod, truncate, touch, install, `dd`, nested directory, FIFO, and in-place edit); all were blocked. A normal edit/test-style flow succeeded, and `.pyc` plus `.git` changes appeared in its receipt.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm run build`: passed; release CLI and static site produced in the documented locations.
- `cargo package`: passed; crate archive verified. The factory can publish later; this worker did not publish.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- Playwright axe integration: 0 serious or critical findings on home, privacy, and terms pages; keyboard demo and offline reload passed; the 390 px page had 0 px horizontal overflow.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 …`: HTTP 200, 647 ms load in the verifier, no console/page errors, `lang=en`, one `h1`, main present, no missing image alt, no unlabeled buttons.
- Lighthouse 13 mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.2 s, CLS 0.055, total blocking time 0 ms, total transfer 104 KiB.
- Production assets: initial JS 3.46 KB, CSS 18.37 KB, all font subsets 72.42 KB, hero WebP 67.02 KB—within the required 200/50/120/300 KB budgets.

## Known gaps and honest limits

- Enforced mode currently requires Linux with Landlock ABI 3+ (roughly Linux 6.2+). macOS, Windows, and older Linux kernels can use explicit audit-only mode, which observes watched paths but cannot block writes.
- Landlock returns permission errors to the child but does not provide attempted-path telemetry to this unprivileged wrapper. Receipts describe persistent before/after changes, not a log of every denied syscall.
- The boundary does not isolate network access, other processes, devices, already-open file descriptors, kernel vulnerabilities, or hostile same-user processes. The README recommends a VM/container for stronger threat models.
- Snapshot cost scales with watched-tree size because every regular file is hashed. v1 deliberately has no ignore mechanism that could recreate the review blind spot.
- Release binaries and registry publication remain factory-owned next steps; no credentials or deployment infrastructure were touched.

## Suggested next steps

1. Add OS-native enforced backends for macOS Seatbelt and Windows AppContainer without weakening the fail-closed default.
2. Add signed, centrally distributed team policies only after the local open-policy workflow has adoption evidence.
3. Publish reproducible release binaries and checksums from CI, then replace source-install-first copy with release install commands.
