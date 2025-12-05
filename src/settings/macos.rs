use super::{Platform, SystemSettings};

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
        match crate::common::run_command("/System/Library/PrivateFrameworks/SystemAdministration.framework/Resources/activateSettings", &["-u"]) {
            Ok(Some(_stdout)) => Ok(()),
            Ok(None) => Err("Failed to activate settings: command returned non-zero status".to_string()),
            Err(e) => Err(format!("Failed to activate settings: {}", e)),
        }
    }

    fn enable_trackpad_tap_to_click(&self) -> Result<(), String> {
        self.set_local_setting_value("com.apple.AppleMultitouchTrackpad", "Clicking", true)?;
        Ok(())
    }

    fn set_local_setting_value(&self, domain: &str, key: &str, value: bool) -> Result<(), String> {
        let bool_str = if value { "true" } else { "false" };
        match crate::common::run_command("defaults", &["write", domain, key, "-bool", bool_str]) {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(format!("Failed to set {} {}: command returned non-zero status", domain, key)),
            Err(e) => Err(format!("Failed to set {} {}: {}", domain, key, e)),
        }
    }

    fn set_global_setting_value(&self, key: &str, value: bool) -> Result<(), String> {
        let bool_str = if value { "true" } else { "false" };
        match crate::common::run_command("defaults", &["write", "-g", key, "-bool", bool_str]) {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(format!("Failed to set mouse {}: command returned non-zero status", key)),
            Err(e) => Err(format!("Failed to set mouse {}: {}", key, e)),
        }
    }

}
