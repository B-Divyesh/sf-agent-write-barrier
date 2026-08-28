# Demo sandbox

## One-click web demo

Open <https://agent-write-barrier.sociobot.in/demo/> or add `?demo=1` to the homepage URL. The route immediately shows a recording of the bundled CLI sample, its blocked outside write, and its receipt.

The banner stays visible throughout demo mode. **Reset demo** removes keys beginning with `demo:awb:` and restores the bundled sample state. **Start for real** removes the same demo namespace and opens the install section. No real project data is read or written by the web demo.

## Real CLI demo

Run:

```sh
awb demo
```

`awb --demo` is an alias. The command creates a unique `agent-write-barrier-demo-*` directory under the operating system temporary directory. It seeds the files from `examples/sample-project/`, runs the real Landlock and snapshot path, and prints both the sample directory and receipt path.

The sample changes `src/config.rs`, creates an ignored cache file and symbolic link, deletes `obsolete.txt`, and changes a Git hook mode. It also tries to overwrite `../blocked-agent.conf`; enforced mode leaves that file unchanged. The caller's current project is never used as an allowed or watched path.
