pub mod platform;
pub mod macos;
pub mod linux;
pub mod windows;

pub use platform::Platform;
pub use macos::MacOSSettings;
pub use linux::LinuxSettings;
pub use windows::WindowsSettings;

/// Trait for platform-specific system settings configuration
pub trait SystemSettings {
    /// Get the platform this settings handler is for
    #[allow(dead_code)]
    fn platform(&self) -> Platform;

    /// Apply platform-specific system settings
    fn apply(&self) -> Result<(), String>;

    /// Get a human-readable name for this settings configuration
    fn name(&self) -> &'static str;
}

/// Factory function to create platform-specific settings handler
pub fn create_platform_settings(platform: Platform) -> Box<dyn SystemSettings> {
    match platform {
        Platform::MacOS => Box::new(MacOSSettings),
        Platform::Linux => Box::new(LinuxSettings),
        Platform::Windows => Box::new(WindowsSettings),
    }
}
