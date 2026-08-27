use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::POLICY_VERSION;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub version: u32,
    pub allow_write: Vec<PathBuf>,
    pub watch: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedPolicy {
    pub source: PathBuf,
    pub allow_write: Vec<PathBuf>,
    pub watch: Vec<PathBuf>,
}

impl Policy {
    pub fn starter() -> Self {
        Self {
            version: POLICY_VERSION,
            allow_write: vec![PathBuf::from(".")],
            watch: vec![PathBuf::from(".")],
        }
    }

    pub fn load(path: &Path) -> Result<(Self, ResolvedPolicy), String> {
        let raw = fs::read_to_string(path)
            .map_err(|error| format!("could not read policy {}: {error}", path.display()))?;
        let policy: Self = serde_json::from_str(&raw)
            .map_err(|error| format!("invalid policy {}: {error}", path.display()))?;
        let resolved = policy.resolve(path)?;
        Ok((policy, resolved))
    }

    pub fn resolve(&self, source: &Path) -> Result<ResolvedPolicy, String> {
        if self.version != POLICY_VERSION {
            return Err(format!(
                "unsupported policy version {}; expected {}",
                self.version, POLICY_VERSION
            ));
        }
        if self.allow_write.is_empty() {
            return Err("allow_write must contain at least one path".into());
        }
        if self.watch.is_empty() {
            return Err("watch must contain at least one path".into());
        }

        let base = source
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .canonicalize()
            .map_err(|error| format!("could not resolve policy directory: {error}"))?;
        let resolve_paths = |paths: &[PathBuf], field: &str| -> Result<Vec<PathBuf>, String> {
            let mut result = Vec::with_capacity(paths.len());
            for value in paths {
                let candidate = if value.is_absolute() {
                    value.clone()
                } else {
                    base.join(value)
                };
                let canonical = candidate.canonicalize().map_err(|error| {
                    format!(
                        "{field} path {} cannot be resolved: {error}",
                        candidate.display()
                    )
                })?;
                if !result.contains(&canonical) {
                    result.push(canonical);
                }
            }
            Ok(result)
        };
        let allow_write = resolve_paths(&self.allow_write, "allow_write")?;
        let watch = resolve_paths(&self.watch, "watch")?;

        let home = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .and_then(|value| PathBuf::from(value).canonicalize().ok());
        for allowed in &allow_write {
            if allowed.parent().is_none() {
                return Err("refusing to allow writes to the filesystem root".into());
            }
            if home.as_ref().is_some_and(|value| value == allowed) {
                return Err("refusing to allow writes to the entire home directory".into());
            }
            if !allowed.is_dir() {
                return Err(format!(
                    "allow_write path {} must be a directory",
                    allowed.display()
                ));
            }
        }

        for allowed in &allow_write {
            if !watch.iter().any(|root| allowed.starts_with(root)) {
                return Err(format!(
                    "allowed path {} is not covered by any watch root",
                    allowed.display()
                ));
            }
        }

        Ok(ResolvedPolicy {
            source: source.to_path_buf(),
            allow_write,
            watch,
        })
    }

    pub fn create(path: &Path, force: bool) -> Result<(), String> {
        if path.exists() && !force {
            return Err(format!(
                "{} already exists; pass --force to replace it",
                path.display()
            ));
        }
        let body = serde_json::to_string_pretty(&Self::starter())
            .map_err(|error| format!("could not serialize starter policy: {error}"))?;
        let mut options = fs::OpenOptions::new();
        options
            .write(true)
            .truncate(force)
            .create(force)
            .create_new(!force);
        let mut file = options
            .open(path)
            .map_err(|error| format!("could not create {}: {error}", path.display()))?;
        file.write_all(format!("{body}\n").as_bytes())
            .map_err(|error| format!("could not write {}: {error}", path.display()))
    }
}

pub fn write_json_file(path: &Path, value: &impl Serialize) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("could not serialize JSON: {error}"))?;
    let mut file = fs::File::create(path)
        .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    file.write_all(&bytes)
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|error| format!("could not write {}: {error}", path.display()))
}

pub fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let file = fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    serde_json::from_reader(io::BufReader::new(file))
        .map_err(|error| format!("invalid JSON in {}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_a_filesystem_wide_write_policy() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("policy.json");
        let policy = Policy {
            version: POLICY_VERSION,
            allow_write: vec![PathBuf::from("/")],
            watch: vec![PathBuf::from("/")],
        };
        let error = policy.resolve(&source).unwrap_err();
        assert!(error.contains("filesystem root"));
    }
}
