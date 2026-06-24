use crate::detectors::app_detector::AppDetector;
use crate::detectors::mac_app::is_mac_app_in_path;

pub struct VSCodeDetector;

impl AppDetector for VSCodeDetector {
    fn is_installed(&self) -> bool {
        is_mac_app_in_path("Visual Studio Code")
    }

    fn name(&self) -> &'static str {
        "Visual Studio Code"
    }
}