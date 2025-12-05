use std::path::Path;
use crate::detectors::app_detector::AppDetector;

#[derive(Copy, Clone)]
pub struct MacAppDetector {
    pub name: &'static str,
    pub display_name: &'static str,
}

impl MacAppDetector {
    pub const fn new(bin: &'static str, display_name: &'static str) -> Self {
        Self { name: bin, display_name }
    }
}

impl AppDetector for MacAppDetector {
    fn is_installed(&self) -> bool {
        Path::new("/Applications/").join(format!("{}.app", self.name)).exists()
    }

    fn name(&self) -> &'static str {
        self.display_name
    }
}
