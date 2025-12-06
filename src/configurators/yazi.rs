use crate::detectors::{YaziDetector, app_detector::AppDetector};
use crate::symlinks::{SetupResult, SetupError};
use crate::configurators::Configurator;
use crate::common::Log;

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
    fn run_configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        let package_name = "Chromium-3-Oxide/everforest-medium";
        
        // Check if the package is already installed
        if self.is_package_installed(package_name) {
            logger.info(&format!("Yazi package already installed: {}", package_name));
            return Ok(());
        }
        
        // Run the command to install the everforest-medium package
        match crate::common::run_command("ya", &["pkg", "add", package_name]) {
            Ok(Some(_)) => {
                logger.ok_with_highlight("Added Yazi package ->", package_name);
            }
            Ok(None) => {
                return Err(SetupError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to add Yazi package: command returned non-zero status",
                )));
            }
            Err(e) => return Err(SetupError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to execute 'ya pkg add': {}", e),
            ))),
        }



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
    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        self.run_configure(logger)
    }

    fn affected_files(&self) -> Vec<String> {
        // Yazi configuration acts via package manager; no direct file paths affected here
        Vec::new()
    }
}
