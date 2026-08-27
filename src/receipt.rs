use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::RECEIPT_VERSION;
use crate::snapshot::Change;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EnforcementMode {
    Enforced,
    AuditOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub schema_version: u32,
    pub session_id: String,
    pub started_unix_ms: u128,
    pub finished_unix_ms: u128,
    pub enforcement: EnforcementMode,
    pub landlock_abi: Option<i32>,
    pub policy: PathBuf,
    pub allowed_write: Vec<PathBuf>,
    pub watched: Vec<PathBuf>,
    pub command: Vec<String>,
    pub command_exit: i32,
    pub changes: Vec<Change>,
}

impl Receipt {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_id: String,
        started_unix_ms: u128,
        finished_unix_ms: u128,
        enforcement: EnforcementMode,
        landlock_abi: Option<i32>,
        policy: PathBuf,
        allowed_write: Vec<PathBuf>,
        watched: Vec<PathBuf>,
        command: Vec<String>,
        command_exit: i32,
        changes: Vec<Change>,
    ) -> Self {
        Self {
            schema_version: RECEIPT_VERSION,
            session_id,
            started_unix_ms,
            finished_unix_ms,
            enforcement,
            landlock_abi,
            policy,
            allowed_write,
            watched,
            command,
            command_exit,
            changes,
        }
    }
}
