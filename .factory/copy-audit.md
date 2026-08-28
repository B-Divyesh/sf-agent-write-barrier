# Copy audit — polish 1

Counts use whitespace-separated words. Interface labels are included where they carry a complete idea. No sentence exceeds 22 words, and no banned marketing term appears.

| Landing sentence or label | Words | Result |
| --- | ---: | --- |
| You’re offline. | 2 | Pass |
| Cached guides and the sample recording remain available. | 8 | Pass |
| Blocks outside project writes on Linux | 6 | Pass |
| Block agent writes outside your project | 6 | Pass |
| For developers running local coding agents who need lasting project changes listed for review. | 13 | Pass |
| Try it with sample data | 6 | Pass |
| Opens a completed sample run with a blocked write and receipt. | 11 | Pass |
| The CLI runs locally | 4 | Pass |
| MIT licensed | 2 | Pass |
| Cached guide works | 3 | Pass |
| Allowed paths glow. | 3 | Pass |
| The outside write stops. | 4 | Pass |
| See the blocked write and receipt | 6 | Pass |
| The bundled demo uses a temporary sample project. | 8 | Pass |
| It leaves your current project unchanged. | 6 | Pass |
| List lasting changes Git can miss | 6 | Pass |
| AWB compares watched paths before and after a run. | 9 | Pass |
| The receipt includes ignored files and Git metadata. | 8 | Pass |
| Block outside writes. | 3 | Pass |
| Record inside changes. | 3 | Pass |
| Landlock applies the write boundary. | 5 | Pass |
| Snapshots compare lasting changes only in watched paths. | 8 | Pass |
| Set allowed paths for writes and watched paths for the receipt. | 11 | Pass |
| Put the local agent command after `--`. | 7 | Pass |
| Child processes inherit the boundary. | 5 | Pass |
| Inspect created, modified, and deleted paths with content and metadata hashes. | 11 | Pass |
| Install and run AWB | 4 | Pass |
| Build the MIT-licensed CLI from its source repository. | 8 | Pass |
| The 15-case test blocks writes outside allowed paths under Landlock. | 10 | Pass |
| AWB does not isolate networks, processes, devices, open file descriptors, or kernel flaws. | 13 | Pass |
| AWB fails closed without enforcement. | 5 | Pass |
| Explicit audit mode labels receipts and cannot block writes. | 9 | Pass |
| The CLI has no telemetry. | 5 | Pass |
| The site uses no analytics or third-party runtime files. | 9 | Pass |

## Terminology

| Concept | One term used |
| --- | --- |
| `allow_write` directories | allowed paths |
| `watch` directories | watched paths |
| enforced Landlock scope | write boundary |
| before-and-after JSON output | receipt |
| isolated bundled example | sample demo |
