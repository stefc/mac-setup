use crate::detectors::{YaziDetector, app_detector::AppDetector};
use crate::symlinks::{SetupResult, SetupError};
use crate::configurators::Configurator;

/// Configurator for Yazi file manager
pub struct YaziConfigurator;

impl YaziConfigurator {
    /// Check if Yazi is installed
    fn is_installed(&self) -> bool {
        YaziDetector.is_installed()
    }

    /// Check if a Yazi package is already installed
    fn is_package_installed(&self, package_name: &str) -> bool {
        match crate::common::run_command("ya", &["pkg", "list"]) {
            Ok(Some(stdout)) => stdout.contains(package_name),
            _ => false,
        }
    }

    /// Configure Yazi by installing the everforest-medium theme package
    fn run_configure(&self) -> SetupResult<()> {
        let package_name = "Chromium-3-Oxide/everforest-medium";
        
        // Check if the package is already installed
        if self.is_package_installed(package_name) {
            println!("Yazi configuration not needed; package '{}' already installed", package_name);
            return Ok(());
        }
        
        // Run the command to install the everforest-medium package
        match crate::common::run_command("ya", &["pkg", "add", package_name]) {
            Ok(Some(_)) => {}
            Ok(None) => return Err(SetupError::IoError("Failed to add Yazi package: command returned non-zero status".to_string())),
            Err(e) => return Err(SetupError::IoError(format!("Failed to execute 'ya pkg add': {}", e))),
        }

        println!("Yazi configured successfully");
        println!("  - Added package: {}", package_name);

        Ok(())
    }
}

impl Configurator for YaziConfigurator {
    fn name(&self) -> &'static str {
        "Yazi"
    }

    fn should_run(&self) -> bool {
        if !self.is_installed() {
            return false;
        }
        // Only run if required package is missing
        !self.is_package_installed("Chromium-3-Oxide/everforest-medium")
    }

    fn configure(&self) -> SetupResult<()> {
        self.run_configure()
    }

    fn affected_files(&self) -> Vec<String> {
        // Yazi configuration acts via package manager; no direct file paths affected here
        Vec::new()
    }
}
