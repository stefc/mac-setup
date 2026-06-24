use crate::detectors::app_detector::AppDetector;
use crate::detectors::which::is_program_in_path;
use crate::symlinks::SymlinkConfig;

pub struct HelixDetector;

impl AppDetector for HelixDetector {
    fn is_installed(&self) -> bool {
        is_program_in_path("hx")
    }

    fn name(&self) -> &'static str {
        "Helix"
    }

    fn symlinks(&self, config_dir: &std::path::Path) -> Vec<SymlinkConfig> {
        vec![
            SymlinkConfig {
                source: config_dir.join("helix.config.toml"),
                destination: "~/.config/helix/config.toml",
                installer_name: "Helix",
            },
            SymlinkConfig {
                source: config_dir.join("helix-theme/warm-burnout-light.toml"),
                destination: "~/.config/helix/themes/warm-burnout-light.toml",
                installer_name: "Helix-Warm Burnout",
            },
            SymlinkConfig {
                source: config_dir.join("helix-theme/warm-burnout-dark.toml"),
                destination: "~/.config/helix/themes/warm-burnout-dark.toml",
                installer_name: "Helix-Warm Burnout",
            },
        ]
    }
}