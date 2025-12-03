use std::path::Path;
use crate::detectors::app_detector::AppDetector;

/// Detects if WezTerm is installed
pub struct WezTermDetector;

impl AppDetector for WezTermDetector {
    fn is_installed(&self) -> bool {
        Path::new("/Applications/WezTerm.app").exists()
    }

    fn name(&self) -> &'static str {
        "WezTerm"
    }
}
