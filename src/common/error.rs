use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Command failed: '{command}' (exit code: {exit_code:?})")]
    CommandFailed { command: String, exit_code: Option<i32> },
    #[error("IO error: {0}")]
    Io(#[from] IoError),
}

pub type SetupResult<T> = Result<T, SetupError>;