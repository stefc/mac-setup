// Re-export commonly used items from the symlinks module root so callers (like main)
// can import them from `crate::symlinks` as before.
mod creator;
pub mod setup;

pub use creator::{ShellSymlinkCreator, SymlinkCreator};

use std::fmt;

/// Configuration for a symlink setup task
#[derive(Clone)]
pub struct SymlinkConfig {
    pub source: String,
    pub destination: String,
    pub installer_name: &'static str,
    pub success_message: &'static str,
}

/// Custom error type for symlink operations
#[derive(Debug)]
pub enum SetupError {
    CommandFailed { command: String, exit_code: Option<i32> },
    IoError(String),
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupError::CommandFailed { command, exit_code } => {
                write!(f, "Command failed: '{}' (exit code: {:?})", command, exit_code)
            }
            SetupError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

pub type SetupResult<T> = Result<T, SetupError>;



