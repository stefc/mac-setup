use std::process::Command;
use crate::detectors::{AppDetector, YaziDetector};
use crate::symlinks::{SetupResult, SetupError};

/// Configurator for Yazi file manager
pub struct YaziConfigurator;

impl YaziConfigurator {
    /// Check if Yazi is installed
    pub fn is_installed(&self) -> bool {
        YaziDetector.is_installed()
    }

    /// Check if a Yazi package is already installed
    fn is_package_installed(&self, package_name: &str) -> bool {
        let output = Command::new("ya")
            .args(&["pkg", "list"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains(package_name);
            }
        }
        false
    }

    /// Configure Yazi by installing the everforest-medium theme package
    pub fn configure(&self) -> SetupResult<()> {
        if !self.is_installed() {
            println!("Yazi is not installed, skipping Yazi configuration.");
            return Ok(());
        }

        println!("Configuring Yazi...");
        
        let package_name = "Chromium-3-Oxide/everforest-medium";
        
        // Check if the package is already installed
        if self.is_package_installed(package_name) {
            println!("Yazi package '{}' is already installed", package_name);
            return Ok(());
        }
        
        // Run the command to install the everforest-medium package
        let output = Command::new("ya")
            .args(&["pkg", "add", package_name])
            .output()
            .map_err(|e| SetupError::IoError(format!("Failed to execute 'ya pkg add': {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SetupError::IoError(format!(
                "Failed to add Yazi package: {}",
                stderr
            )));
        }

        println!("Yazi configured successfully");
        println!("  - Added package: {}", package_name);

        Ok(())
    }
}
