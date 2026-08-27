use std::fs::File;
use std::io;
#[cfg(target_os = "linux")]
use std::os::fd::{AsRawFd, FromRawFd};
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

pub const MINIMUM_COMPLETE_ABI: i32 = 3;

const CREATE_RULESET_VERSION: u32 = 1;
const RULE_PATH_BENEATH: i32 = 1;
const ACCESS_FS_WRITE_FILE: u64 = 1 << 1;
const ACCESS_FS_REMOVE_DIR: u64 = 1 << 4;
const ACCESS_FS_REMOVE_FILE: u64 = 1 << 5;
const ACCESS_FS_MAKE_CHAR: u64 = 1 << 6;
const ACCESS_FS_MAKE_DIR: u64 = 1 << 7;
const ACCESS_FS_MAKE_REG: u64 = 1 << 8;
const ACCESS_FS_MAKE_SOCK: u64 = 1 << 9;
const ACCESS_FS_MAKE_FIFO: u64 = 1 << 10;
const ACCESS_FS_MAKE_BLOCK: u64 = 1 << 11;
const ACCESS_FS_MAKE_SYM: u64 = 1 << 12;
const ACCESS_FS_REFER: u64 = 1 << 13;
const ACCESS_FS_TRUNCATE: u64 = 1 << 14;

const BASE_WRITE_ACCESS: u64 = ACCESS_FS_WRITE_FILE
    | ACCESS_FS_REMOVE_DIR
    | ACCESS_FS_REMOVE_FILE
    | ACCESS_FS_MAKE_CHAR
    | ACCESS_FS_MAKE_DIR
    | ACCESS_FS_MAKE_REG
    | ACCESS_FS_MAKE_SOCK
    | ACCESS_FS_MAKE_FIFO
    | ACCESS_FS_MAKE_BLOCK
    | ACCESS_FS_MAKE_SYM;

#[repr(C)]
struct RulesetAttr {
    handled_access_fs: u64,
}

#[repr(C)]
struct PathBeneathAttr {
    allowed_access: u64,
    parent_fd: i32,
    reserved: u32,
}

#[derive(Debug)]
pub struct PreparedRules {
    directories: Vec<File>,
    files: Vec<File>,
    abi: i32,
}

pub fn available_abi() -> Result<i32, String> {
    #[cfg(target_os = "linux")]
    {
        let result = unsafe {
            libc::syscall(
                libc::SYS_landlock_create_ruleset,
                std::ptr::null::<RulesetAttr>(),
                0,
                CREATE_RULESET_VERSION,
            )
        };
        if result < 0 {
            return Err(format!(
                "Landlock is unavailable: {}",
                io::Error::last_os_error()
            ));
        }
        let abi = result as i32;
        if abi < MINIMUM_COMPLETE_ABI {
            return Err(format!(
                "Landlock ABI {abi} is incomplete; ABI {MINIMUM_COMPLETE_ABI}+ is required"
            ));
        }
        Ok(abi)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err("Landlock enforcement is available only on Linux".into())
    }
}

#[cfg(target_os = "linux")]
impl PreparedRules {
    pub fn new(allowed: &[PathBuf], temporary: &Path) -> Result<Self, String> {
        let abi = available_abi()?;
        let mut directories = Vec::with_capacity(allowed.len() + 1);
        for path in allowed
            .iter()
            .chain(std::iter::once(&temporary.to_path_buf()))
        {
            directories.push(open_path(path)?);
        }
        let mut files = Vec::new();
        for path in [Path::new("/dev/null"), Path::new("/dev/tty")] {
            if path.exists() {
                files.push(open_path(path)?);
            }
        }
        Ok(Self {
            directories,
            files,
            abi,
        })
    }

    pub fn abi(&self) -> i32 {
        self.abi
    }

    /// Apply the prepared rules between fork and exec.
    ///
    /// # Safety
    ///
    /// Call only from `CommandExt::pre_exec` in the forked child.
    pub unsafe fn restrict_child(&self) -> io::Result<()> {
        let handled = access_for_abi(self.abi);
        let attr = RulesetAttr {
            handled_access_fs: handled,
        };
        let ruleset_fd = unsafe {
            libc::syscall(
                libc::SYS_landlock_create_ruleset,
                &attr,
                std::mem::size_of::<RulesetAttr>(),
                0,
            )
        };
        if ruleset_fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let ruleset = unsafe { File::from_raw_fd(ruleset_fd as i32) };
        for directory in &self.directories {
            add_rule(ruleset.as_raw_fd(), directory.as_raw_fd(), handled)?;
        }
        let file_access = ACCESS_FS_WRITE_FILE | if self.abi >= 3 { ACCESS_FS_TRUNCATE } else { 0 };
        for file in &self.files {
            add_rule(ruleset.as_raw_fd(), file.as_raw_fd(), file_access)?;
        }
        let no_new_privileges = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
        if no_new_privileges != 0 {
            return Err(io::Error::last_os_error());
        }
        let restricted =
            unsafe { libc::syscall(libc::SYS_landlock_restrict_self, ruleset.as_raw_fd(), 0) };
        if restricted != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
impl PreparedRules {
    pub fn new(_allowed: &[PathBuf], _temporary: &Path) -> Result<Self, String> {
        Err("Landlock enforcement is available only on Linux".into())
    }

    pub fn abi(&self) -> i32 {
        0
    }
}

#[cfg(target_os = "linux")]
fn access_for_abi(abi: i32) -> u64 {
    BASE_WRITE_ACCESS
        | if abi >= 2 { ACCESS_FS_REFER } else { 0 }
        | if abi >= 3 { ACCESS_FS_TRUNCATE } else { 0 }
}

#[cfg(target_os = "linux")]
fn open_path(path: &Path) -> Result<File, String> {
    let bytes = path.as_os_str().as_bytes();
    let value = std::ffi::CString::new(bytes)
        .map_err(|_| format!("path contains a null byte: {}", path.display()))?;
    let fd = unsafe { libc::open(value.as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
    if fd < 0 {
        Err(format!(
            "could not prepare Landlock path {}: {}",
            path.display(),
            io::Error::last_os_error()
        ))
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

#[cfg(target_os = "linux")]
fn add_rule(ruleset_fd: i32, parent_fd: i32, allowed_access: u64) -> io::Result<()> {
    let attr = PathBeneathAttr {
        allowed_access,
        parent_fd,
        reserved: 0,
    };
    let result = unsafe {
        libc::syscall(
            libc::SYS_landlock_add_rule,
            ruleset_fd,
            RULE_PATH_BENEATH,
            &attr,
            0,
        )
    };
    if result != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
