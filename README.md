# Agent Write Barrier

Agent Write Barrier (`awb`) runs a local coding agent inside an explicit filesystem write policy, then emits a reviewable receipt of every created, changed, deleted, or metadata-modified path in the review surface—including ignored files, untracked files, and `.git` internals.

It is for developers who want their agent to edit and test a real worktree without trusting `git diff` to reveal every persistent artifact. It is not a malware scanner, a code reviewer, or a portable container runtime.

## Install

Prebuilt binaries will be attached to GitHub releases. To build from source:

```sh
cargo install --path .
```

Linux 5.13+ with Landlock is the enforced platform in v0.1. On unsupported kernels and non-Linux platforms, `awb` fails closed unless `--allow-unsafe-fallback` is explicitly passed; that mode observes writes but cannot block them.

## Usage

From the repository you want an agent to edit:

```sh
awb init
awb check
awb run -- claude
```

`awb init` creates `.awb-policy.json`:

```json
{
  "version": 1,
  "allow_write": ["."],
  "watch": ["."]
}
```

Paths are resolved relative to the policy file. `allow_write` is the complete persistent write surface. `watch` is snapshotted before and after the command; it should contain every allowed path and may include adjacent paths for audit coverage. Home and credential directories are never added implicitly. A private temporary directory is created for the child, exposed through `TMPDIR`, and removed after the run.

Run any non-interactive or interactive coding agent after `--`:

```sh
awb run --receipt .awb/last-run.json -- aider --model sonnet
awb run --json -- sh -c 'printf "done\n" > result.txt'
```

The command receives the working directory normally. On Linux, Landlock blocks persistent writes outside `allow_write` at the kernel boundary. After exit, `awb` prints a compact summary to stderr and writes a JSON receipt. The wrapped command's stdout/stderr remain untouched, and `awb` returns the wrapped command's exit code.

Inspect an existing receipt:

```sh
awb inspect .awb/last-run.json
awb inspect --json .awb/last-run.json
```

Machine-readable `--json` output is written to stderr for `run` (so the child owns stdout) and stdout for `check`/`inspect`. No command prompts in CI.

### Exit codes

| Code | Meaning |
| ---: | --- |
| wrapped status | The command ran; its exit status is preserved |
| `64` | Invalid arguments or policy |
| `70` | Snapshot, receipt, or process failure |
| `77` | Enforcement unavailable and unsafe fallback not authorized |

### Security model and limits

On supported Linux kernels, `awb` uses unprivileged Landlock rules inherited by the child. It restricts filesystem write operations by path hierarchy without restricting reads. The wrapper intentionally does not mount `$HOME`, add credential exceptions, interpret shell syntax, or claim to isolate networks, processes, devices, already-open file descriptors, kernel bugs, or hostile same-user processes. Use a VM/container as well when the agent itself is untrusted code.

Audit fallback is evidence, not enforcement. It detects before/after state changes only in `watch` and cannot prevent, attribute, or guarantee observation of transient writes. The receipt records which mode actually ran.

## Development

Requirements: Rust 1.85+, Node.js 22+ for the docs site.

```sh
npm install
npm test
npm run build        # CLI plus site -> dist/bin and dist/site
npm run dev          # docs site
cargo package        # ready-to-publish crate archive; do not publish here
```

`npm test` covers the documented CLI flows, a 15-case write-escape suite on Linux, receipt behavior, policy validation, and browser accessibility/interaction smoke tests. The static documentation site is deployed from `dist/site`.

## Privacy

`awb` is local-only, has no telemetry, and makes no network requests. Receipts contain local paths and file hashes, so review them before sharing. The documentation site uses no analytics, cookies, or third-party runtime resources.

## License

MIT © 2026 Sociobot (Param Factory). See [LICENSE](LICENSE).
