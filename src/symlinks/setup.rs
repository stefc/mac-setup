use crate::common::Log;
use crate::detectors::{
    AppDetector, HelixDetector, OhMyZshDetector, VSCodeDetector, WezTermDetector, YaziDetector,
};
use crate::symlinks::{SetupResult, SymlinkConfig};
use std::env;

pub fn setup_symlinks(logger: &mut dyn Log) -> SetupResult<()> {
    logger.info("▶ Create Symlinks");
    // Get the executable directory and construct config path
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let config_dir = exe_dir.join("config");

    let configs: Vec<(&dyn AppDetector, Vec<SymlinkConfig>)> = vec![
        (
            &WezTermDetector as &dyn AppDetector,
            vec![
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
        ),
        (
            &OhMyZshDetector as &dyn AppDetector,
            vec![SymlinkConfig {
                source: config_dir.join("{}/stefc.zsh-theme"),
                destination: "~/.oh-my-zsh/themes/stefc.zsh-theme",
                installer_name: "oh-my-zsh",
            }],
        ),
        (
            &VSCodeDetector as &dyn AppDetector,
            vec![SymlinkConfig {
                source: config_dir.join("{}/code.settings.json"),
                destination: "~/Library/Application Support/Code/User/settings.json",
                installer_name: "Visual Studio Code",
            }],
        ),
        (
            &YaziDetector as &dyn AppDetector,
            vec![SymlinkConfig {
                source: config_dir.join("{}/yazi.theme.toml"),
                destination: "~/.config/yazi/theme.toml",
                installer_name: "Yazi",
            }],
        ),
        (
            &HelixDetector as &dyn AppDetector,
            vec![
                SymlinkConfig {
                    source: config_dir.join("{}/helix.config.toml"),
                    destination: "~/.config/helix/config.toml",
                    installer_name: "Helix",
                },
                SymlinkConfig {
                    source: config_dir.join("{}/helix-theme/warm-burnout-light.toml"),
                    destination: "~/.config/helix/themes/warm-burnout-light.toml",
                    installer_name: "Helix-Warm Burnout",
                },
                SymlinkConfig {
                    source: config_dir.join("{}/helix-theme/warm-burnout-dark.toml"),
                    destination: "~/.config/helix/themes/warm-burnout-dark.toml",
                    installer_name: "Helix-Warm Burnout",
                },
            ],
        ),
    ];

    let mut affected = 0usize;
    for (detector, symlinks) in configs {
        if detector.is_installed() {
            for config in symlinks {
                symlink_create(&config)?;
                logger.ok_with_highlight("Symlink created successfully", &config.destination);
                affected += 1;
            }
        } else {
            for config in symlinks {
                logger.warn(&format!(
                    "{} is not installed, skipping {} config symlink creation.",
                    detector.name(),
                    config.installer_name
                ));
            }
        }
    }

    logger.add_group("Symlinks", affected);

    Ok(())
}

fn symlink_create(config: &SymlinkConfig) -> SetupResult<()> {
    let dest_escaped = config.destination.replace(" ", "\\ ");
    let command = format!(
        "mkdir -p $(dirname {}) && ln -fsv {} {}",
        dest_escaped,
        config.source.display(),
        dest_escaped
    );

    match crate::common::run_command("sh", &["-c", &command]) {
        Ok(Some(stdout)) => {
            if !stdout.is_empty() {
                print!("{}", stdout);
            }
            Ok(())
        }
        Ok(None) => Err(crate::common::SetupError::CommandFailed {
            command,
            exit_code: None,
        }),
        Err(e) => Err(crate::common::SetupError::Io(e)),
    }
}
