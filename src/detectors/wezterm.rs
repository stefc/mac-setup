use crate::detectors::mac_app::MacAppDetector;

pub const WEZTERM_DETECTOR: MacAppDetector = MacAppDetector::new("WezTerm", "WezTerm");

pub use WEZTERM_DETECTOR as WezTermDetector;
