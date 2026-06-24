use crate::detectors::app_detector::AppDetector;
use crate::detectors::mac_app::is_mac_app_in_path;

pub struct RustRoverDetector;

impl AppDetector for RustRoverDetector {
    fn is_installed(&self) -> bool {
        is_mac_app_in_path("RustRover")
    }

    fn name(&self) -> &'static str {
        "RustRover"
    }
}
