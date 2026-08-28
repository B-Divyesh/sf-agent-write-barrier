# Adversarial first-read review 1 — Agent Write Barrier

**Verdict: FAIL**  
**Reviewed:** 2026-08-28 UTC  
**Live URL:** <https://agent-write-barrier.sociobot.in/>  
**Repository base:** `7a851d74dd0f69bf19576e7c7dbb67b2444ef152`

There are blocking first-read, demo, claims, routing, and accessibility findings. PASS is unavailable while any one finding remains.

## Cold first screen: 390 px and desktop

Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900 before scrolling.

| Question | First-read answer |
| --- | --- |
| What does it do? | It appears to limit where a local coding agent can write and produce a record for review. This is inferred from the lede; “See every write. Bound every agent.” does not state the filesystem job and overstates the documented limits. |
| For whom? | Probably developers running local coding agents. The first screen never says “developers”; it expects the visitor to understand “worktree,” “persistent files,” and “review surface.” |
| What should I click first? | The visual primary action says “Install the barrier.” The safer first action should be the required sample-data demo, but “Try the receipt demo” is secondary and does not open a real demo. |

The same ambiguity exists at both sizes. The mobile first screen does fit without horizontal overflow and exposes both actions, but visibility does not make the job or audience plain.

## Blocking findings

### F-1-1 — The first screen does not plainly answer what this is for or who should use it

- **Exact copy:** “See every write. Bound every agent.” / “Let a coding agent edit and test your real worktree—without letting persistent files slip beyond the surface you’ll review.”
- **Location:** landing hero, mobile and desktop.
- **Why this blocks:** “Bound” is ambiguous as an imperative, “every” conflicts with the stated watched-tree and platform limits, and “worktree,” “persistent,” and “surface” require prior context. The audience is only inferable.
- **Concrete fix:** use a job headline such as **“Block agent writes outside your project”** and a ≤22-word line such as **“For developers running local coding agents who need every lasting project change listed for review.”** Make **“Try it with sample data”** the primary action and state what opens.

### F-1-2 — There is no one-click, isolated product demo

- **Exact location:** hero link “Try the receipt demo” → `/#demo`; direct `/demo`; CLI commands `awb demo` and `awb --demo`.
- **Observed:** the hero link scrolls to an empty simulator saying “Choose a scenario, then run it.” `/demo` returns the ordinary homepage. There is no “Demo — sample data, nothing is saved” banner, Reset demo, Start for real, demo namespace, `examples/`, real terminal recording, or bundled CLI sample. Both CLI demo commands exit 2 as unrecognized.
- **Why this blocks:** one click does not show the product already working on realistic sample input. The JavaScript simulation does not execute the binary and cannot verify the real receipt workflow.
- **Concrete fix:** ship realistic files in `examples/`; add `awb demo` that copies them to a new temporary directory, runs the real binary, prints the receipt and output path, and does not touch the caller’s project. Add a self-hosted recording of that exact command. Make `/demo` or `?demo=1` enter the same documented sample flow with the required banner, Reset, and Start for real controls. Add `.factory/demo.md` and demo isolation tests.

### F-1-3 — The required claims registry and tagged claim tests do not exist

- **Exact location:** `.factory/claims.json` is absent; `rg '@claim:'` returns no matches.
- **Why this blocks:** there were zero listed tests to run. Passing generic tests cannot establish which published sentence is verified in the required clean sandbox.
- **Concrete fix:** create `.factory/claims.json`; give each retained claim exactly one `@claim:<id>` test using the shipped demo entry point; remove claims that cannot be observed. The unlisted claim findings below define the initial registry backlog.

### Unlisted claim findings

Every row is a separate unlisted-claim finding. The fix is to narrow false or untestable wording, then add the named sandbox assertion and registry entry.

| ID | Exact quote and location | Concrete fix / required observable test |
| --- | --- | --- |
| F-1-4 | “The guide stays available; external install links may not.” — offline bar | Register `offline-guide`; load the demo, go offline, and reload every cached route. |
| F-1-5 | “See every write.” — hero | This conflicts with the transient-write limit. Rewrite to “List lasting changes in watched folders,” then assert created, changed, deleted, ignored, and metadata paths. |
| F-1-6 | “Bound every agent.” — hero | Rewrite to “Block outside writes on supported Linux,” then run the real demo under Landlock. |
| F-1-7 | “Let a coding agent … without letting persistent files slip beyond the surface you’ll review.” — hero | Narrow to supported Linux and policy paths; assert all documented escape shapes plus receipt coverage. |
| F-1-8 | “15/15 escape shapes blocked” — hero | Register the quantitative 15-case test and assert all 15 are denied from a clean demo directory. |
| F-1-9 | “0 network calls” — hero | Register a CLI network-isolation/interception test; clarify that the static site itself makes same-origin asset requests. |
| F-1-10 | “1 local binary” — hero | Register a packaged-artifact test that installs and runs one executable with no daemon. |
| F-1-11 | “AWB makes the filesystem—not Git—the unit of review.” — landing | Register receipt coverage for tracked, untracked, ignored, and `.git` paths. |
| F-1-12 | “Landlock limits where the child can persist changes; content snapshots tell you exactly what changed inside.” — landing | Test denial outside policy and receipt differences inside it; replace “exactly” because transient changes are explicitly not guaranteed. |
| F-1-13 | “Nothing in your home or credentials is added implicitly.” — landing | Assert generated/default policies omit home and credential paths. |
| F-1-14 | “Its terminal stays familiar; the write boundary is inherited.” — landing | Replace the subjective first clause; test child-process inheritance with nested writes. |
| F-1-15 | “Read one stable JSON receipt covering content, permissions, links, ignored files, and Git metadata.” — landing | Define “stable”; assert schema and each listed change type. |
| F-1-16 | “This local simulation shows how an enforced run is reported.” — landing demo | Use output captured from the real CLI demo or test byte/field parity with the current binary. |
| F-1-17 | “It executes no shell commands and sends no data.” — landing demo | Register a browser interception test plus an assertion that no process/backend endpoint is invoked. |
| F-1-18 | “Build from the open-source repository today.” — landing | Register the repository license/build result or state the exact MIT source link without promotional wording. |
| F-1-19 | “No daemon, account, telemetry, or global configuration.” — landing | Test process/network/config effects in an isolated home and temp directory. |
| F-1-20 | “Enforced: Linux 6.2+ / Landlock ABI 3+” — landing | Test supported ABI behavior and fail-closed behavior below it; clarify that kernel version alone does not prove ABI availability. |
| F-1-21 | “Block persistent filesystem writes outside policy on supported Linux, then hash watched trees before and after.” — landing | Register an end-to-end packaged CLI test for both denial and before/after hashes. |
| F-1-22 | “Isolate networks, processes, devices, kernel bugs, or hostile same-user processes.” — landing, under “It does not” | Keep as a limitation but register a help/copy consistency assertion so no other route claims these protections. |
| F-1-23 | “Unsupported platforms fail closed.” — landing | Test an enforcement-unavailable environment and assert the child never starts. |
| F-1-24 | “Explicit audit-only mode reports evidence but never pretends to enforce.” — landing | Test mode labeling, allowed outside write, and receipt mode. |
| F-1-25 | “Open source.” — footer | Register a license/source presence test. |
| F-1-26 | “Local only.” — footer | Register a packaged CLI network test or narrow this to “The CLI runs locally.” |
| F-1-27 | “Agent Write Barrier … runs a local coding agent inside an explicit filesystem write policy, then emits a reviewable receipt of every … path …” — README | Replace “every” with the watched-tree limit and register an end-to-end receipt test. |
| F-1-28 | “It is not a malware scanner, a code reviewer, or a portable container runtime.” — README | Register a cross-copy limitations check so UI/help never imply these functions. |
| F-1-29 | “Prebuilt binaries will be attached to GitHub releases.” — README | There are currently no releases. Remove the future promise or publish and test a downloadable release asset. |
| F-1-30 | “Linux 6.2+ with Landlock ABI 3 or newer is the enforced platform in v0.1.” — README | Register platform/ABI detection against the demo binary. |
| F-1-31 | “On unsupported kernels and non-Linux platforms, `awb` fails closed unless `--allow-unsafe-fallback` is explicitly passed; that mode observes writes but cannot block them.” — README | Register fail-closed and explicit fallback tests, including child-start and outside-write observations. |
| F-1-32 | “Paths are resolved relative to the policy file.” — README | Test invocation from a different working directory. |
| F-1-33 | “`allow_write` is the complete persistent write surface.” — README | Test multiple allowed roots and denial immediately outside each root. |
| F-1-34 | “`watch` is snapshotted before and after the command …” — README | Test before/after receipt coverage for every configured watched root. |
| F-1-35 | “Home and credential directories are never added implicitly; policies that allow the filesystem root or an entire home directory are rejected.” — README | Test defaults and rejection for `/`, the isolated home, and equivalent normalized paths. |
| F-1-36 | “A private temporary directory is created for the child, exposed through `TMPDIR`, and removed after the run.” — README | Test privacy, `TMPDIR`, and cleanup on success, non-zero exit, and interruption. |
| F-1-37 | “The command receives the working directory normally.” — README | Test the child’s `pwd` from a clean temp project. |
| F-1-38 | “On Linux, Landlock blocks persistent writes outside `allow_write` at the kernel boundary.” — README | Register the packaged 15-shape denial test and assert enforced mode/ABI. |
| F-1-39 | “After exit, `awb` prints a compact summary to stderr and writes a JSON receipt.” — README | Assert both streams and receipt creation. |
| F-1-40 | “The wrapped command's stdout/stderr remain untouched, and `awb` returns the wrapped command's exit code.” — README | Assert exact stdout, stderr, and representative exit codes. |
| F-1-41 | “Machine-readable `--json` output is written to stderr for `run` … and stdout for `check`/`inspect`.” — README | Assert the stream contract for all three commands. |
| F-1-42 | “No command prompts in CI.” — README | Run every command with closed stdin/`CI=1` and a timeout. |
| F-1-43 | “On supported Linux kernels, `awb` uses unprivileged Landlock rules inherited by the child.” — README | Assert no elevated privilege and nested-child denial. |
| F-1-44 | “It restricts filesystem write operations by path hierarchy without restricting reads.” — README | Assert an outside read succeeds while the corresponding write fails. |
| F-1-45 | “The wrapper intentionally does not mount `$HOME`, add credential exceptions, interpret shell syntax, or claim to isolate networks, processes, devices, already-open file descriptors, kernel bugs, or hostile same-user processes.” — README | Split the sentence and add config/help consistency plus direct tests for the observable clauses. |
| F-1-46 | “Audit fallback is evidence, not enforcement.” — README | Assert explicit fallback labeling and a permitted outside write. |
| F-1-47 | “It detects before/after state changes only in `watch` and cannot prevent, attribute, or guarantee observation of transient writes.” — README | Assert watched versus unwatched behavior; keep the non-guarantee visible beside any receipt claim. |
| F-1-48 | “The receipt records which mode actually ran.” — README | Assert `enforced` and fallback mode values. |
| F-1-49 | “`npm test` covers … a 15-case write-escape suite … and browser accessibility/interaction smoke tests.” — README | Register a test-manifest assertion or replace this unstable implementation claim with a link to CI. |
| F-1-50 | “The static documentation site is deployed from `dist/site`.” — README | Test the build output and deployment configuration path. |
| F-1-51 | “`awb` is local-only, has no telemetry, and makes no network requests.” — README | Register whole-process network interception for the packaged demo. |
| F-1-52 | “Receipts contain local paths and file hashes …” — README | Assert those fields in demo output and keep the sharing warning. |
| F-1-53 | “The documentation site uses no analytics, cookies, or third-party runtime resources.” — README | Register browser request/cookie/storage interception on every route. |
| F-1-54 | “Agent Write Barrier has no telemetry, account, analytics, cookies, or network calls.” — `/privacy/` | Split CLI from website scope and register process plus browser privacy tests. |
| F-1-55 | “The CLI reads only the policy and filesystem roots you select.” — `/privacy/` | This is too broad for executable/library loading. Narrow it to AWB’s own explicit file traversal and test access in an isolated filesystem. |
| F-1-56 | “Write receipts stay on your device.” — `/privacy/` | Register no-upload/no-network behavior for the packaged demo. |
| F-1-57 | “They contain local paths, file metadata, hashes, and the wrapped command.” — `/privacy/` | Assert each field in a real sample receipt. |
| F-1-58 | “This site loads no third-party scripts, fonts, or trackers.” — `/privacy/` | Register request interception across `/`, `/demo`, `/privacy/`, `/terms/`, and 404. |
| F-1-59 | “Its service worker caches public site files on your device for offline reading.” — `/privacy/` | Register offline reload for every claimed route and cache-reset behavior. |
| F-1-60 | “Agent Write Barrier is free software provided under the MIT License.” — `/terms/` | Assert `LICENSE` content and package metadata. |
| F-1-61 | “You may use, copy, modify, and distribute it under that license.” — `/terms/` | Keep linked to the canonical license and assert that the link resolves. |
| F-1-62 | “It reduces a specific filesystem-write risk on supported systems; it does not make untrusted code safe and does not isolate networks, processes, devices, or kernel vulnerabilities.” — `/terms/` | Register the observable supported-system denial and cross-copy limitations check. |
| F-1-63 | “Audit-only mode observes selected paths but cannot block writes.” — `/terms/` | Register a fallback receipt plus a successful outside write. |

### F-1-64 — Broken routing: unknown routes and `/demo` silently render the homepage

- **Exact evidence:** `/definitely-missing-review-route` returned HTTP 200 with homepage title and h1. `/demo` did the same.
- **Why this blocks:** there is no designed 404, and a catalog/demo deep link cannot identify its state. This is broken routing, not a cosmetic omission.
- **Concrete fix:** add real `/demo` and styled 404 routes, return 404 for unknown paths, give each route its own title/h1, and list `/demo` in `sitemap.xml` and the service-worker shell.

### F-1-65 — The mobile install command is a serious keyboard accessibility failure

- **Exact location:** landing install section, first `<pre>` containing `cargo install --git …` at 390 px.
- **Evidence:** Axe reports `scrollable-region-focusable` (serious): “Element should have focusable content / Element should be focusable.”
- **Why this blocks:** keyboard users cannot reach and scroll the horizontally scrollable command.
- **Concrete fix:** make the scroll region focusable with an accessible label and visible focus style, or wrap the command without horizontal scrolling. Add a 390 px Axe test that fails on all serious/critical issues.

## Minor findings

### F-1-66 — The homepage title is a slogan, not a plain description

- **Exact:** `Agent Write Barrier — See every write. Bound every agent.`
- **Why:** the required pattern is “Product — what it does”; the suffix repeats the ambiguous/overbroad hero.
- **Fix:** `Agent Write Barrier — Block writes outside a project` (52 characters).

### F-1-67 — Required social and route metadata is incomplete

- **Exact locations:** `/` lacks Open Graph, Twitter card, and apple-touch icon metadata. `/privacy/` and `/terms/` also lack meta descriptions and canonical links. No route has a 1200 × 630 product OG image.
- **Fix:** add route-specific description/canonical/OG/Twitter metadata, an original 1200 × 630 image derived from the boundary art, and a 180 px apple-touch icon.

### F-1-68 — Header/footer structure is inconsistent across routes

- **Exact evidence:** landing header has How it works, Install, Source; legal headers have only Install and Source. Legal pages have no footer. The landing footer says “Built by Sociobot,” has no product one-liner or build/version, and the header has no Privacy link.
- **Fix:** use one consistent header and footer on every route, including 404/demo; include Privacy, Terms, “Built by Param Factory,” a product one-liner, and version/build id.

### F-1-69 — Hash navigation does not manage focus or announce route/state changes

- **Exact evidence:** clicking “Try the receipt demo” changes the URL to `/#demo` but leaves focus on `<body>`; Back returns near the top with focus still on `<body>`. There is no route-change announcer.
- **Fix:** use a real `/demo` route with `pushState`; on navigation/back, restore scroll as appropriate, focus the new h1, and announce it through a polite live region.

### F-1-70 — Several mobile touch targets are below 44 px

- **Exact locations at 390 px:** header Install is 40 × 44; “Read the security model” is 181 × 19; footer Privacy 44 × 21, Terms 38 × 21, GitHub 42 × 21; the privacy-page repository link is 128 × 22.
- **Fix:** provide at least a 44 × 44 clickable box through padding while preserving inline-link semantics in prose where appropriate.

### F-1-71 — The three hero facts do not cover privacy, offline use, and price

- **Exact:** “15/15 escape shapes blocked” / “0 network calls” / “1 local binary.”
- **Fix:** after registering claims, use three plain facts such as “Runs locally,” “Free under MIT,” and “Guide works offline.” Put the 15-case proof lower on the page.

### F-1-72 — Landing sections do not follow the required information order

- **Exact order:** hero → blind-spot comparison → How it works → simulator.
- **Fix:** put the real sample product/demo immediately after the hero, then How it works, limits/privacy, and footer. The current visual identity itself is distinct and on-thesis; this finding concerns sequence, not art direction.

### F-1-73 — Some external links do not identify themselves as external

- **Exact locations:** footer “Sociobot” and “GitHub,” legal-page repository and MIT License links. Other external links use ↗, so the treatment is inconsistent.
- **Fix:** apply a consistent visible and accessible external-link label/icon.

## Copy findings

The full count appears in the appendices. Counts treat hyphenated compounds and code/path tokens as one word; an em dash separates words.

| ID | Flagged exact copy | Why it fails | Proposed rewrite |
| --- | --- | --- | --- |
| F-1-74 | README: “Agent Write Barrier (`awb`) runs … `.git` internals.” — 40 words | Over the 22-word hard cap and carries several ideas. | “Run a local coding agent under an explicit filesystem write policy. AWB records lasting changes in watched folders. Receipts include ignored, untracked, and `.git` paths.” |
| F-1-75 | README: “It is for developers … every persistent artifact.” — 24 words | Over the hard cap. | “For developers who let coding agents edit real worktrees but need more coverage than `git diff`.” |
| F-1-76 | README: “The wrapper intentionally does not mount `$HOME` … hostile same-user processes.” — 29 words | Over the hard cap and mixes unrelated limits. | “AWB does not mount `$HOME`, add credential exceptions, or interpret shell syntax. It does not isolate networks, processes, devices, open file descriptors, kernel bugs, or hostile same-user processes.” |
| F-1-77 | “Local-first write enforcement” | “Local-first” and “enforcement” are jargon before the job is explained. | “Blocks outside project writes on Linux” |
| F-1-78 | “The blind spot” | The heading makes no sense in a heading list. | “Changes Git does not show” |
| F-1-79 | “Kernel block. Full-tree receipt.” | Compressed jargon hides both actions. | “Block outside writes. Record inside changes.” |
| F-1-80 | “Try a write at the edge.” | “Edge” is a visual metaphor and the section is only a simulation. | “See a simulated outside write” |
| F-1-81 | “A boundary in three commands.” | The heading omits the action and product. | “Install and run AWB” |
| F-1-82 | “Honest by design” | Self-certifying marketing adjective; the content below is simply limits. | “Limits” |
| F-1-83 | “write policy,” “write surface,” “review surface,” “boundary,” “allowed worktree,” and “watched trees” | Related concepts change names without a terminology table; a cold visitor cannot tell policy scope from receipt scope. | Use **allowed paths** for `allow_write`, **watched paths** for `watch`, **write boundary** for enforcement, and **receipt** for output everywhere. Add this table to the README. |
| F-1-84 | “Install the barrier” | The button uses the metaphor instead of naming the installed result. | “Install AWB” |
| F-1-85 | “Live receipt” | Nothing is live; the UI is scripted and does not run AWB. | “Simulated receipt” until the real demo exists. |
| F-1-86 | “Run simulation” | It names an implementation action rather than the result. | “Show blocked write” for the chosen scenario. |

No banned plain-words terms such as “seamless,” “robust,” or “unlock” were found. No static sentence on the landing page exceeds 22 words. The three README overages above are the only hard-cap failures.

## Demo and sandbox evidence

| Check | Result |
| --- | --- |
| One-click sample-data entry | **FAIL** — no “Try it with sample data”; `/#demo` opens an empty simulator. |
| First post-click screen already shows realistic product use | **FAIL** — it asks the visitor to choose and run a scenario. |
| Required banner, Reset, Start for real | **FAIL** — all absent. |
| Direct `/demo` state | **FAIL** — homepage fallback. |
| CLI demo in a fresh temp directory | **FAIL** — `awb demo` and `awb --demo` both exit 2; temp directory remains empty. |
| Shipped realistic sample | **FAIL** — no `examples/` or sample artifact. |
| Browser simulator privacy | PASS as far as the simulator goes — no post-load requests, cookies, localStorage, or sessionStorage during a run. This is not a substitute for demo isolation because there is no real demo state. |
| Offline behavior | PASS — fresh live context was service-worker controlled; offline reload retained `/` and `/privacy/`. |
| Initial browser requests | PASS — all observed requests were same-origin static files. |

## Claims and clean-clone evidence

There was no `.factory/claims.json`, so the number of listed claim commands was **0** and the number of tested registered claims was **0**. This is an untested-claim failure, not a vacuous pass.

For baseline evidence, a clean local clone at `/tmp/awb-review.9MVStN/repo` ran:

- `npm ci`: PASS, 23 packages, 0 vulnerabilities.
- `npm test`: PASS — 2 Rust unit tests, 4 Rust CLI integration tests, and 8 Playwright tests.
- `npm run build`: PASS — produced `dist/site` and `dist/bin/awb`.

The current generic tests exercise several claimed behaviors, but none is tagged or mapped to published copy. The browser suite also runs its Axe check at the default desktop viewport, which is why it misses F-1-65.

## History check

There are no earlier `.factory/review-*.md` or `.factory/polish-*.md` files. I read the existing handoff and verification records and rechecked their earlier defects:

| Earlier item | Live and code confirmation |
| --- | --- |
| Stale service worker after content-only release | **Fixed.** Clean-clone PWA update regression passed. Live `sw.js` uses content-fingerprinted cache `awb-site-04f8637cd8c88e774175` and network-first navigation. |
| Immutable asset caching absent in production | **Fixed.** Live hashed JS returns `public, max-age=31536000, immutable`; `sw.js` returns `no-cache`. |
| Missing response hardening noted in verification 2 | **Fixed.** Live CSP, frame denial, Permissions-Policy, CORP, `nosniff`, and referrer policy are present. |
| Linux Landlock-only enforcement limit | **Still present and disclosed.** This is an intentional product limit, but hero wording still overgeneralizes it (F-1-6/F-1-7). |

No earlier finding is being reopened under an old review id because no earlier review ids exist.

## Structure and quality checks that passed

- Root, Privacy, Terms, favicon, robots, sitemap, GitHub, security-model anchor, and Sociobot links returned 200.
- Root has `lang=en`, one h1, one main, image alt text, skip link, visible product-specific art, no console errors, no horizontal overflow at 390 px, and no generic SaaS-template appearance.
- Heading order is valid on the three implemented pages.
- The root canonical URL, SVG favicon, meta description, theme color, reduced-motion behavior, same-origin CSP, and original-art provenance are present.
- The luminous glass filesystem identity is distinct and matches `.factory/design.md`.

## Appendix A — landing-page sentence and UI-string audit

| # | Copy | Words |
| -: | --- | -: |
| 1 | You’re offline. | 2 |
| 2 | The guide stays available; external install links may not. | 9 |
| 3 | How it works | 3 |
| 4 | Install | 1 |
| 5 | Source | 1 |
| 6 | Local-first write enforcement | 3 |
| 7 | See every write. | 3 |
| 8 | Bound every agent. | 3 |
| 9 | Let a coding agent edit and test your real worktree—without letting persistent files slip beyond the surface you’ll review. | 20 |
| 10 | Install the barrier | 3 |
| 11 | Try the receipt demo | 4 |
| 12 | 15/15 escape shapes blocked | 4 |
| 13 | 0 network calls | 3 |
| 14 | 1 local binary | 3 |
| 15 | Allowed worktree | 2 |
| 16 | Write blocked | 2 |
| 17 | One bright review surface. | 4 |
| 18 | Everything outside stays dark. | 4 |
| 19 | The blind spot | 3 |
| 20 | A clean diff is not a clean filesystem. | 8 |
| 21 | Agents can leave executable caches, environments, hooks, and metadata that ordinary code review never shows. | 15 |
| 22 | AWB makes the filesystem—not Git—the unit of review. | 10 |
| 23 | The boundary | 2 |
| 24 | Kernel block. | 2 |
| 25 | Full-tree receipt. | 2 |
| 26 | Two small mechanisms do one clear job. | 7 |
| 27 | Landlock limits where the child can persist changes; content snapshots tell you exactly what changed inside. | 16 |
| 28 | Declare | 1 |
| 29 | Keep the open JSON policy beside your code. | 8 |
| 30 | Nothing in your home or credentials is added implicitly. | 9 |
| 31 | Run | 1 |
| 32 | Put any local coding agent after `--`. | 6 |
| 33 | Its terminal stays familiar; the write boundary is inherited. | 9 |
| 34 | Review | 1 |
| 35 | Read one stable JSON receipt covering content, permissions, links, ignored files, and Git metadata. | 14 |
| 36 | Live receipt | 2 |
| 37 | Try a write at the edge. | 6 |
| 38 | This local simulation shows how an enforced run is reported. | 10 |
| 39 | It executes no shell commands and sends no data. | 9 |
| 40 | Choose a write scenario | 4 |
| 41 | Outside worktree — Write to `~/.ssh/agent.conf` | 5 |
| 42 | Ignored cache — Create `target/cache.pyc` | 4 |
| 43 | Git metadata — Modify `.git/hooks/pre-commit` | 4 |
| 44 | Old kernel — Enforcement is unavailable | 5 |
| 45 | Run simulation | 2 |
| 46 | Choose a scenario, then run it. | 6 |
| 47 | The receipt will appear here. | 5 |
| 48 | Start local | 2 |
| 49 | A boundary in three commands. | 5 |
| 50 | Build from the open-source repository today. | 6 |
| 51 | No daemon, account, telemetry, or global configuration. | 7 |
| 52 | Enforced: Linux 6.2+ / Landlock ABI 3+ | 6 |
| 53 | Copy install command | 3 |
| 54 | Read the security model | 4 |
| 55 | Honest by design | 3 |
| 56 | A write boundary, not a magic shield. | 7 |
| 57 | It does | 2 |
| 58 | Block persistent filesystem writes outside policy on supported Linux, then hash watched trees before and after. | 16 |
| 59 | It does not | 3 |
| 60 | Isolate networks, processes, devices, kernel bugs, or hostile same-user processes. | 10 |
| 61 | Pair with a VM when needed. | 6 |
| 62 | Fallback is labeled | 3 |
| 63 | Unsupported platforms fail closed. | 4 |
| 64 | Explicit audit-only mode reports evidence but never pretends to enforce. | 10 |
| 65 | Open source. | 2 |
| 66 | Local only. | 2 |
| 67 | Built by Sociobot. | 3 |
| 68 | Checking boundary | 2 |
| 69 | Checking policy and snapshot… | 4 |
| 70 | Checking the selected write against policy. | 6 |
| 71 | BLOCKED · operation not permitted | 4 |
| 72 | ALLOWED · included in receipt | 4 |
| 73 | Ignored file reported. | 3 |
| 74 | Git metadata reported. | 3 |
| 75 | REFUSED · enforcement unavailable | 3 |
| 76 | Landlock ABI 3+ required. | 4 |
| 77 | The command was not started. | 5 |
| 78 | Use a supported kernel or explicit audit mode. | 8 |
| 79 | Copied | 1 |
| 80 | Select command to copy | 4 |

## Appendix B — README sentence and heading audit

| # | Copy | Words |
| -: | --- | -: |
| 1 | Agent Write Barrier | 3 |
| 2 | Agent Write Barrier (`awb`) runs a local coding agent inside an explicit filesystem write policy, then emits a reviewable receipt of every created, changed, deleted, or metadata-modified path in the review surface—including ignored files, untracked files, and `.git` internals. | **40** |
| 3 | It is for developers who want their agent to edit and test a real worktree without trusting `git diff` to reveal every persistent artifact. | **24** |
| 4 | It is not a malware scanner, a code reviewer, or a portable container runtime. | 14 |
| 5 | Install | 1 |
| 6 | Prebuilt binaries will be attached to GitHub releases. | 8 |
| 7 | To build from source: | 4 |
| 8 | Linux 6.2+ with Landlock ABI 3 or newer is the enforced platform in v0.1. | 14 |
| 9 | On unsupported kernels and non-Linux platforms, `awb` fails closed unless `--allow-unsafe-fallback` is explicitly passed; that mode observes writes but cannot block them. | 22 |
| 10 | Usage | 1 |
| 11 | From the repository you want an agent to edit: | 9 |
| 12 | `awb init` creates `.awb-policy.json`: | 4 |
| 13 | Paths are resolved relative to the policy file. | 8 |
| 14 | `allow_write` is the complete persistent write surface. | 7 |
| 15 | `watch` is snapshotted before and after the command; it should contain every allowed path and may include adjacent paths for audit coverage. | 22 |
| 16 | Home and credential directories are never added implicitly; policies that allow the filesystem root or an entire home directory are rejected. | 21 |
| 17 | A private temporary directory is created for the child, exposed through `TMPDIR`, and removed after the run. | 17 |
| 18 | Run any non-interactive or interactive coding agent after `--`: | 8 |
| 19 | The command receives the working directory normally. | 7 |
| 20 | On Linux, Landlock blocks persistent writes outside `allow_write` at the kernel boundary. | 12 |
| 21 | After exit, `awb` prints a compact summary to stderr and writes a JSON receipt. | 14 |
| 22 | The wrapped command's stdout/stderr remain untouched, and `awb` returns the wrapped command's exit code. | 14 |
| 23 | Inspect an existing receipt: | 4 |
| 24 | Machine-readable `--json` output is written to stderr for `run` (so the child owns stdout) and stdout for `check`/`inspect`. | 18 |
| 25 | No command prompts in CI. | 5 |
| 26 | Exit codes | 2 |
| 27 | The command ran; its exit status is preserved. | 8 |
| 28 | Invalid arguments or policy. | 4 |
| 29 | Snapshot, receipt, or process failure. | 5 |
| 30 | Enforcement unavailable and unsafe fallback not authorized. | 7 |
| 31 | Security model and limits | 4 |
| 32 | On supported Linux kernels, `awb` uses unprivileged Landlock rules inherited by the child. | 13 |
| 33 | It restricts filesystem write operations by path hierarchy without restricting reads. | 11 |
| 34 | The wrapper intentionally does not mount `$HOME`, add credential exceptions, interpret shell syntax, or claim to isolate networks, processes, devices, already-open file descriptors, kernel bugs, or hostile same-user processes. | **29** |
| 35 | Use a VM/container as well when the agent itself is untrusted code. | 12 |
| 36 | Audit fallback is evidence, not enforcement. | 6 |
| 37 | It detects before/after state changes only in `watch` and cannot prevent, attribute, or guarantee observation of transient writes. | 18 |
| 38 | The receipt records which mode actually ran. | 7 |
| 39 | Development | 1 |
| 40 | Requirements: Rust 1.85+, Node.js 22+ for the docs site. | 9 |
| 41 | `npm test` covers the documented CLI flows, a 15-case write-escape suite on Linux, receipt behavior, policy validation, and browser accessibility/interaction smoke tests. | 22 |
| 42 | The static documentation site is deployed from `dist/site`. | 8 |
| 43 | Privacy | 1 |
| 44 | `awb` is local-only, has no telemetry, and makes no network requests. | 11 |
| 45 | Receipts contain local paths and file hashes, so review them before sharing. | 12 |
| 46 | The documentation site uses no analytics, cookies, or third-party runtime resources. | 11 |
| 47 | License | 1 |
| 48 | MIT © 2026 Sociobot (Param Factory). | 5 |
| 49 | See LICENSE. | 2 |

## What would make this perfect

Resolve every finding above, then rerun the entire review from a fresh context and clean clone. The decisive result should be: a cold phone visitor reads one plain job headline, sees the developer audience, clicks **Try it with sample data**, and immediately watches the real packaged CLI block a realistic outside write and produce a resettable, isolated receipt. Every retained sentence must map to one passing claim test; `/demo`, legal routes, Back/focus behavior, metadata, 404, keyboard access, touch targets, and shared shell must all pass. Only a rerun with zero findings and zero untested claims merits PASS.
