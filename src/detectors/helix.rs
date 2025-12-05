use crate::detectors::which::WhichDetector;

/// Detects if Helix is installed (which-based detector)
pub const HELIX_DETECTOR: WhichDetector = WhichDetector::new("hx", "Helix");

pub use HELIX_DETECTOR as HelixDetector;
