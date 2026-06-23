// Re-export commonly used items from the symlinks module root so callers (like main)
// can import them from `crate::symlinks` as before.
pub mod setup;

pub use crate::common::{SetupError, SetupResult};
use std::path::PathBuf;

/// Configuration for a symlink setup task
#[derive(Clone)]
pub struct SymlinkConfig {
    pub source: PathBuf,
    pub destination: &'static str,
    pub installer_name: &'static str,
}
