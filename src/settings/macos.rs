use super::{Platform, SetupResult, SystemSettings};

/// macOS-specific system settings
pub struct MacOSSettings;

impl SystemSettings for MacOSSettings {
    fn platform(&self) -> Platform {
        Platform::MacOS
    }

    fn apply(&self) -> SetupResult<()> {
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
    fn activate_settings(&self) -> SetupResult<()> {
        crate::common::run_command(
            "/System/Library/PrivateFrameworks/SystemAdministration.framework/Resources/activateSettings",
            &["-u"],
        )?;
        Ok(())
    }

    fn enable_trackpad_tap_to_click(&self) -> SetupResult<()> {
        self.set_local_setting_value("com.apple.AppleMultitouchTrackpad", "Clicking", true)?;
        Ok(())
    }

    fn set_local_setting_value(&self, domain: &str, key: &str, value: bool) -> SetupResult<()> {
        let bool_str = if value { "true" } else { "false" };
        crate::common::run_command("defaults", &["write", domain, key, "-bool", bool_str])?;
        Ok(())
    }

    fn set_global_setting_value(&self, key: &str, value: bool) -> SetupResult<()> {
        let bool_str = if value { "true" } else { "false" };
        crate::common::run_command("defaults", &["write", "-g", key, "-bool", bool_str])?;
        Ok(())
    }
}
