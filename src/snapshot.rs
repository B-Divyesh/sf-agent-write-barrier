use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntryState {
    pub kind: EntryKind,
    pub size: u64,
    pub modified_ns: u128,
    pub mode: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlink_target: Option<String>,
}

pub type Snapshot = BTreeMap<PathBuf, EntryState>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Change {
    pub path: PathBuf,
    pub change: ChangeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<EntryState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<EntryState>,
}

pub fn capture(roots: &[PathBuf]) -> Result<Snapshot, String> {
    let mut snapshot = Snapshot::new();
    for root in roots {
        for item in WalkDir::new(root).follow_links(false).sort_by_file_name() {
            let item = item.map_err(|error| format!("snapshot walk failed: {error}"))?;
            let path = item.path().to_path_buf();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
            let file_type = metadata.file_type();
            let kind = if file_type.is_file() {
                EntryKind::File
            } else if file_type.is_dir() {
                EntryKind::Directory
            } else if file_type.is_symlink() {
                EntryKind::Symlink
            } else {
                EntryKind::Other
            };
            let sha256 = if file_type.is_file() {
                Some(
                    hash_file(&path)
                        .map_err(|error| format!("could not hash {}: {error}", path.display()))?,
                )
            } else {
                None
            };
            let symlink_target = if file_type.is_symlink() {
                Some(
                    fs::read_link(&path)
                        .map_err(|error| {
                            format!("could not read link {}: {error}", path.display())
                        })?
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                None
            };
            #[cfg(unix)]
            let (modified_ns, mode) = (
                (metadata.mtime() as i128 * 1_000_000_000_i128 + metadata.mtime_nsec() as i128)
                    .max(0) as u128,
                metadata.permissions().mode(),
            );
            #[cfg(not(unix))]
            let (modified_ns, mode) = (
                metadata
                    .modified()
                    .ok()
                    .and_then(|value| value.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|value| value.as_nanos())
                    .unwrap_or_default(),
                if metadata.permissions().readonly() {
                    0o444
                } else {
                    0o666
                },
            );
            snapshot.insert(
                path,
                EntryState {
                    kind,
                    size: metadata.len(),
                    modified_ns,
                    mode,
                    sha256,
                    symlink_target,
                },
            );
        }
    }
    Ok(snapshot)
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    Ok(hex::encode(digest.finalize()))
}

pub fn diff(before: &Snapshot, after: &Snapshot) -> Vec<Change> {
    let mut paths = before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .filter_map(|path| match (before.get(&path), after.get(&path)) {
            (None, Some(state)) => Some(Change {
                path,
                change: ChangeKind::Created,
                before: None,
                after: Some(state.clone()),
            }),
            (Some(state), None) => Some(Change {
                path,
                change: ChangeKind::Deleted,
                before: Some(state.clone()),
                after: None,
            }),
            (Some(old), Some(new)) if old != new => Some(Change {
                path,
                change: ChangeKind::Modified,
                before: Some(old.clone()),
                after: Some(new.clone()),
            }),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn finds_hidden_content_and_metadata_changes() {
        let temp = tempfile::tempdir().unwrap();
        let hidden = temp.path().join(".ignored.pyc");
        fs::File::create(&hidden)
            .unwrap()
            .write_all(b"before")
            .unwrap();
        let before = capture(&[temp.path().to_path_buf()]).unwrap();
        fs::write(&hidden, b"after").unwrap();
        let created = temp.path().join(".git");
        fs::create_dir(&created).unwrap();
        fs::write(created.join("config"), b"hook").unwrap();
        let after = capture(&[temp.path().to_path_buf()]).unwrap();
        let changes = diff(&before, &after);
        assert!(changes.iter().any(|change| change.path == hidden));
        assert!(
            changes
                .iter()
                .any(|change| change.path.ends_with(".git/config"))
        );
    }
}
