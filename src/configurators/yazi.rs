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

    /// Configure Yazi by installing the everforest-medium theme package
    pub fn configure(&self) -> SetupResult<()> {
        if !self.is_installed() {
            println!("Yazi is not installed, skipping Yazi configuration.");
            return Ok(());
        }

        println!("Configuring Yazi...");
        
        // Run the command to install the everforest-medium package
        let output = Command::new("ya")
            .args(&["pkg", "add", "Chromium-3-Oxide/everforest-medium"])
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
        println!("  - Added package: Chromium-3-Oxide/everforest-medium");

        Ok(())
    }
}
