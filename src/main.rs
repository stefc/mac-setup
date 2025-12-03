use std::env;

mod detectors;
use detectors::{AppDetector, WezTermDetector, OhMyZshDetector, VSCodeDetector, YaziDetector};
mod common;
use common::replace_home_with_tilde;
mod symlinks;
use symlinks::{SymlinkCreator, ShellSymlinkCreator, SymlinkConfig, SetupResult};
mod configurators;
use configurators::{ZshrcConfigurator, YaziConfigurator};

fn main() {
    let orchestrator = SetupOrchestrator::new(ShellSymlinkCreator);

    if let Err(e) = orchestrator.run() {
        eprintln!("Setup failed: {}", e);
        std::process::exit(1);
    }
}

/// Orchestrates setup tasks (Single Responsibility, Dependency Inversion)
struct SetupOrchestrator<C: SymlinkCreator> {
    symlink_creator: C,
}

impl<C: SymlinkCreator> SetupOrchestrator<C> {
    fn new(symlink_creator: C) -> Self {
        Self { symlink_creator }
    }

    fn run(&self) -> SetupResult<()> {
        print_current_working_directory();
        print_executable_directory();

        // Configure Yazi before application detection
        let yazi_configurator = YaziConfigurator;
        if yazi_configurator.is_installed() {
            yazi_configurator.configure()?;
        } else {
            println!("Yazi is not installed, skipping Yazi configuration.");
        }

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
                    installer_name: "WezTerm".to_string(),
                    success_message: "Symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(OhMyZshDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/stefc.zsh-theme", config_dir_str),
                    destination: "~/.oh-my-zsh/themes/stefc.zsh-theme".to_string(),
                    installer_name: "oh-my-zsh".to_string(),
                    success_message: "Theme symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(VSCodeDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/code.settings.json", config_dir_str),
                    destination: "~/Library/Application Support/Code/User/settings.json".to_string(),
                    installer_name: "Visual Studio Code".to_string(),
                    success_message: "Settings symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(YaziDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: format!("{}/yazi.theme.toml", config_dir_str),
                    destination: "~/.config/yazi/theme.toml".to_string(),
                    installer_name: "Yazi".to_string(),
                    success_message: "Theme symlink created successfully".to_string(),
                },
            ),
        ];

        for (detector, config) in configs {
            if detector.is_installed() {
                println!(
                    "{} is installed, creating symlink for {} config...",
                    detector.name(),
                    config.installer_name
                );
                self.symlink_creator.create(&config)?;
            } else {
                println!(
                    "{} is not installed, skipping {} config symlink creation.",
                    detector.name(),
                    config.installer_name
                );
            }
        }

        // Configure .zshrc if it exists
        let zshrc_configurator = ZshrcConfigurator;
        if zshrc_configurator.exists() {
            zshrc_configurator.configure(
                "stefc",
                &["z", "gh"],
                &[("HOMEBREW_NO_AUTO_UPDATE", "1")],
            )?;
        } else {
            println!(".zshrc not found, skipping zsh configuration.");
        }

        Ok(())
    }
}

/// Print current working directory with tilde substitution
fn print_current_working_directory() {
    let path = env::current_dir().expect("Failed to get current working directory");
    let path_str = replace_home_with_tilde(path.display().to_string());
    println!("Current working directory: {}", path_str);
}

/// Print executable directory with tilde substitution
fn print_executable_directory() {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    if let Some(exe_dir) = exe_path.parent() {
        let path_str = replace_home_with_tilde(exe_dir.display().to_string());
        println!("Executable directory: {}", path_str);
    } else {
        eprintln!("Failed to get parent directory of executable");
    }
}
