use std::path;
use crate::detectors::app_detector::AppDetector;
use crate::detectors::mac_app::is_mac_app_in_path;
use crate::symlinks::SymlinkConfig;

pub struct WezTermDetector;

impl AppDetector for WezTermDetector {
    fn is_installed(&self) -> bool {
        is_mac_app_in_path("WezTerm")
    }

    fn name(&self) -> &'static str {
        "WezTerm"
    }

    fn symlinks(&self, config_dir: &path::Path) -> Vec<SymlinkConfig> {
        vec![
            SymlinkConfig {
                source: config_dir.join(".wezterm.lua"),
                destination: "~/.wezterm.lua",
                installer_name: "WezTerm",
            },
            SymlinkConfig {
                source: config_dir.join("wezterm-theme/warm-burnout-light.toml"),
                destination: "~/.config/wezterm/colors/warm-burnout-light.toml",
                installer_name: "WezTerm-Warm Burnout",
            },
            SymlinkConfig {
                source: config_dir.join("wezterm-theme/warm-burnout-dark.toml"),
                destination: "~/.config/wezterm/colors/warm-burnout-dark.toml",
                installer_name: "WezTerm-Warm Burnout",
            },
        ]
    }
}