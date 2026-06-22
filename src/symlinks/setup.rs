use crate::common::Log;
use crate::detectors::{
    AppDetector, HelixDetector, OhMyZshDetector, VSCodeDetector, WezTermDetector, YaziDetector
};
use crate::symlinks::{SetupResult, ShellSymlinkCreator, SymlinkConfig, SymlinkCreator};
use std::env;

pub fn setup_symlinks(
    logger: &mut dyn Log,
) -> SetupResult<()> {
    let symlink_creator = ShellSymlinkCreator;
    setup_symlinks_impl(logger, &symlink_creator)
}

fn setup_symlinks_impl(
    logger: &mut dyn Log,
    symlink_creator: &dyn SymlinkCreator,
) -> SetupResult<()> {
    logger.info("▶ Create Symlinks");
    // Get the executable directory and construct config path
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    let config_dir = exe_dir.join("config");
    let config_dir_str = config_dir.display().to_string();

    let configs: Vec<(&dyn AppDetector, Vec<SymlinkConfig>)> = vec![
        (
            &WezTermDetector as &dyn AppDetector,
            vec![
                SymlinkConfig {
                    source: format!("{}/.wezterm.lua", config_dir_str),
                    destination: "~/.wezterm.lua",
                    installer_name: "WezTerm"
                },
                SymlinkConfig {
                    source: format!("{}/wezterm-theme/warm-burnout-light.toml", config_dir_str),
                    destination: "~/.config/wezterm/colors/warm-burnout-light.toml",
                    installer_name: "WezTerm-Warm Burnout"
                },
                SymlinkConfig {
                    source: format!("{}/wezterm-theme/warm-burnout-dark.toml", config_dir_str),
                    destination: "~/.config/wezterm/colors/warm-burnout-dark.toml",
                    installer_name: "WezTerm-Warm Burnout"
                },
            ],
        ),
        (
            &OhMyZshDetector as &dyn AppDetector,
            vec![
                SymlinkConfig {
                    source: format!("{}/stefc.zsh-theme", config_dir_str),
                    destination: "~/.oh-my-zsh/themes/stefc.zsh-theme",
                    installer_name: "oh-my-zsh"
                },
            ],
        ),
        (
            &VSCodeDetector as &dyn AppDetector,
            vec![
                SymlinkConfig {
                    source: format!("{}/code.settings.json", config_dir_str),
                    destination: "~/Library/Application Support/Code/User/settings.json",
                    installer_name: "Visual Studio Code"
                },
            ],
        ),
        (
            &YaziDetector as &dyn AppDetector,
            vec![
                SymlinkConfig {
                    source: format!("{}/yazi.theme.toml", config_dir_str),
                    destination: "~/.config/yazi/theme.toml",
                    installer_name: "Yazi"
                },
            ],
        ),
        (
            &HelixDetector as &dyn AppDetector,
            vec![
                SymlinkConfig {
                    source: format!("{}/helix.config.toml", config_dir_str),
                    destination: "~/.config/helix/config.toml",
                    installer_name: "Helix"
                },
                SymlinkConfig {
                    source: format!("{}/helix-theme/warm-burnout-light.toml", config_dir_str),
                    destination: "~/.config/helix/themes/warm-burnout-light.toml",
                    installer_name: "Helix-Warm Burnout"
                },
                SymlinkConfig {
                    source: format!("{}/helix-theme/warm-burnout-dark.toml", config_dir_str),
                    destination: "~/.config/helix/themes/warm-burnout-dark.toml",
                    installer_name: "Helix-Warm Burnout"
                },
            ],
        )
    ];

    let mut affected = 0usize;
    for (detector, symlinks) in configs {
        if detector.is_installed() {
            for config in symlinks {
                symlink_creator.create(&config)?;
                logger.ok_with_highlight(
                    "Symlink created successfully",
                    &config.destination,
                );
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
