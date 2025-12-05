use crate::detectors::mac_app::MacAppDetector;

/// Detects if Yazi is installed (which-based detector)
pub const VS_CODE_DETECTOR: MacAppDetector = MacAppDetector::new("Visual Studio Code", "Visual Studio Code");

pub use VS_CODE_DETECTOR as VSCodeDetector;

