use crate::detectors::app_detector::AppDetector;
use crate::detectors::which::is_program_in_path;

pub struct HelixDetector;

impl AppDetector for HelixDetector {
    fn is_installed(&self) -> bool {
        is_program_in_path("hx")
    }

    fn name(&self) -> &'static str {
        "Helix"
    }
}