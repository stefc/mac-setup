use super::{Platform, SetupResult, SystemSettings};

/// Linux-specific system settings
pub struct LinuxSettings;

impl SystemSettings for LinuxSettings {
    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn apply(&self) -> SetupResult<()> {
        // Add Linux-specific settings here as needed
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Linux Settings"
    }
}
