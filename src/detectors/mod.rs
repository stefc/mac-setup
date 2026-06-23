pub mod app_detector;
pub mod helix;
pub mod mac_app;
pub mod oh_my_zsh;
pub mod rustrover;
pub mod vscode;
pub mod wezterm;
pub mod which;
pub mod yazi;

pub use app_detector::AppDetector;
pub use helix::HelixDetector;
pub use oh_my_zsh::OhMyZshDetector;
pub use rustrover::RustRoverDetector;
pub use vscode::VSCodeDetector;
pub use wezterm::WezTermDetector;
pub use yazi::YaziDetector;
