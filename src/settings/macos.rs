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
        self.set_global_setting_value("com.apple.swipescrolldirection", false)?;
        self.enable_trackpad_tap_to_click()?;
        self.activate_settings()?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "macOS Settings"
    }
}

impl MacOSSettings {

    fn activate_settings(&self) -> Result<(), String> {
        let output = Command::new("/System/Library/PrivateFrameworks/SystemAdministration.framework/Resources/activateSettings")
            .arg("-u")
            .output()
            .map_err(|e| format!("Failed to activate settings: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to activate settings: {}", stderr));
        }

        Ok(())
    }

    fn enable_trackpad_tap_to_click(&self) -> Result<(), String> {
        self.set_local_setting_value("com.apple.AppleMultitouchTrackpad", "Clicking", true)?;
        Ok(())
    }

    fn set_local_setting_value(&self, domain: &str, key: &str, value: bool) -> Result<(), String> {
        let bool_str = if value { "true" } else { "false" };
        let output = Command::new("defaults")
            .args(&["write", domain, key, "-bool", bool_str])
            .output()
            .map_err(|e| format!("Failed to set {} {}: {}", domain, key, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set {} {}: {}", domain, key, stderr));
        }

        Ok(())
    }

    fn set_global_setting_value(&self, key: &str, value: bool) -> Result<(), String> {
        let bool_str = if value { "true" } else { "false" };
        let output = Command::new("defaults")
            .args(&["write", "-g", key, "-bool", bool_str])
            .output()
            .map_err(|e| format!("Failed to set mouse {}: {}", key, e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to set mouse {}: {}", key, stderr));
        }

        Ok(())
    }

}
