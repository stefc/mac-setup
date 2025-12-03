use std::env;
// no local io usage

mod detectors;
use detectors::{AppDetector, WezTermDetector, OhMyZshDetector, VSCodeDetector, YaziDetector};
mod common;
use common::replace_home_with_tilde;
mod symlinks;
use symlinks::{SymlinkCreator, ShellSymlinkCreator, SymlinkConfig, SetupResult};
mod configurators;
use configurators::{Configurator, ZshrcConfigurator, YaziConfigurator};
mod logging;
use logging::{Log, MemoryLogger, render_ui};

fn main() {
    let orchestrator = SetupOrchestrator::new(ShellSymlinkCreator);
    let mut logger = MemoryLogger::default();

    let res = orchestrator.run_with_logger(&mut logger);

    // Render a simple Ratatui UI summarizing the outcome
    let snapshot = logger.snapshot();
    if let Err(e) = render_ui(&snapshot, res.as_ref().err().map(|e| e.to_string())) {
        eprintln!("Failed to render UI: {}", e);
    }

    // Also print a plain-text summary to stdout for non-TUI contexts
    if snapshot.groups.is_empty() {
        // Fallback: no groups recorded
        println!("Summary: no changes");
    } else {
        let parts: Vec<String> = snapshot
            .groups
            .iter()
            .map(|g| format!("{}: {}", g.title, g.affected_count))
            .collect();
        println!("Summary — {}", parts.join(" · "));
    }

    if let Err(e) = res {
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

    fn run_with_logger(&self, logger: &mut dyn Log) -> SetupResult<()> {
        logger.info(format!("Current working directory: {}", current_working_directory()));
        logger.info(format!("Executable directory: {}", executable_directory()));
        self.run_configurators(logger)?;
        self.setup_symlinks(logger)?;

        Ok(())
    }

    fn run_configurators(&self, logger: &mut dyn Log) -> SetupResult<()> {
        let configurators: Vec<Box<dyn Configurator>> = vec![
            Box::new(YaziConfigurator),
            Box::new(ZshrcConfigurator::default()),
        ];
        let mut affected = 0usize;
        for configurator in configurators {
            if configurator.should_run() {
                logger.info(format!("{} configuration needed, configuring...", configurator.name()));
                configurator.configure()?;
                let files = configurator.affected_files();
                for file in files {
                    logger.ok_with_highlight("Configured successfully ->".to_string(), file);
                }
                affected += 1;
            } else {
                logger.info(format!("{} configuration not needed, skipping.", configurator.name()));
            }
        }

        logger.add_group("Configurators".to_string(), affected);

        Ok(())
    }

    fn setup_symlinks(&self, logger: &mut dyn Log) -> SetupResult<()> {
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

        let mut affected = 0usize;
        for (detector, config) in configs {
            if detector.is_installed() {
                logger.info(format!(
                    "{} is installed, creating symlink for {} config...",
                    detector.name(),
                    config.installer_name
                ));
                self.symlink_creator.create(&config)?;
                logger.ok_with_highlight(format!("{} ->", config.success_message), config.destination.clone());
                affected += 1;
            } else {
                logger.warn(format!(
                    "{} is not installed, skipping {} config symlink creation.",
                    detector.name(),
                    config.installer_name
                ));
            }
        }

        logger.add_group("Symlinks".to_string(), affected);

        Ok(())
    }
}

/// Print current working directory with tilde substitution
fn current_working_directory() -> String {
    let path = env::current_dir().expect("Failed to get current working directory");
    replace_home_with_tilde(path.display().to_string())
}

fn executable_directory() -> String {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    exe_path
        .parent()
        .map(|p| replace_home_with_tilde(p.display().to_string()))
        .unwrap_or_else(|| "<unknown>".to_string())
}

// logging moved to `logging` module
