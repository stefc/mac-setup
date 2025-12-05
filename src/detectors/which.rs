use crate::detectors::app_detector::AppDetector;

/// A reusable which-based detector for simple binaries
#[derive(Copy, Clone)]
pub struct WhichDetector {
    pub bin: &'static str,
    pub display_name: &'static str,
}

impl WhichDetector {
    pub const fn new(bin: &'static str, display_name: &'static str) -> Self {
        Self { bin, display_name }
    }
}

impl AppDetector for WhichDetector {
    fn is_installed(&self) -> bool {
        match crate::common::run_command("which", &[self.bin]) {
            Ok(Some(_stdout)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

    fn name(&self) -> &'static str {
        self.display_name
    }
}
