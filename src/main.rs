use agent_write_barrier::landlock::{self, PreparedRules};
use agent_write_barrier::policy::{Policy, ResolvedPolicy, read_json_file, write_json_file};
use agent_write_barrier::receipt::{EnforcementMode, Receipt};
use agent_write_barrier::snapshot;
use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use std::env;
use std::ffi::OsString;
use std::fs;
#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

const EXIT_USAGE: u8 = 64;
const EXIT_SOFTWARE: u8 = 70;
const EXIT_UNAVAILABLE: u8 = 77;

#[derive(Debug, Parser)]
#[command(
    name = "awb",
    version,
    about = "Give a coding agent an explicit filesystem write boundary",
    long_about = "Agent Write Barrier runs a command with writes restricted to allowed paths, then records lasting changes in watched paths—including ignored, untracked, and .git files.",
    after_help = "Security model: Linux Landlock ABI 3+ enforces filesystem writes. Other systems fail closed unless --allow-unsafe-fallback is explicit. This does not isolate networks or processes."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the real barrier on bundled sample data in an isolated directory
    Demo(DemoArgs),
    /// Create a minimal policy in the current directory
    Init(InitArgs),
    /// Validate a policy and report enforcement support
    Check(CheckArgs),
    /// Run a command behind the barrier and write a receipt
    Run(RunArgs),
    /// Read a saved write receipt
    Inspect(InspectArgs),
}

#[derive(Debug, Args)]
struct DemoArgs {}

#[derive(Debug, Args)]
struct InitArgs {
    /// Policy path to create
    #[arg(long, default_value = ".awb-policy.json")]
    policy: PathBuf,
    /// Replace an existing policy
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Args)]
struct CheckArgs {
    /// Policy to validate
    #[arg(long, default_value = ".awb-policy.json")]
    policy: PathBuf,
    /// Print machine-readable JSON
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct RunArgs {
    /// Policy to enforce
    #[arg(long, default_value = ".awb-policy.json")]
    policy: PathBuf,
    /// Receipt destination (default: a unique file under .awb/receipts)
    #[arg(long)]
    receipt: Option<PathBuf>,
    /// Print the complete receipt as JSON to stderr
    #[arg(long)]
    json: bool,
    /// Run in audit-only mode when Landlock enforcement is unavailable
    #[arg(long)]
    allow_unsafe_fallback: bool,
    /// Command and arguments to run (place after --)
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<OsString>,
}

#[derive(Debug, Args)]
struct InspectArgs {
    /// Receipt JSON to inspect
    receipt: PathBuf,
    /// Print the original machine-readable JSON
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Serialize)]
struct CheckReport {
    valid: bool,
    policy: PathBuf,
    allowed_write: Vec<PathBuf>,
    watched: Vec<PathBuf>,
    enforcement_available: bool,
    landlock_abi: Option<i32>,
    note: String,
}

fn main() -> ExitCode {
    // Keep `--demo` as a convenient alias for people arriving from the site.
    let mut values = env::args_os().collect::<Vec<_>>();
    if values.get(1).is_some_and(|value| value == "--demo") {
        values[1] = OsString::from("demo");
    }
    let cli = Cli::parse_from(values);
    let result = match cli.command {
        Commands::Demo(args) => demo(args),
        Commands::Init(args) => init(args),
        Commands::Check(args) => check(args),
        Commands::Run(args) => run(args),
        Commands::Inspect(args) => inspect(args),
    };
    match result {
        Ok(code) => ExitCode::from(code),
        Err((code, message)) => {
            eprintln!("awb: {message}");
            ExitCode::from(code)
        }
    }
}

fn demo(_args: DemoArgs) -> Result<u8, (u8, String)> {
    let started = unix_ms();
    let demo_root = env::temp_dir().join(format!(
        "agent-write-barrier-demo-{started}-{}",
        std::process::id()
    ));
    let project = demo_root.join("sample-project");
    fs::create_dir_all(project.join("src"))
        .map_err(|error| software_error(format!("could not create demo source: {error}")))?;
    fs::create_dir_all(project.join(".git/hooks"))
        .map_err(|error| software_error(format!("could not create demo metadata: {error}")))?;
    fs::write(
        project.join("src/config.rs"),
        "pub const API_TIMEOUT_SECONDS: u8 = 20;\n",
    )
    .map_err(|error| software_error(format!("could not write demo source: {error}")))?;
    fs::write(project.join("obsolete.txt"), "remove after migration\n")
        .map_err(|error| software_error(format!("could not write demo fixture: {error}")))?;
    fs::write(project.join(".git/hooks/pre-commit"), "#!/bin/sh\nexit 0\n")
        .map_err(|error| software_error(format!("could not write demo metadata: {error}")))?;
    fs::write(demo_root.join("blocked-agent.conf"), "developer-owned\n")
        .map_err(|error| software_error(format!("could not write demo boundary file: {error}")))?;
    let policy_path = demo_root.join("demo-policy.json");
    fs::write(
        &policy_path,
        "{\n  \"version\": 1,\n  \"allow_write\": [\"sample-project\"],\n  \"watch\": [\"sample-project\"]\n}\n",
    )
    .map_err(|error| software_error(format!("could not write demo policy: {error}")))?;
    let receipt_path = demo_root.join("write-receipt.json");

    println!("AWB demo — bundled sample in an isolated temporary directory");
    println!("Agent task: update a timeout and remove an obsolete file.");
    let script = r#"
if printf 'agent-key' > ../blocked-agent.conf 2>/dev/null; then
  printf 'UNEXPECTED: outside write succeeded\n'
else
  printf 'BLOCKED: ../blocked-agent.conf stayed outside allowed paths\n'
fi
mkdir -p target
printf 'pub const API_TIMEOUT_SECONDS: u8 = 30;\n' > src/config.rs
printf 'compiled sample\n' > target/config.pyc
rm obsolete.txt
chmod 755 .git/hooks/pre-commit
ln -s src/config.rs current-config
printf 'RECORDED: source, ignored cache, deletion, link, and Git metadata\n'
"#;
    let caller = env::current_dir()
        .map_err(|error| software_error(format!("could not read current directory: {error}")))?;
    env::set_current_dir(&project)
        .map_err(|error| software_error(format!("could not enter demo project: {error}")))?;
    let run_result = run(RunArgs {
        policy: policy_path,
        receipt: Some(receipt_path.clone()),
        json: false,
        allow_unsafe_fallback: false,
        command: vec![
            OsString::from("sh"),
            OsString::from("-c"),
            OsString::from(script),
        ],
    });
    env::set_current_dir(&caller)
        .map_err(|error| software_error(format!("could not restore current directory: {error}")))?;
    let code = run_result?;
    let boundary = fs::read_to_string(demo_root.join("blocked-agent.conf"))
        .map_err(|error| software_error(format!("could not verify demo boundary: {error}")))?;
    if boundary != "developer-owned\n" {
        return Err(software_error(
            "demo boundary verification failed; outside file changed".into(),
        ));
    }
    println!("Demo complete. Nothing was written to your current project.");
    println!("Sample directory: {}", project.display());
    println!("Receipt: {}", receipt_path.display());
    Ok(code)
}

fn init(args: InitArgs) -> Result<u8, (u8, String)> {
    Policy::create(&args.policy, args.force).map_err(usage_error)?;
    println!("Created {}", args.policy.display());
    println!("Next: review it, then run `awb check`.");
    Ok(0)
}

fn check(args: CheckArgs) -> Result<u8, (u8, String)> {
    let (_, policy) = Policy::load(&args.policy).map_err(usage_error)?;
    let support = landlock::available_abi();
    let report = CheckReport {
        valid: true,
        policy: args.policy,
        allowed_write: policy.allow_write,
        watched: policy.watch,
        enforcement_available: support.is_ok(),
        landlock_abi: support.as_ref().ok().copied(),
        note: support
            .err()
            .unwrap_or_else(|| "Landlock enforcement is ready.".into()),
    };
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        println!("Policy valid: {}", report.policy.display());
        println!("Allowed write roots: {}", report.allowed_write.len());
        println!("Watched roots: {}", report.watched.len());
        if report.enforcement_available {
            println!(
                "Enforcement: ready (Landlock ABI {})",
                report.landlock_abi.unwrap()
            );
        } else {
            println!("Enforcement: unavailable — {}", report.note);
        }
    }
    Ok(0)
}

fn run(args: RunArgs) -> Result<u8, (u8, String)> {
    let (_, policy) = Policy::load(&args.policy).map_err(usage_error)?;
    let started = unix_ms();
    let session_id = format!("{started}-{}", std::process::id());
    let receipt_path = args
        .receipt
        .unwrap_or_else(|| PathBuf::from(format!(".awb/receipts/{session_id}.json")));
    if let Some(parent) = receipt_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            software_error(format!("could not create receipt directory: {error}"))
        })?;
    }

    let temp = create_session_temp(&session_id).map_err(software_error)?;
    #[cfg(debug_assertions)]
    let forced_unavailable = env::var_os("AWB_TEST_DISABLE_LANDLOCK").is_some();
    #[cfg(not(debug_assertions))]
    let forced_unavailable = false;
    let prepared_result = if forced_unavailable {
        Err("Landlock disabled by the test harness".into())
    } else {
        PreparedRules::new(&policy.allow_write, &temp)
    };
    let prepared = match prepared_result {
        Ok(rules) => Some(rules),
        Err(error) if args.allow_unsafe_fallback => {
            eprintln!("awb: warning: audit-only fallback; writes are not blocked ({error})");
            None
        }
        Err(error) => {
            let _ = fs::remove_dir_all(&temp);
            return Err((
                EXIT_UNAVAILABLE,
                format!(
                    "{error}. Refusing to run without enforcement; pass --allow-unsafe-fallback to audit only"
                ),
            ));
        }
    };
    let mode = if prepared.is_some() {
        EnforcementMode::Enforced
    } else {
        EnforcementMode::AuditOnly
    };
    let abi = prepared.as_ref().map(PreparedRules::abi);
    let before = snapshot::capture(&policy.watch).map_err(|error| {
        let _ = fs::remove_dir_all(&temp);
        software_error(error)
    })?;

    let status = execute(&args.command, &temp, prepared).map_err(|error| {
        let _ = fs::remove_dir_all(&temp);
        software_error(error)
    })?;
    let after = snapshot::capture(&policy.watch).map_err(|error| {
        let _ = fs::remove_dir_all(&temp);
        software_error(error)
    })?;
    fs::remove_dir_all(&temp).map_err(|error| {
        software_error(format!("could not remove private temp directory: {error}"))
    })?;

    let exit = status_code(status);
    let receipt = Receipt::new(
        session_id,
        started,
        unix_ms(),
        mode,
        abi,
        args.policy,
        policy.allow_write,
        policy.watch,
        args.command
            .iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect(),
        exit,
        snapshot::diff(&before, &after),
    );
    write_json_file(&receipt_path, &receipt).map_err(software_error)?;

    if args.json {
        eprintln!("{}", serde_json::to_string_pretty(&receipt).unwrap());
    } else {
        print_receipt(&receipt, Some(&receipt_path));
    }
    Ok(exit.clamp(0, 255) as u8)
}

fn inspect(args: InspectArgs) -> Result<u8, (u8, String)> {
    let receipt: Receipt = read_json_file(&args.receipt).map_err(usage_error)?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&receipt).unwrap());
    } else {
        print_receipt(&receipt, Some(&args.receipt));
    }
    Ok(0)
}

fn execute(
    values: &[OsString],
    temporary: &Path,
    prepared: Option<PreparedRules>,
) -> Result<std::process::ExitStatus, String> {
    let mut command = Command::new(&values[0]);
    command.args(&values[1..]);
    command
        .env("TMPDIR", temporary)
        .env("TMP", temporary)
        .env("TEMP", temporary);
    #[cfg(target_os = "linux")]
    if let Some(rules) = prepared {
        unsafe {
            command.pre_exec(move || rules.restrict_child());
        }
    }
    #[cfg(not(target_os = "linux"))]
    let _ = prepared;
    command
        .status()
        .map_err(|error| format!("could not start {}: {error}", values[0].to_string_lossy()))
}

fn print_receipt(receipt: &Receipt, path: Option<&Path>) {
    let mode = match receipt.enforcement {
        EnforcementMode::Enforced => "ENFORCED",
        EnforcementMode::AuditOnly => "AUDIT ONLY",
    };
    eprintln!(
        "awb: {mode} · exit {} · {} persistent change{}",
        receipt.command_exit,
        receipt.changes.len(),
        if receipt.changes.len() == 1 { "" } else { "s" }
    );
    for change in &receipt.changes {
        eprintln!("  {:?}\t{}", change.change, change.path.display());
    }
    if let Some(path) = path {
        eprintln!("awb: receipt {}", path.display());
    }
}

fn create_session_temp(session_id: &str) -> Result<PathBuf, String> {
    let path = env::temp_dir().join(format!("agent-write-barrier-{session_id}"));
    fs::create_dir(&path).map_err(|error| {
        format!(
            "could not create private temp directory {}: {error}",
            path.display()
        )
    })?;
    Ok(path)
}

fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn status_code(status: std::process::ExitStatus) -> i32 {
    if let Some(code) = status.code() {
        return code;
    }
    #[cfg(unix)]
    {
        128 + status.signal().unwrap_or(1)
    }
    #[cfg(not(unix))]
    {
        1
    }
}

fn usage_error(message: String) -> (u8, String) {
    (EXIT_USAGE, message)
}

fn software_error(message: String) -> (u8, String) {
    (EXIT_SOFTWARE, message)
}

#[allow(dead_code)]
fn _assert_policy_is_send(_: &ResolvedPolicy) {}
