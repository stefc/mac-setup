use crate::common::Log;
use crate::detectors::{
    AppDetector, HelixDetector, OhMyZshDetector, VSCodeDetector, WezTermDetector, YaziDetector,
};
use crate::symlinks::{SetupResult, SymlinkConfig};
use std::env;
use std::path::Path;

pub fn setup_symlinks(logger: &mut dyn Log) -> SetupResult<()> {
    logger.info("▶ Create Symlinks");

    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
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
            let symlinks = get_symlinks_for_app(detector.name(), &config_dir);
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

fn get_symlinks_for_app(app_name: &str, config_dir: &Path) -> Vec<SymlinkConfig> {
    match app_name {
        "WezTerm" => vec![
            SymlinkConfig {
                source: config_dir.join(".wezterm.lua"),
                destination: "~/.wezterm.lua",
                installer_name: "WezTerm",
            },
            SymlinkConfig {
                source: config_dir.join("wezterm-theme/warm-burnout-light.toml"),
                destination: "~/.config/wezterm/colors/warm-burnout-light.toml",
                installer_name: "WezTerm-Warm Burnout",
            },
            SymlinkConfig {
                source: config_dir.join("wezterm-theme/warm-burnout-dark.toml"),
                destination: "~/.config/wezterm/colors/warm-burnout-dark.toml",
                installer_name: "WezTerm-Warm Burnout",
            },
        ],
        "oh-my-zsh" => vec![SymlinkConfig {
            source: config_dir.join("stefc.zsh-theme"),
            destination: "~/.oh-my-zsh/themes/stefc.zsh-theme",
            installer_name: "oh-my-zsh",
        }],
        "Visual Studio Code" => vec![SymlinkConfig {
            source: config_dir.join("code.settings.json"),
            destination: "~/Library/Application Support/Code/User/settings.json",
            installer_name: "Visual Studio Code",
        }],
        "Yazi" => vec![SymlinkConfig {
            source: config_dir.join("yazi.theme.toml"),
            destination: "~/.config/yazi/theme.toml",
            installer_name: "Yazi",
        }],
        "Helix" => vec![
            SymlinkConfig {
                source: config_dir.join("helix.config.toml"),
                destination: "~/.config/helix/config.toml",
                installer_name: "Helix",
            },
            SymlinkConfig {
                source: config_dir.join("helix-theme/warm-burnout-light.toml"),
                destination: "~/.config/helix/themes/warm-burnout-light.toml",
                installer_name: "Helix-Warm Burnout",
            },
            SymlinkConfig {
                source: config_dir.join("helix-theme/warm-burnout-dark.toml"),
                destination: "~/.config/helix/themes/warm-burnout-dark.toml",
                installer_name: "Helix-Warm Burnout",
            },
        ],
        _ => vec![],
    }
}

fn symlink_create(config: &SymlinkConfig) -> SetupResult<()> {
    let dest_escaped = config.destination.replace(" ", "\\ ");
    let src_escaped = config.source.display().to_string().replace(" ", "\\ ");
    let command = format!(
        "mkdir -p $(dirname {}) && ln -fsv {} {}",
        dest_escaped, src_escaped, dest_escaped
    );

    let stdout = crate::common::run_command("sh", &["-c", &command])?;
    if !stdout.is_empty() {
        print!("{}", stdout);
    }
    Ok(())
}
