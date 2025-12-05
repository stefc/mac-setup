use crate::detectors::which::WhichDetector;

/// Detects if Yazi is installed (which-based detector)
pub const YAZI_DETECTOR: WhichDetector = WhichDetector::new("yazi", "Yazi");

pub use YAZI_DETECTOR as YaziDetector;
