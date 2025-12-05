use std::error::Error as StdError;
use std::fmt;

/// Custom error type for setup operations
#[derive(Debug)]
pub enum SetupError {
    CommandFailed { command: String, exit_code: Option<i32> },
    Io(std::io::Error),
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupError::CommandFailed { command, exit_code } => {
                write!(f, "Command failed: '{}' (exit code: {:?})", command, exit_code)
            }
            SetupError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl StdError for SetupError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            SetupError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SetupError {
    fn from(err: std::io::Error) -> Self {
        SetupError::Io(err)
    }
}

pub type SetupResult<T> = Result<T, SetupError>;
