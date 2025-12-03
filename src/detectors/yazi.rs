use std::process::Command;
use crate::detectors::app_detector::AppDetector;

/// Detects if Yazi is installed
pub struct YaziDetector;

impl AppDetector for YaziDetector {
    fn is_installed(&self) -> bool {
        Command::new("which")
            .arg("yazi")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn name(&self) -> &str {
        "Yazi"
    }
}
