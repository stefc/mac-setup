mod error;
mod logging;
mod platform;
mod utils;

pub use error::{SetupError, SetupResult};
pub use logging::{Log, MemoryLogger, render_ui};
pub use platform::Platform;
pub use utils::{replace_home_with_tilde, run_command};