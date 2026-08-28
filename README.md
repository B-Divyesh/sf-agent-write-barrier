# Agent Write Barrier

Run a local coding agent under an explicit filesystem write policy. AWB records lasting changes in watched paths.

Receipts include ignored files, untracked files, links, and `.git` metadata. AWB is not a malware scanner or code reviewer.

It is for developers who need more filesystem coverage than `git diff` provides.

## Try the bundled sample

```sh
cargo run -- demo
```

The demo creates a unique temporary project and runs the real barrier. It blocks a realistic outside write and prints the receipt path.

The sample files live in [`examples/sample-project`](examples/sample-project). The demo leaves the caller's project unchanged.

The web recording is at <https://agent-write-barrier.sociobot.in/demo/>. Use **Reset demo** to restore its isolated sample state.

## Install

Build from the MIT-licensed source repository:

```sh
cargo install --git https://github.com/B-Divyesh/sf-agent-write-barrier
```

Landlock ABI 3 or newer provides enforced mode on Linux. Kernel version alone does not prove that Landlock is available.

AWB fails closed when enforcement is unavailable. Pass `--allow-unsafe-fallback` explicitly to run in labeled audit-only mode.

## Run AWB

From the project an agent should edit:

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

### Terms used here

| Term | Meaning |
| --- | --- |
| allowed paths | Directories where the wrapped command may write |
| watched paths | Directories compared before and after the command |
| write boundary | The Landlock path rules applied to the child |
| receipt | The JSON record of lasting changes in watched paths |

Paths resolve from the policy file, even when AWB starts elsewhere. Every allowed path must sit inside a watched path.

AWB adds no home or credential paths. It rejects policies allowing the filesystem root or the complete home directory.

AWB creates a private temporary directory for the child. It sets `TMPDIR`, `TMP`, and `TEMP`, then removes that directory.

Run a coding agent or another command after `--`:

```sh
awb run --receipt .awb/last-run.json -- aider --model sonnet
awb run --json -- sh -c 'printf "done\n" > result.txt'
```

The child receives its normal working directory. Nested child processes inherit the write boundary.

Reads outside allowed paths still work. Persistent writes outside those paths fail in enforced mode.

AWB leaves the child's standard output and error streams intact. It returns the child's exit status.

The normal summary goes to standard error. `run --json` also writes its JSON receipt to standard error.

`check --json` and `inspect --json` write JSON to standard output. No command prompts when standard input is closed.

Inspect a receipt:

```sh
awb inspect .awb/last-run.json
awb inspect --json .awb/last-run.json
```

Receipts record content hashes, metadata, link targets, the command, its exit code, and the mode used.

### Exit codes

| Code | Meaning |
| ---: | --- |
| wrapped status | The command ran and its status was preserved |
| `64` | Arguments or policy were invalid |
| `70` | A snapshot, receipt, or process operation failed |
| `77` | Enforcement was unavailable and fallback was not allowed |

## Security model and limits

On supported Linux systems, unprivileged Landlock rules restrict writes by path hierarchy. The rules are inherited by child processes.

AWB does not mount `$HOME`, add credential exceptions, or interpret shell syntax.

AWB does not isolate networks, processes, devices, open file descriptors, kernel flaws, or hostile same-user processes.

Use a VM or container when the command itself is untrusted code.

Audit-only mode is evidence, not enforcement. It observes watched paths but cannot block writes.

Receipts compare before and after states in watched paths. They cannot attribute changes or guarantee observation of transient writes.

## Develop and verify

Requirements are Rust 1.85 or newer and Node.js 22 or newer.

```sh
npm ci
npm test
npm run lint
npm run build
cargo package
```

`npm test` runs type checks, Rust tests, claim tests, browser tests, and accessibility checks. The build writes the site to `dist/site`.

The release binary is written to `dist/bin/awb`. Registry publishing is handled outside this repository.

## Privacy

AWB is one local CLI. Its package contains no telemetry or network client.

Receipts remain at the path you choose. Review them before sharing because local paths can reveal names.

The documentation site uses no analytics, cookies, trackers, or third-party runtime files. Its service worker caches public routes for offline reading.

See the [privacy page](https://agent-write-barrier.sociobot.in/privacy/) for browser storage controls.

## License

MIT © 2026 Sociobot (Param Factory). See [LICENSE](LICENSE).
