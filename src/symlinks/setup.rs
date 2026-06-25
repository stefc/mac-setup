use crate::{
    common::Log,
    detectors::{
        AppDetector, HelixDetector, OhMyZshDetector, VSCodeDetector, WezTermDetector, YaziDetector,
    },
    symlinks::{SetupResult, SymlinkConfig},
};
use std::{
    env, fs,
    os::unix,
    path::{Path, PathBuf},
};

pub fn setup_symlinks(logger: &mut dyn Log) -> SetupResult<()> {
    logger.info("▶ Create Symlinks");

    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let config_dir = exe_dir.join("config");

    let detectors: Vec<&dyn AppDetector> = vec![
        &WezTermDetector,
        &OhMyZshDetector,
        &VSCodeDetector,
        &YaziDetector,
        &HelixDetector,
    ];

    let mut affected = 0;

    for detector in detectors {
        if detector.is_installed() {
            let symlinks = detector.symlinks(&config_dir);
            for config in symlinks {
                if let Err(e) = symlink_create(&config) {
                    logger.warn(&format!(
                        "Failed to create symlink for {}: {}",
                        config.installer_name, e
                    ));
                } else {
                    logger.ok_with_highlight("Symlink created successfully", config.destination);
                    affected += 1;
                }
            }
        } else {
            logger.warn(&format!(
                "{} is not installed, skipping symlink creation.",
                detector.name()
            ));
        }
    }

    logger.add_group("Symlinks", affected);
    Ok(())
}

fn symlink_create(config: &SymlinkConfig) -> SetupResult<()> {
    let dest_str = config.destination;
    let dest_expanded = dest_str
        .strip_prefix("~/")
        .and_then(|stripped| env::var_os("HOME").map(|home| Path::new(&home).join(stripped)))
        .unwrap_or_else(|| PathBuf::from(dest_str));

    if let Some(parent) = dest_expanded.parent() {
        fs::create_dir_all(parent)?;
    }

    if dest_expanded.exists() || dest_expanded.is_symlink() {
        fs::remove_file(&dest_expanded)?;
    }

    unix::fs::symlink(&config.source, &dest_expanded)?;
    Ok(())
}
