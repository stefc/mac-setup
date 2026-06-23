use crate::detectors::mac_app::MacAppDetector;

pub const RUSTROVER_DETECTOR: MacAppDetector = MacAppDetector::new("RustRover", "RustRover");

pub use RUSTROVER_DETECTOR as RustRoverDetector;
