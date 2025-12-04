use super::{Platform, SystemSettings};

/// Windows-specific system settings
pub struct WindowsSettings;

impl SystemSettings for WindowsSettings {
    fn platform(&self) -> Platform {
        Platform::Windows
    }

    fn apply(&self) -> Result<(), String> {
        // Add Windows-specific settings here as needed
        Ok(())
    }

    fn name(&self) -> String {
        "Windows Settings".to_string()
    }
}
