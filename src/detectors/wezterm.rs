use crate::detectors::app_detector::AppDetector;
use crate::detectors::mac_app::is_mac_app_in_path;

pub struct WezTermDetector;

impl AppDetector for WezTermDetector {
    fn is_installed(&self) -> bool {
        is_mac_app_in_path("WezTerm")
    }

    fn name(&self) -> &'static str {
        "WezTerm"
    }
}