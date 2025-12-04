use super::{Platform, SystemSettings};
use std::process::Command;

/// macOS-specific system settings
pub struct MacOSSettings;

impl SystemSettings for MacOSSettings {
    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn apply(&self) -> Result<(), String> {
        // Disable natural scrolling (swipe scrolling direction)
        self.disable_natural_scrolling()?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "macOS Settings"
    }
}

impl MacOSSettings {
    /// Disable natural scrolling (reverse scroll direction for trackpad/mouse)
    /// This makes scrolling feel like traditional scroll wheels
    fn disable_natural_scrolling(&self) -> Result<(), String> {
        // Use defaults command to disable natural scrolling
        let output = Command::new("defaults")
            .args(&["write", "-g", "com.apple.swapMouse", "-bool", "true"])
            .output()
            .map_err(|e| format!("Failed to disable natural scrolling: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to disable natural scrolling: {}", stderr));
        }

        Ok(())
    }
}
