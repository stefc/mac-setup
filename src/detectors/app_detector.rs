/// Trait for detecting if an application is installed
pub trait AppDetector {
    fn is_installed(&self) -> bool;
    fn name(&self) -> &'static str;
    fn symlinks(&self, _config_dir: &std::path::Path) -> Vec<crate::symlinks::SymlinkConfig> {
        vec![]
    }
}
