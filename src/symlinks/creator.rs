use crate::symlinks::SymlinkConfig;
use crate::symlinks::SetupResult;
use std::process::Command;

/// Trait for creating symlinks
pub trait SymlinkCreator {
    fn create(&self, config: &SymlinkConfig) -> SetupResult<()>;
}

/// Default implementation of SymlinkCreator using shell commands
pub struct ShellSymlinkCreator;

impl SymlinkCreator for ShellSymlinkCreator {
    fn create(&self, config: &SymlinkConfig) -> SetupResult<()> {
        let dest_escaped = config.destination.replace(" ", "\\ ");
        let command = format!(
            "mkdir -p $(dirname {}) && ln -fsv {} {}",
            dest_escaped, config.source, dest_escaped
        );

        println!("Executing: sh -c \"{}\"", command);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| crate::symlinks::SetupError::IoError(e.to_string()))?;

        // Print stdout
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            print!("{}", stdout);
        }

        // Print stderr
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprint!("{}", stderr);
        }

        if output.status.success() {
            println!("{}", config.success_message);
            Ok(())
        } else {
            Err(crate::symlinks::SetupError::CommandFailed {
                command,
                exit_code: output.status.code(),
            })
        }
    }
}
