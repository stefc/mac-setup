
use crate::detectors::{
    AppDetector, WezTermDetector, OhMyZshDetector, VSCodeDetector, YaziDetector, HelixDetector,
};
use crate::common::Log;
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

    let configs = vec![
        (
            Box::new(WezTermDetector) as Box<dyn AppDetector>,
            SymlinkConfig {
                source: format!("{}/.wezterm.lua", config_dir_str),
                destination: "~/.wezterm.lua".to_string(),
                installer_name: "WezTerm",
                success_message: "Symlink created successfully",
            },
        ),
        (
            Box::new(OhMyZshDetector) as Box<dyn AppDetector>,
            SymlinkConfig {
                source: format!("{}/stefc.zsh-theme", config_dir_str),
                destination: "~/.oh-my-zsh/themes/stefc.zsh-theme".to_string(),
                installer_name: "oh-my-zsh",
                success_message: "Theme symlink created successfully",
            },
        ),
        (
            Box::new(VSCodeDetector) as Box<dyn AppDetector>,
            SymlinkConfig {
                source: format!("{}/code.settings.json", config_dir_str),
                destination: "~/Library/Application Support/Code/User/settings.json".to_string(),
                installer_name: "Visual Studio Code",
                success_message: "Settings symlink created successfully",
            },
        ),
        (
            Box::new(YaziDetector) as Box<dyn AppDetector>,
            SymlinkConfig {
                source: format!("{}/yazi.theme.toml", config_dir_str),
                destination: "~/.config/yazi/theme.toml".to_string(),
                installer_name: "Yazi",
                success_message: "Theme symlink created successfully",
            },
        ),
        (
            Box::new(HelixDetector) as Box<dyn AppDetector>,
            SymlinkConfig {
                source: format!("{}/helix.config.toml", config_dir_str),
                destination: "~/.config/helix/config.toml".to_string(),
                installer_name: "Helix",
                success_message: "Theme symlink created successfully",
            },
        ),
    ];

    let mut affected = 0usize;
    for (detector, config) in configs {
        if detector.is_installed() {
            logger.info(&format!(
                "{} is installed, creating symlink for {} config...",
                detector.name(),
                config.installer_name
            ));
            symlink_creator.create(&config)?;
            logger.ok_with_highlight(
                &format!("{} ->", config.success_message),
                &config.destination,
            );
            affected += 1;
        } else {
            logger.warn(&format!(
                "{} is not installed, skipping {} config symlink creation.",
                detector.name(),
                config.installer_name
            ));
        }
    }

    logger.add_group("Symlinks", affected);

    Ok(())
}
