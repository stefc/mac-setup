pub mod app_detector;
pub mod wezterm;
pub mod oh_my_zsh;
pub mod vscode;
pub mod yazi;
pub mod helix;
pub mod which;
pub mod mac_app;

pub use app_detector::AppDetector;
pub use wezterm::WezTermDetector;
pub use oh_my_zsh::OhMyZshDetector;
pub use vscode::VSCodeDetector;
pub use yazi::YaziDetector;
pub use helix::HelixDetector;
