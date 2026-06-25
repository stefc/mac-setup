use crate::common::{Log, Platform, replace_home_with_tilde};
use std::env;

pub fn log_environment_info(logger: &mut dyn Log, platform: &Platform) {
    logger.info("▶ Environment");

    let mut items_count = 3;

    logger.ok_with_highlight("Detected platform ->", &platform.to_string());
    logger.ok_with_highlight("Current working directory ->", &current_working_directory());
    logger.ok_with_highlight("Executable directory ->", &executable_directory());

    if let Some(serial) = platform.get_serial_number() {
        logger.ok_with_highlight("Serial number ->", &serial);
        items_count += 1;
    }

    logger.add_group("Environment", items_count);
}

fn current_working_directory() -> String {
    let path = env::current_dir().expect("Failed to get current working directory");
    replace_home_with_tilde(&path)
}

fn executable_directory() -> String {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    exe_path
        .parent()
        .map(replace_home_with_tilde)
        .unwrap_or_else(|| "<unknown>".into())
}
