use std::env;

mod detectors;
use detectors::{AppDetector, WezTermDetector, OhMyZshDetector};
mod symlinks;
use symlinks::{SymlinkCreator, ShellSymlinkCreator, SymlinkConfig, SetupResult};

// ============================================================================
// SOLID Principles Application:
// S - Single Responsibility: Each trait/struct has one reason to change
// O - Open/Closed: Traits allow extending without modifying existing code
// L - Liskov Substitution: Implementors are substitutable for their traits
// I - Interface Segregation: Small, focused trait interfaces
// D - Dependency Inversion: Depend on abstractions, not concretions
// ============================================================================

// Display is still used for error printing; types now imported from symlinks module

// detectors are defined in src/detectors/mod.rs and re-exported here

// Symlink-related types and implementations moved to symlinks module

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

        let configs = vec![
            (
                Box::new(WezTermDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: "$(pwd)/config/.wezterm.lua".to_string(),
                    destination: "~/.wezterm.lua".to_string(),
                    installer_name: "WezTerm".to_string(),
                    success_message: "Symlink created successfully".to_string(),
                },
            ),
            (
                Box::new(OhMyZshDetector) as Box<dyn AppDetector>,
                SymlinkConfig {
                    source: "$(pwd)/config/stefc.zsh-theme".to_string(),
                    destination: "~/.oh-my-zsh/themes/stefc.zsh-theme".to_string(),
                    installer_name: "oh-my-zsh".to_string(),
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

        Ok(())
    }
}

fn main() {
    let orchestrator = SetupOrchestrator::new(ShellSymlinkCreator);

    if let Err(e) = orchestrator.run() {
        eprintln!("Setup failed: {}", e);
        std::process::exit(1);
    }
}

/// Helper function to replace home directory path with tilde
fn replace_home_with_tilde(path_str: String) -> String {
    if let Some(home_dir) = env::var_os("HOME") {
        if let Some(home_str) = home_dir.to_str() {
            return path_str.replace(home_str, "~");
        }
    }
    path_str
}

/// Print current working directory with tilde substitution
fn print_current_working_directory() {
    match env::current_dir() {
        Ok(path) => {
            let path_str = replace_home_with_tilde(path.display().to_string());
            println!("Current working directory: {}", path_str);
        }
        Err(e) => eprintln!("Failed to get current working directory: {}", e),
    }
}

/// Print executable directory with tilde substitution
fn print_executable_directory() {
    match env::current_exe() {
        Ok(exe_path) => {
            if let Some(exe_dir) = exe_path.parent() {
                let path_str = replace_home_with_tilde(exe_dir.display().to_string());
                println!("Executable directory: {}", path_str);
            } else {
                eprintln!("Failed to get parent directory of executable");
            }
        }
        Err(e) => eprintln!("Failed to get executable path: {}", e),
    }
}
