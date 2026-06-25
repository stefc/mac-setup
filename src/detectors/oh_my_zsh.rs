use crate::detectors::app_detector::AppDetector;
use crate::symlinks::SymlinkConfig;
use std::env;
use std::path::Path;

pub struct OhMyZshDetector;

impl AppDetector for OhMyZshDetector {
    fn is_installed(&self) -> bool {
        if let Some(home_dir) = env::var_os("HOME") {
            let home_path = Path::new(&home_dir);
            home_path.join(".oh-my-zsh").exists()
        } else {
            false
        }
    }

    fn name(&self) -> &'static str {
        "oh-my-zsh"
    }

    fn symlinks(&self, config_dir: &Path) -> Vec<SymlinkConfig> {
        vec![SymlinkConfig {
            source: config_dir.join("stefc.zsh-theme"),
            destination: "~/.oh-my-zsh/themes/stefc.zsh-theme",
            installer_name: "oh-my-zsh",
        }]
    }
}
