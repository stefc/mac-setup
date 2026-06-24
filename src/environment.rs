use crate::common::{Log, Platform, replace_home_with_tilde};
use std::env;

pub fn log_environment_info(logger: &mut dyn Log, platform: &Platform) {
    logger.info("▶ Environment");

    let mut env_info = vec![
        ("Detected platform ->", platform.as_str().to_string()),
        ("Current working directory ->", current_working_directory()),
        ("Executable directory ->", executable_directory()),
    ];

    if let Some(serial) = platform.get_serial_number() {
        env_info.push(("Serial number ->", serial));
    }

    let items_count = env_info.len();

    for (key, value) in env_info {
        logger.ok_with_highlight(key, &value);
    }

    logger.add_group("Environment", items_count);
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