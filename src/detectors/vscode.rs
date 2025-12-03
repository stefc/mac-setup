use std::path::Path;
use crate::detectors::app_detector::AppDetector;

/// Detects if Visual Studio Code is installed
pub struct VSCodeDetector;

impl AppDetector for VSCodeDetector {
    fn is_installed(&self) -> bool {
        Path::new("/Applications/Visual Studio Code.app").exists()
    }

    fn name(&self) -> &str {
        "Visual Studio Code"
    }
}
