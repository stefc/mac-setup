use std::path;
use crate::detectors::app_detector::AppDetector;
use crate::detectors::mac_app::is_mac_app_in_path;
use crate::symlinks::SymlinkConfig;

pub struct VSCodeDetector;

impl AppDetector for VSCodeDetector {
    fn is_installed(&self) -> bool {
        is_mac_app_in_path("Visual Studio Code")
    }

    fn name(&self) -> &'static str {
        "Visual Studio Code"
    }

    fn symlinks(&self, config_dir: &path::Path) -> Vec<SymlinkConfig> {
        vec![SymlinkConfig {
            source: config_dir.join("code.settings.json"),
            destination: "~/Library/Application Support/Code/User/settings.json",
            installer_name: "Visual Studio Code",
        }]
    }
}