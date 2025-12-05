use std::fmt;

/// Custom error type for setup operations
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
