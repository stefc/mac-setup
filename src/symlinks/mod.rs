pub mod setup;

pub use crate::common::SetupResult;
use std::path::PathBuf;

/// Configuration for a symlink setup task
#[derive(Clone)]
pub struct SymlinkConfig {
    pub source: PathBuf,
    pub destination: &'static str,
    pub installer_name: &'static str,
    /// Copy the file instead of symlinking (for apps that reject symlinks)
    pub copy: bool,
}
