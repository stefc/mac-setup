use crate::detectors::app_detector::AppDetector;
use std::env;
use std::path::Path;

/// Detects if oh-my-zsh is installed
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
}