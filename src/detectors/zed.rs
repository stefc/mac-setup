use crate::detectors::app_detector::AppDetector;
use crate::detectors::mac_app::is_mac_app_in_path;
use crate::symlinks::SymlinkConfig;
use std::path;

pub struct ZedDetector;

impl AppDetector for ZedDetector {
    fn is_installed(&self) -> bool {
        is_mac_app_in_path("Zed")
    }

    fn name(&self) -> &'static str {
        "Zed"
    }

    fn symlinks(&self, config_dir: &path::Path) -> Vec<SymlinkConfig> {
        vec![SymlinkConfig {
            source: config_dir.join("zed.settings.json"),
            destination: "~/.config/zed/settings.json",
            installer_name: "Zed",
            copy: true,
        }]
    }
}
