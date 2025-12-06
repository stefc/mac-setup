use crate::symlinks::SymlinkConfig;
use crate::symlinks::SetupResult;

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

        match crate::common::run_command("sh", &["-c", &command]) {
            Ok(Some(stdout)) => {
                if !stdout.is_empty() {
                    print!("{}", stdout);
                }
                Ok(())
            }
            Ok(None) => Err(crate::common::SetupError::CommandFailed { command, exit_code: None }),
            Err(e) => Err(crate::common::SetupError::Io(e)),
        }
    }
}
