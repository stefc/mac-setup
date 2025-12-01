// Re-export commonly used items from the symlinks module root so callers (like main)
// can import them from `crate::symlinks` as before.
use std::process::Command;
use std::fmt;
use crate::common::replace_home_with_tilde;

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

/// Configuration for a symlink setup task
#[derive(Clone)]
pub struct SymlinkConfig {
    pub source: String,
    pub destination: String,
    pub installer_name: String,
    pub success_message: String,
}

/// Trait for creating symlinks
pub trait SymlinkCreator {
    fn create(&self, config: &SymlinkConfig) -> SetupResult<()>;
}

/// Default implementation of SymlinkCreator using shell commands
pub struct ShellSymlinkCreator;

impl SymlinkCreator for ShellSymlinkCreator {
    fn create(&self, config: &SymlinkConfig) -> SetupResult<()> {
        let command = format!(
            "mkdir -p $(dirname {}) && ln -fsv {} {}",
            config.destination, config.source, config.destination
        );

        println!("Executing: sh -c \"{}\"", command);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| SetupError::IoError(e.to_string()))?;

        // Print stdout
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let formatted = replace_home_with_tilde(stdout.to_string());
            print!("{}", formatted);
        }

        // Print stderr
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let formatted = replace_home_with_tilde(stderr.to_string());
            eprint!("{}", formatted);
        }

        if output.status.success() {
            println!("{}", config.success_message);
            Ok(())
        } else {
            Err(SetupError::CommandFailed {
                command,
                exit_code: output.status.code(),
            })
        }
    }
}



