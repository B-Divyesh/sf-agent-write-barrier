# Adversarial first-read review 2 — Agent Write Barrier

**Verdict: PASS**  
**Reviewed:** 2026-08-28 UTC  
**Live URL:** <https://agent-write-barrier.sociobot.in/>  
**Candidate:** `1284e41499fe3f230cd0dee26f518e2c26122c25`

This is a full repeat review, not a diff review. There are **zero findings**: no blocking, minor, unlisted-claim, or untested-claim items remain.

## Cold first read

Fresh Chromium contexts opened the live root at 390 × 844 and 1440 × 900. No scrolling occurred before answering these questions. The primary action was visible at 390 px (`top 473 px`, `bottom 522 px`) and desktop (`top 556 px`, `bottom 605 px`). Neither context had horizontal overflow or console/page errors.

| Question | Answer from the first screen |
| --- | --- |
| What does it do? | It blocks a coding agent from writing outside the project, then lists lasting project changes for review. |
| For whom? | Developers running local coding agents. |
| What should I click first? | **Try it with sample data**; it says it opens a completed run with a blocked write and receipt. |

The evidence is direct and plain: **“Block agent writes outside your project”**; **“For developers running local coding agents who need lasting project changes listed for review.”**; and **“Try it with sample data”** / **“Opens a completed sample run with a blocked write and receipt.”** The headline is six words, begins with a verb, and the audience line is 13 words.

## Copy audit

Counts use whitespace-separated words. Code blocks, JSON keys, file paths, and table cells that are not prose are excluded. Headings and controls are included because a visitor encounters them as standalone copy. No entry is over 22 words; no banned marketing term, inconsistent product term, contextless heading, or non-result-naming control was found.

### Landing page

| Copy | Words |
| --- | ---: |
| Skip to main content | 4 |
| Demo | 1 |
| Install | 1 |
| Privacy | 1 |
| Source | 1 |
| You’re offline. | 2 |
| Cached guides and the sample recording remain available. | 8 |
| Blocks outside project writes on Linux | 6 |
| Block agent writes outside your project | 6 |
| For developers running local coding agents who need lasting project changes listed for review. | 13 |
| Try it with sample data | 6 |
| Install AWB | 2 |
| Opens a completed sample run with a blocked write and receipt. | 11 |
| The CLI runs locally | 4 |
| MIT licensed | 2 |
| Cached guide works | 3 |
| Allowed paths glow. | 3 |
| The outside write stops. | 4 |
| See the blocked write and receipt | 6 |
| The bundled demo uses a temporary sample project. | 8 |
| It leaves your current project unchanged. | 6 |
| Open the sample run | 4 |
| List lasting changes Git can miss | 6 |
| AWB compares watched paths before and after a run. | 9 |
| The receipt includes ignored files and Git metadata. | 8 |
| Block outside writes. | 3 |
| Record inside changes. | 3 |
| Choose paths | 2 |
| Set allowed paths for writes and watched paths for the receipt. | 11 |
| Run the agent | 3 |
| Put the local agent command after `--`. | 7 |
| Child processes inherit the boundary. | 5 |
| Review the receipt | 3 |
| Inspect created, modified, and deleted paths with content and metadata hashes. | 11 |
| Install and run AWB | 4 |
| Build the MIT-licensed CLI from its source repository. | 8 |
| Copy install command | 3 |
| Read the security model | 4 |
| Know what the boundary covers | 6 |
| Supported Linux | 2 |
| The 15-case test blocks writes outside allowed paths under Landlock. | 10 |
| Not a full sandbox | 4 |
| AWB does not isolate networks, processes, devices, open file descriptors, or kernel flaws. | 13 |
| Visible fallback | 2 |
| AWB fails closed without enforcement. | 5 |
| Explicit audit mode labels receipts and cannot block writes. | 9 |
| The CLI has no telemetry. | 5 |
| The site uses no analytics or third-party runtime files. | 9 |
| Read the privacy details | 4 |
| Block outside writes. Review lasting changes. | 7 |
| Terms | 1 |
| GitHub | 1 |
| Built by Param Factory · v0.1.0 · polish 1 | 7 |

### README

| Copy | Words |
| --- | ---: |
| Agent Write Barrier | 3 |
| Run a local coding agent under an explicit filesystem write policy. | 11 |
| AWB records lasting changes in watched paths. | 7 |
| Receipts include ignored files, untracked files, links, and `.git` metadata. | 9 |
| AWB is not a malware scanner or code reviewer. | 10 |
| It is for developers who need more filesystem coverage than `git diff` provides. | 13 |
| Try the bundled sample | 4 |
| The demo creates a unique temporary project and runs the real barrier. | 12 |
| It blocks a realistic outside write and prints the receipt path. | 11 |
| The sample files live in `examples/sample-project`. | 6 |
| The demo leaves the caller’s project unchanged. | 7 |
| The web recording is at `/demo/`. | 6 |
| Use Reset demo to restore its isolated sample state. | 9 |
| Install | 1 |
| Build from the MIT-licensed source repository. | 6 |
| Landlock ABI 3 or newer provides enforced mode on Linux. | 10 |
| Kernel version alone does not prove that Landlock is available. | 10 |
| AWB fails closed when enforcement is unavailable. | 7 |
| Pass `--allow-unsafe-fallback` explicitly to run in labeled audit-only mode. | 8 |
| Run AWB | 2 |
| From the project an agent should edit: | 8 |
| `awb init` creates `.awb-policy.json`: | 4 |
| Terms used here | 3 |
| Paths resolve from the policy file, even when AWB starts elsewhere. | 11 |
| Every allowed path must sit inside a watched path. | 10 |
| AWB adds no home or credential paths. | 7 |
| It rejects policies allowing the filesystem root or the complete home directory. | 12 |
| AWB creates a private temporary directory for the child. | 9 |
| It sets `TMPDIR`, `TMP`, and `TEMP`, then removes that directory. | 10 |
| Run a coding agent or another command after `--`: | 9 |
| The child receives its normal working directory. | 7 |
| Nested child processes inherit the write boundary. | 7 |
| Reads outside allowed paths still work. | 6 |
| Persistent writes outside those paths fail in enforced mode. | 9 |
| AWB leaves the child’s standard output and error streams intact. | 10 |
| It returns the child’s exit status. | 7 |
| The normal summary goes to standard error. | 8 |
| `run --json` also writes its JSON receipt to standard error. | 10 |
| `check --json` and `inspect --json` write JSON to standard output. | 10 |
| No command prompts when standard input is closed. | 8 |
| Inspect a receipt: | 3 |
| Receipts record content hashes, metadata, link targets, the command, its exit code, and the mode used. | 16 |
| Exit codes | 2 |
| The command ran and its status was preserved. | 8 |
| Arguments or policy were invalid. | 6 |
| A snapshot, receipt, or process operation failed. | 8 |
| Enforcement was unavailable and fallback was not allowed. | 8 |
| Security model and limits | 4 |
| On supported Linux systems, unprivileged Landlock rules restrict writes by path hierarchy. | 12 |
| The rules are inherited by child processes. | 7 |
| AWB does not mount `$HOME`, add credential exceptions, or interpret shell syntax. | 11 |
| AWB does not isolate networks, processes, devices, open file descriptors, kernel flaws, or hostile same-user processes. | 16 |
| Use a VM or container when the command itself is untrusted code. | 13 |
| Audit-only mode is evidence, not enforcement. | 6 |
| It observes watched paths but cannot block writes. | 9 |
| Receipts compare before and after states in watched paths. | 9 |
| They cannot attribute changes or guarantee observation of transient writes. | 10 |
| Develop and verify | 3 |
| Requirements are Rust 1.85 or newer and Node.js 22 or newer. | 12 |
| `npm test` runs type checks, Rust tests, claim tests, browser tests, and accessibility checks. | 13 |
| The build writes the site to `dist/site`. | 8 |
| The release binary is written to `dist/bin/awb`. | 8 |
| Registry publishing is handled outside this repository. | 7 |
| Privacy | 1 |
| AWB is one local CLI. | 5 |
| Its package contains no telemetry or network client. | 8 |
| Receipts remain at the path you choose. | 8 |
| Review them before sharing because local paths can reveal names. | 10 |
| The documentation site uses no analytics, cookies, trackers, or third-party runtime files. | 12 |
| Its service worker caches public routes for offline reading. | 9 |
| See the privacy page for browser storage controls. | 9 |
| License | 1 |
| MIT © 2026 Sociobot (Param Factory). | 6 |
| See LICENSE. | 2 |

The established terminology remains consistent: **allowed paths**, **watched paths**, **write boundary**, **receipt**, and **sample demo**. “Landlock” is the named Linux mechanism in the technical explanation, not unexplained hero jargon. The product uses no AI claim or decorative AI control; the brief does not imply an AI step, and JSON receipts already provide the expected machine-readable export.

## Demo and sandbox

The one-click hero action opened `/demo/` directly. Its first viewport already displayed a completed, realistic `awb demo` result: a denied `../blocked-agent.conf` write plus modified source, ignored cache, deletion, link, Git-hook mode, enforced status, and receipt path.

The persistent banner reads **“Demo — sample data, nothing is saved”**, identifies the `demo:awb:` namespace, and offers **Reset demo** and **Start for real**. In a fresh browser, the only storage key after entry was `demo:awb:session`; Reset restored the original receipt and retained only that key; Start for real navigated to `/#install` and removed it. Requests during the flow stayed same-origin.

The actual CLI path was exercised from a new temporary caller directory using the fresh-clone binary. `awb demo` created a separate `/tmp/agent-write-barrier-demo-*` project, blocked the outside write, created the receipt, and left the caller directory’s sentinel unchanged. This also confirms that the self-hosted page recording represents the bundled sample workflow rather than an invented browser simulation.

The offline claim was exercised by the published `@claim:offline-guide` test: public routes and the sample recording reload after the service worker is controlling the page; the Privacy control clears caches. The privacy claim test intercepts requests across root, demo, legal, and missing routes, confirms same-origin requests, no cookies, and only the demo namespace in local storage.

## Claims and clean-clone verification

`.factory/claims.json` has 13 entries, each with one `@claim:` test. A fresh clone of `main` at the candidate SHA was created at `/tmp/awb-review-2.za1Iiv/repo`. All 13 listed commands passed separately:

`demo-isolation`, `write-boundary-15`, `receipt-coverage`, `policy-safety`, `process-contract`, `enforcement-modes`, `local-cli`, `license-source`, `site-privacy`, `offline-guide`, `routing-metadata`, `mobile-access`, and `limits-consistent`.

The fresh clone also passed:

```text
npm ci
npm test
npm run lint
npm run build
cargo test --doc
cargo package
```

`npm test` passed the TypeScript check, 6 Rust tests, and the Playwright browser/claims suite. `cargo package` produced the verified `agent-write-barrier-0.1.0.crate` (30,761 bytes). Published claims on the landing page and README map to the registry: enforcement/15 cases, receipt coverage, policy safety, process behavior, fallback behavior, local/no-telemetry package scope, license, site privacy, offline guide, routing/accessibility, and limits. No claim-like sentence without a registry entry remains.

## Earlier finding closure

Every earlier review item was checked again on the live site and in the candidate source, rather than accepted from its “fixed” label.

| Earlier IDs | Recheck result |
| --- | --- |
| F-1-1 | Hero now states the job, named audience, sample action, next result, and three plain facts in the initial 390 px viewport. |
| F-1-2 | `/demo/`, `?demo=1`, persistent sandbox banner, Reset, Start for real, bundled files, self-hosted recording, `awb demo`, and `awb --demo` all work. |
| F-1-3 | The 13-entry registry and uniquely tagged tests exist and pass from the clean clone. |
| F-1-4 through F-1-63 | Each former unlisted claim is now narrowed to the observable behavior above and covered by its applicable passed registry test. This includes offline, 15-write enforcement, receipt fields, policy/home safety, stream and child-process contract, audit fallback, local/network scope, MIT source, browser privacy, and consistent limitations. |
| F-1-64 | `/demo/`, `/privacy/`, `/terms/`, and a missing route load distinct routes; an unknown URL returned HTTP 404 and rendered the designed “Return to a known path” page. |
| F-1-65 | The 390 px Axe claim passes; the install command is focusable and scrollable by keyboard. |
| F-1-66 through F-1-73 | Route titles, descriptions, canonical/OG/Twitter/favicon metadata, social art, shared header/footer, focus/history behavior, 44 px controls, three facts, landing order, and labeled external links were confirmed live. |
| F-1-74 through F-1-86 | The full fresh-copy audit above confirms the README/landing rewrites, terminology, result-naming controls, and real completed sample state. |

Navigation verification was live: Demo navigation focused its h1 and announced **“Watch AWB block a sample outside write page loaded”**; Back restored root and focused **“Block agent writes outside your project”**. Every crawlable internal and GitHub link returned 200. Root, demo, privacy, terms, and 404 each have one h1, one main, their own title/description/canonical/OG/Twitter metadata, and the shared Privacy/Terms footer. The dark glass-boundary art is distinct and agrees with `.factory/design.md`; it is not a generic SaaS template.

## What would make this perfect

Nothing is required for acceptance. Keep the demonstrated evidence current when the binary, public copy, service worker, or deployment configuration changes: rerun the 13 claim commands from a fresh clone and repeat the live route, demo-storage, offline, and link checks before release.
