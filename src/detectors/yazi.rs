use crate::detectors::app_detector::AppDetector;
use crate::detectors::which::is_program_in_path;

pub struct YaziDetector;

impl AppDetector for YaziDetector {
    fn is_installed(&self) -> bool {
        is_program_in_path("yazi")
    }

    fn name(&self) -> &'static str {
        "Yazi"
    }
}