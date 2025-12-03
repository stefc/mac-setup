use std::env;
use crate::detectors::app_detector::AppDetector;

/// Detects if oh-my-zsh is installed
pub struct OhMyZshDetector;

impl AppDetector for OhMyZshDetector {
    fn is_installed(&self) -> bool {
        env::var_os("HOME")
            .and_then(|home| {
                let mut path = std::path::PathBuf::from(home);
                path.push(".oh-my-zsh");
                Some(path.exists())
            })
            .unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "oh-my-zsh"
    }
}
