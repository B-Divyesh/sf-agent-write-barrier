# Handoff — adversarial first-read review 1

## Decision

**FAIL.** The full review is in `.factory/review-1.md`. Product code was not changed.

## What was done

- Opened the live site in fresh Chromium contexts at 390 × 844 and 1440 × 900 and recorded the cold, above-fold interpretation.
- Audited all landing/README sentences and relevant UI strings for word count, jargon, headings, terminology, and action labels.
- Checked the hero simulator, direct `/demo`, `awb demo`, and `awb --demo` against the required isolated sample-data flow.
- Looked for `.factory/claims.json` and `@claim:*` tests, inventoried every unlisted claim, and proposed a concrete sandbox assertion for each.
- Exercised browser request interception, cookies, Web Storage, live offline reload, keyboard accessibility, mobile target sizes, metadata, links, headers, routing, 404 behavior, and route shells.
- Read the prior handoff and verification records and rechecked the earlier service-worker, cache-policy, and response-header fixes live and in the clean test suite.

## Verification run

From a clean clone at base `7a851d74dd0f69bf19576e7c7dbb67b2444ef152`:

```text
npm ci          PASS — 23 packages, 0 vulnerabilities
npm test        PASS — 2 Rust unit, 4 Rust integration, 8 Playwright
npm run build   PASS — dist/site and dist/bin/awb produced
```

Additional live checks:

- `/opt/fleet/lib/verify-url.sh`: PASS for basic root semantics and console errors.
- Full mobile Axe sweep: **FAIL** with one serious `scrollable-region-focusable` issue on the install command `<pre>`.
- Link crawl: all implemented internal/external links returned 200.
- Unknown route and `/demo`: **FAIL** — both return the homepage with HTTP 200.
- CLI demo in a fresh temporary directory: **FAIL** — both expected entry points exit 2 as unrecognized.
- Simulator privacy interception: PASS — no post-load request, cookie, localStorage, or sessionStorage write.
- Live offline reload: PASS for root and Privacy.

## Known gaps and next steps

The review records 86 findings. The release blockers are unclear/overbroad first-screen copy, no real sample demo, no claims registry or tagged claim tests, broken `/demo`/404 routing, and a serious mobile keyboard accessibility issue. Metadata, shared route shell, focus management, touch targets, information order, and copy also need repair.

Implement the fixes in finding order, then rerun the entire checklist from fresh browser contexts and a clean clone. Do not treat the passing generic suite as claim verification; every retained claim must have one mapped, passing `@claim` test before another review can pass.
