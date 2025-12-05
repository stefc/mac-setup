// Re-export commonly used items from the symlinks module root so callers (like main)
// can import them from `crate::symlinks` as before.
mod creator;
pub mod setup;

pub use creator::{ShellSymlinkCreator, SymlinkCreator};
pub use crate::common::{SetupError, SetupResult};

/// Configuration for a symlink setup task
#[derive(Clone)]
pub struct SymlinkConfig {
    pub source: String,
    pub destination: String,
    pub installer_name: &'static str,
    pub success_message: &'static str,
}



