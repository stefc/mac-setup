use std::path::Path;
use crate::symlinks::SymlinkConfig;

/// Trait for detecting if an application is installed
pub trait AppDetector {
    fn is_installed(&self) -> bool;
    fn name(&self) -> &'static str;
    fn symlinks(&self, _config_dir: &Path) -> Vec<SymlinkConfig> {
        vec![]
    }
}
