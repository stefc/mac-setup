use crate::detectors::app_detector::AppDetector;
use crate::detectors::which::is_program_in_path;
use crate::symlinks::SymlinkConfig;
use std::path::Path;

pub struct YaziDetector;

impl AppDetector for YaziDetector {
    fn is_installed(&self) -> bool {
        is_program_in_path("yazi")
    }

    fn name(&self) -> &'static str {
        "Yazi"
    }

    fn symlinks(&self, config_dir: &Path) -> Vec<SymlinkConfig> {
        vec![SymlinkConfig {
            source: config_dir.join("yazi.theme.toml"),
            destination: "~/.config/yazi/theme.toml",
            installer_name: self.name(),
        }]
    }
}
