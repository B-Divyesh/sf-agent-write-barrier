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
    long_about = "Agent Write Barrier runs a command with writes restricted to policy roots, then records every persistent change in watched paths—including ignored, untracked, and .git files.",
    after_help = "Security model: Linux Landlock ABI 3+ enforces filesystem writes. Other systems fail closed unless --allow-unsafe-fallback is explicit. This does not isolate networks or processes."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
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
    let cli = Cli::parse();
    let result = match cli.command {
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
    let prepared = match PreparedRules::new(&policy.allow_write, &temp) {
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
