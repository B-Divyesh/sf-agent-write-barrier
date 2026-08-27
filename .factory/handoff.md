# Handoff — Agent Write Barrier v0.1.0

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
