# Polish round 1 — finding closure

**Repaired candidate:** `4c95edf`  
**Live URL:** <https://agent-write-barrier.sociobot.in/>  
**Live audit:** `.factory/evidence/live-audit.json`  
**Screenshots:** `.factory/evidence/home-mobile.png`, `.factory/evidence/demo-mobile.png`, `.factory/evidence/demo-desktop.png`, `.factory/evidence/404-desktop.png`, `.factory/evidence/live-root/`, `.factory/evidence/live-demo/`

All findings from `.factory/review-1.md` are closed. There were no earlier review or polish files. Earlier verification defects were also rechecked.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the slogan with “Block agent writes outside your project,” named developers, and made the sample action primary. | `first screen states the job…`; `home-mobile.png`; live `/` |
| F-1-2 | Added bundled sample files, real `awb demo` and `awb --demo`, isolated temp execution, `/demo/`, `?demo=1`, banner, reset, and exit. | `@claim:demo-isolation`; `demo-mobile.png`; live `/demo/` |
| F-1-3 | Added 13 claims to `.factory/claims.json`, each with one unique tagged test. | all `@claim:*` tests; clean-clone 13/13; live claim routes |
| F-1-4 | Reworded the offline notice and cached every public route. | `@claim:offline-guide`; `live-audit.json`; live offline `/demo/` |
| F-1-5 | Removed “See every write” and stated lasting changes in watched paths. | `@claim:receipt-coverage`; `home-mobile.png`; live `/` |
| F-1-6 | Replaced the universal claim with supported-Linux wording. | `@claim:write-boundary-15`; `home-mobile.png`; live `/` |
| F-1-7 | Limited the hero promise to allowed and watched paths. | `@claim:write-boundary-15`, `@claim:receipt-coverage`; live `/` |
| F-1-8 | Moved the 15-case result below the fold and registered its quantitative test. | `@claim:write-boundary-15`; live `/` limits |
| F-1-9 | Removed “0 network calls” from the hero and separated CLI/site privacy wording. | `@claim:local-cli`, `@claim:site-privacy`; live audit has no third party requests |
| F-1-10 | Replaced “1 local binary” with “The CLI runs locally” and tested the one-bin package. | `@claim:local-cli`; live `/` |
| F-1-11 | Replaced the slogan with concrete watched-path receipt coverage. | `@claim:receipt-coverage`; live `/` receipt comparison |
| F-1-12 | Removed “exactly” and distinguished enforcement from lasting-change snapshots. | `@claim:write-boundary-15`, `@claim:receipt-coverage`; live `/` |
| F-1-13 | Documented no implicit home/credential paths and tested generated policies. | `@claim:policy-safety`; README |
| F-1-14 | Removed the subjective terminal claim and tested nested-child inheritance. | `@claim:process-contract`; live `/` step 2 |
| F-1-15 | Defined the receipt fields and asserted each field/change class. | `@claim:receipt-coverage`; live `/demo/` |
| F-1-16 | Replaced the simulator with a self-hosted recording checked against real `awb demo` output. | `@claim:demo-isolation`, `@claim:receipt-coverage`; `demo-desktop.png` |
| F-1-17 | Removed the simulator claim; the static demo privacy scope is now tested. | `@claim:site-privacy`; live `/demo/` |
| F-1-18 | Replaced promotional wording with the exact source and MIT license statement. | `@claim:license-source`; live `/` install |
| F-1-19 | Removed the compound marketing claim; retained testable local CLI and telemetry facts. | `@claim:local-cli`; live `/privacy/` |
| F-1-20 | Described support by Landlock ABI instead of kernel version alone. | `@claim:enforcement-modes`; live `/` install |
| F-1-21 | Split the action into allowed-path enforcement and watched-path receipts. | `@claim:write-boundary-15`, `@claim:receipt-coverage`; live `/` |
| F-1-22 | Kept the limitation and made it consistent across site, README, terms, and help. | `@claim:limits-consistent`; live `/terms/` |
| F-1-23 | Tested that unavailable enforcement exits 77 before the child starts. | `@claim:enforcement-modes`; live `/` limits |
| F-1-24 | Tested explicit fallback, successful outside write, warning, and `audit-only` receipt. | `@claim:enforcement-modes`; live `/terms/` |
| F-1-25 | Linked open source status to the MIT license and repository evidence. | `@claim:license-source`; live footer |
| F-1-26 | Narrowed “Local only” to “The CLI runs locally.” | `@claim:local-cli`; live hero fact |
| F-1-27 | Rewrote README introduction to say lasting changes in watched paths. | `@claim:receipt-coverage`; README |
| F-1-28 | Kept the non-goals and synchronized them across public copy. | `@claim:limits-consistent`; README and live `/terms/` |
| F-1-29 | Removed the untrue future release promise. | copy audit; README install section |
| F-1-30 | Replaced kernel shorthand with Landlock ABI detection wording. | `@claim:enforcement-modes`; README and live `/` |
| F-1-31 | Added fail-closed and explicit audit fallback tests. | `@claim:enforcement-modes`; README |
| F-1-32 | Tested policy-relative paths from a different current directory. | `@claim:policy-safety`; README |
| F-1-33 | Tested two allowed roots and a denied sibling root. | `@claim:policy-safety`; README terminology table |
| F-1-34 | Tested before/after changes within watched roots. | `@claim:receipt-coverage`; README |
| F-1-35 | Tested starter omission plus normalized root and full-home rejection. | `@claim:policy-safety`; README |
| F-1-36 | Tested private `TMPDIR` cleanup after non-zero and signaled children; demo covers success. | `@claim:process-contract`, `@claim:demo-isolation`; README |
| F-1-37 | Tested the child’s working directory. | `@claim:process-contract`; README |
| F-1-38 | Kept the supported-Linux statement and ran all 15 write shapes under ABI 3. | `@claim:write-boundary-15`; clean clone |
| F-1-39 | Tested summary output and JSON receipt creation. | `@claim:process-contract`; packaged consumer run |
| F-1-40 | Tested child stdout, stderr, and exit 23. | `@claim:process-contract`; README |
| F-1-41 | Tested `run`, `check`, and `inspect` stream/JSON behavior. | `@claim:process-contract`, `@claim:policy-safety`; README |
| F-1-42 | Claim subprocesses use closed standard input and timeouts; no prompt occurs. | all CLI claim tests; clean clone |
| F-1-43 | Tested nested-child denial/inheritance under unprivileged Landlock. | `@claim:process-contract`, `@claim:write-boundary-15` |
| F-1-44 | Tested an outside read succeeds while outside writes remain denied. | `@claim:process-contract`; README |
| F-1-45 | Split the long sentence and synchronized explicit non-goals. | `@claim:local-cli`, `@claim:limits-consistent`; `.factory/copy-audit.md` |
| F-1-46 | Tested and labeled audit fallback as non-enforcing. | `@claim:enforcement-modes`; README |
| F-1-47 | Kept the transient-write limitation beside the receipt description. | `@claim:receipt-coverage`, `@claim:limits-consistent`; README |
| F-1-48 | Asserted `enforced` and `audit-only` receipt modes. | `@claim:receipt-coverage`, `@claim:enforcement-modes` |
| F-1-49 | Updated the development statement and made `npm test` run all named suites. | clean-clone `npm test`: 18 browser + 6 Rust tests |
| F-1-50 | Kept `dist/site` as the static build and verified its deploy config. | `npm run build`; live artifact hashes match |
| F-1-51 | Narrowed the claim to package contents and tested dependencies/source. | `@claim:local-cli`; README |
| F-1-52 | Asserted local paths, metadata, hashes, command, and exit code. | `@claim:receipt-coverage`; README and `/privacy/` |
| F-1-53 | Tested all routes for cookies, storage, and third-party runtime requests. | `@claim:site-privacy`; `live-audit.json` |
| F-1-54 | Split CLI and website privacy scopes. | `@claim:local-cli`, `@claim:site-privacy`; live `/privacy/` |
| F-1-55 | Narrowed the statement to AWB traversal of configured watched paths and disclosed child reads. | `@claim:policy-safety`, `@claim:process-contract`; live `/privacy/` |
| F-1-56 | Stated receipts use the chosen local path and tested no upload/client. | `@claim:local-cli`, `@claim:receipt-coverage`; live `/privacy/` |
| F-1-57 | Asserted every stated receipt field in the real demo receipt. | `@claim:receipt-coverage`; live `/privacy/` |
| F-1-58 | Tested root, demo, legal, and 404 runtime requests. | `@claim:site-privacy`; `live-audit.json` |
| F-1-59 | Tested offline reload of every cached route and the clear-data control. | `@claim:offline-guide`; live offline `/demo/` |
| F-1-60 | Added the canonical MIT title and tested package metadata. | `@claim:license-source`; live `/terms/` |
| F-1-61 | Kept the rights statement beside an accessible external canonical license link. | `@claim:license-source`; live `/terms/` link check |
| F-1-62 | Narrowed risk language and synchronized non-goals. | `@claim:write-boundary-15`, `@claim:limits-consistent`; live `/terms/` |
| F-1-63 | Tested that audit mode permits the outside write and labels the receipt. | `@claim:enforcement-modes`; live `/terms/` |
| F-1-64 | Added physical demo/legal pages, history routing, and a styled 404 with HTTP 404 configuration. | `@claim:routing-metadata`; `404-desktop.png`; live missing route = 404 |
| F-1-65 | Made scrolling command/recording regions focusable and added mobile Axe coverage. | `@claim:mobile-access`; `home-mobile.png`; live 390 px Axe |
| F-1-66 | Set the root title to “Agent Write Barrier — Block writes outside a project.” | `@claim:routing-metadata`; live `/` title |
| F-1-67 | Added canonical, description, Open Graph, Twitter, 1200×630 art, and 180px touch icon metadata. | `@claim:routing-metadata`; live metadata and asset 200s |
| F-1-68 | Applied the same header/footer, legal links, factory credit, one-liner, and version to every route. | `@claim:mobile-access`; all live routes |
| F-1-69 | Added `pushState`, `popstate`, h1 focus, scroll reset, and polite route announcements. | `@claim:routing-metadata`; `live-audit.json` |
| F-1-70 | Expanded header, footer, prose, demo, and legal targets to at least 44px. | `@claim:mobile-access`; `home-mobile.png` |
| F-1-71 | Replaced hero metrics with local, MIT, and offline facts backed by claims. | `@claim:local-cli`, `@claim:license-source`, `@claim:offline-guide`; live `/` |
| F-1-72 | Moved the working sample preview directly after the first screen, before explanation. | `home-mobile.png`; live `/` section order |
| F-1-73 | Added visible arrows and accessible “external” names to off-site links. | `@claim:mobile-access`; live link crawl |
| F-1-74 | Split the README opening into three short sentences. | `.factory/copy-audit.md`; README |
| F-1-75 | Rewrote the audience sentence to 13 words. | `.factory/copy-audit.md`; README |
| F-1-76 | Split the long wrapper-limit sentence into two plain statements. | `.factory/copy-audit.md`; README |
| F-1-77 | Replaced “Local-first write enforcement” with supported-Linux wording. | `.factory/copy-audit.md`; `home-mobile.png` |
| F-1-78 | Replaced “The blind spot” with “List lasting changes Git can miss.” | `.factory/copy-audit.md`; live `/` |
| F-1-79 | Replaced compressed jargon with “Block outside writes. Record inside changes.” | `.factory/copy-audit.md`; live `/` |
| F-1-80 | Removed the simulated-edge heading; the page now opens a real sample result. | `@claim:demo-isolation`; live `/demo/` |
| F-1-81 | Replaced the metaphor with “Install and run AWB.” | `.factory/copy-audit.md`; live `/` |
| F-1-82 | Replaced self-certifying language with “Know what the boundary covers.” | `.factory/copy-audit.md`; live `/` |
| F-1-83 | Standardized allowed paths, watched paths, write boundary, and receipt; added a README terminology table. | `@claim:limits-consistent`; README |
| F-1-84 | Replaced “Install the barrier” with “Install AWB.” | `first screen states the job…`; `home-mobile.png` |
| F-1-85 | Replaced “Live receipt” with a labeled, self-hosted real CLI recording. | `@claim:demo-isolation`; `demo-desktop.png` |
| F-1-86 | Removed “Run simulation”; the one-click demo opens already completed. | `query demo entry…`; live `/?demo=1` |

## Earlier verification items

- Content-fingerprinted service-worker updates still pass `a content-only release refreshes an existing controlled client`.
- Live hashed assets return `public, max-age=31536000, immutable`; `sw.js` remains `no-cache`.
- Live CSP, frame denial, permissions policy, CORP, `nosniff`, and strict referrer policy remain present.
- The Linux Landlock-only enforcement limit remains visible and is now tested by ABI rather than claimed from kernel version alone.

## Final evidence

- Fresh clone: `npm ci`, `npm test`, `npm run lint`, `npm run build`, `cargo test --doc`, and `cargo package` passed.
- Every command in `.factory/claims.json` passed separately from the fresh clone.
- The packaged crate installed into a separate prefix; `awb demo`, `check`, `run`, and `inspect` passed there.
- Live hashes for root, demo, privacy, terms, 404, and `sw.js` match `dist/site` exactly.
- Live Lighthouse: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 959 ms, CLS 0.066, TBT 68 ms.
- Initial JS is 4,571 B raw / 1,940 B gzip. CSS is 21,964 B raw / 8,080 B gzip. Fonts total 72,420 B.
