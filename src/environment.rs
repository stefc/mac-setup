use crate::common::{replace_home_with_tilde, Log, Platform};
use std::env;

pub fn log_environment_info(logger: &mut dyn Log, platform: &Platform) {
    logger.info("▶ Environment");
    logger.ok_with_highlight("Detected platform ->", platform.as_str());
    logger.ok_with_highlight("Current working directory ->", &current_working_directory());
    logger.ok_with_highlight("Executable directory ->", &executable_directory());
    
    // Display serial number for macOS
    if let Some(serial) = platform.get_serial_number() {
        logger.ok_with_highlight("Serial number ->", &serial);
    }
    
    logger.add_group("Environment", 0); // platform, cwd, exe
}

/// Print current working directory with tilde substitution
fn current_working_directory() -> String {
    let path = env::current_dir().expect("Failed to get current working directory");
    replace_home_with_tilde(&path)
}

fn executable_directory() -> String {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    exe_path
        .parent()
        .map(|p| replace_home_with_tilde(p))
        .unwrap_or_else(|| "<unknown>".to_string())
}
