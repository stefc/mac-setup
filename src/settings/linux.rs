use super::{Platform, SystemSettings};

/// Linux-specific system settings
pub struct LinuxSettings;

impl SystemSettings for LinuxSettings {
    fn platform(&self) -> Platform {
        Platform::Linux
    }

    fn apply(&self) -> Result<(), String> {
        // Add Linux-specific settings here as needed
        Ok(())
    }

    fn name(&self) -> String {
        "Linux Settings".to_string()
    }
}
