# Handoff — polish round 1

## Outcome

All 86 findings in `.factory/review-1.md` are resolved. No earlier review or polish files existed. The released static site is <https://agent-write-barrier.sociobot.in/>.

The product remains a Rust `clap` CLI with a Vite static documentation site. The luminous glass boundary identity, palette, typography, and original hero art remain intact.

## What changed

- Added `awb demo` and `awb --demo`. They seed bundled sample files in a unique temporary directory, run the real Landlock/snapshot path, and print the sample and receipt paths.
- Added `examples/sample-project/`, a checked terminal recording, `.factory/demo.md`, and an isolated web demo at `/demo/` and `?demo=1`.
- Added the required demo banner, separate `demo:awb:` storage namespace, Reset demo, Start for real, and a privacy control that clears demo and offline data.
- Rewrote the first screen, README, legal pages, limits, labels, and catalog description in plain words.
- Added `.factory/claims.json` with 13 claims and one unique `@claim:<id>` test for each.
- Added route-specific titles, descriptions, canonicals, Open Graph/Twitter metadata, 1200×630 social art, touch icon, sitemap entry, physical routes, History API focus management, and an HTTP 404 page.
- Unified headers and footers across all routes. External links are labeled, touch targets are at least 44px, and horizontal code regions are keyboard focusable.
- Preserved the existing offline/update and deployment-header protections.

The complete finding-by-finding record is in `.factory/polish-1.md`. Copy counts and terminology are in `.factory/copy-audit.md`.

## Verification

The repair was pushed as `4c95edf`, then cloned fresh from GitHub into `/tmp/awb-polish-clean.lVJ5IN/repo`.

Fresh-clone results:

```text
npm ci             PASS — 23 packages, 0 vulnerabilities
npm test           PASS — 6 Rust tests and 18 Playwright tests
npm run lint       PASS — rustfmt and clippy with warnings denied
npm run build      PASS — dist/site and dist/bin/awb
cargo test --doc   PASS
cargo package      PASS — 26 files, 30.0 KiB compressed
13 claim commands  PASS — each claims.json command run separately
```

The packaged crate was installed into `/tmp/awb-consumer-install.EhEf7a`. The installed `awb 0.1.0` passed `demo`, `check --json`, enforced `run`, and `inspect --json` in a separate consumer project.

Build budgets:

```text
initial JS          4,571 B raw / 1,940 B gzip
CSS                21,964 B raw / 8,080 B gzip
self-hosted fonts  72,420 B total
hero image         67,018 B
```

## Deployment and cold production check

Deployment used the work order configuration:

```sh
/opt/fleet/lib/deploy-static.sh agent-write-barrier dist/site
```

Final Azure deployment id: `8a2bfd4c-57d3-44dd-b999-bccbd7301767`.

Post-deploy checks:

- `/opt/fleet/lib/verify-url.sh` passed on `/` and `/demo/` with no console errors.
- `/`, `/demo`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown route returns 404 with the designed page.
- Root, demo, privacy, terms, 404, and service-worker SHA-256 hashes exactly match `dist/site`.
- A cold 390×844 Chromium profile found one h1, one main, zero overflow, and zero serious/critical Axe issues on every route.
- `?demo=1` resolves to `/demo/`, shows the completed blocked write, and writes only `demo:awb:session`. Reset and Start for real work.
- History navigation restores the expected URL and focuses the new h1 in both directions.
- All observed runtime requests were same-origin; there were no cookies or console/page errors.
- The service worker controlled a fresh profile; `/demo/` reloaded offline with its heading and offline notice.
- Live cache and security headers match `staticwebapp.config.json`.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 959 ms, CLS 0.066, TBT 68 ms.

Evidence is under `.factory/evidence/`, including `live-audit.json`, `lighthouse.json`, live verifier reports, and desktop/mobile screenshots.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo test --doc
cargo package
```

Run any single published claim with its exact command from `.factory/claims.json`.

## Known gaps

None found. Registry publication remains factory-owned and was intentionally not performed from this work order.
