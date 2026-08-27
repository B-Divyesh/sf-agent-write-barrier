use serde_json::Value;
use std::fs;
use std::process::Command;

fn awb() -> Command {
    Command::new(env!("CARGO_BIN_EXE_awb"))
}

fn write_policy(root: &std::path::Path) {
    fs::write(
        root.join(".awb-policy.json"),
        r#"{
  "version": 1,
  "allow_write": ["worktree"],
  "watch": ["worktree"]
}
"#,
    )
    .unwrap();
}

#[test]
fn documented_init_and_check_flow_works() {
    let temp = tempfile::tempdir().unwrap();
    let init = awb().arg("init").current_dir(temp.path()).output().unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    let check = awb()
        .args(["check", "--json"])
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
    let report: Value = serde_json::from_slice(&check.stdout).unwrap();
    assert_eq!(report["valid"], true);
}

#[test]
#[cfg(target_os = "linux")]
fn allows_normal_workflow_and_receipts_hidden_files() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join("worktree")).unwrap();
    write_policy(temp.path());
    let receipt = temp.path().join("receipt.json");
    let output = awb()
        .args(["run", "--policy", "../.awb-policy.json", "--receipt"])
        .arg(&receipt)
        .args(["--", "sh", "-c", "mkdir -p .git target && printf code > src.rs && printf object > target/cache.pyc && printf ref > .git/HEAD"])
        .current_dir(temp.path().join("worktree"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body: Value = serde_json::from_slice(&fs::read(receipt).unwrap()).unwrap();
    let paths = body["changes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|change| change["path"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(paths.iter().any(|path| path.ends_with("target/cache.pyc")));
    assert!(paths.iter().any(|path| path.ends_with(".git/HEAD")));
    assert_eq!(body["enforcement"], "enforced");
}

#[test]
#[cfg(target_os = "linux")]
fn blocks_fifteen_escape_write_shapes() {
    let temp = tempfile::tempdir().unwrap();
    let worktree = temp.path().join("worktree");
    let outside = temp.path().join("outside");
    fs::create_dir(&worktree).unwrap();
    fs::create_dir(&outside).unwrap();
    fs::write(worktree.join("source"), b"source").unwrap();
    fs::write(outside.join("victim"), b"do not change").unwrap();
    write_policy(temp.path());
    let receipt = temp.path().join("escape-receipt.json");
    let script = r#"
printf x > ../outside/new-redirection 2>/dev/null || true
mkdir ../outside/new-directory 2>/dev/null || true
rm ../outside/victim 2>/dev/null || true
mv source ../outside/moved 2>/dev/null || true
cp source ../outside/copied 2>/dev/null || true
ln source ../outside/hard-link 2>/dev/null || true
ln -s ../worktree/source ../outside/sym-link 2>/dev/null || true
chmod 777 ../outside/victim 2>/dev/null || true
truncate -s 0 ../outside/victim 2>/dev/null || true
touch ../outside/victim 2>/dev/null || true
install source ../outside/installed 2>/dev/null || true
dd if=source of=../outside/dd-copy status=none 2>/dev/null || true
mkdir -p ../outside/deep/nested 2>/dev/null || true
mkfifo ../outside/fifo 2>/dev/null || true
sed -i s/do/not/ ../outside/victim 2>/dev/null || true
printf allowed > allowed.txt
"#;
    let output = awb()
        .args(["run", "--policy", "../.awb-policy.json", "--receipt"])
        .arg(&receipt)
        .args(["--", "sh", "-c", script])
        .current_dir(&worktree)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fs::read(outside.join("victim")).unwrap(), b"do not change");
    assert_eq!(fs::read_dir(&outside).unwrap().count(), 1);
    assert_eq!(fs::read(worktree.join("allowed.txt")).unwrap(), b"allowed");
}

#[test]
fn invalid_policy_fails_with_usage_code() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(
        temp.path().join(".awb-policy.json"),
        r#"{"version":99,"allow_write":["."],"watch":["."]}"#,
    )
    .unwrap();
    let status = awb()
        .arg("check")
        .current_dir(temp.path())
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(64));
}
