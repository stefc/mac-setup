use std::process::Command;

/// Represents the operating system the program is running on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    MacOS,
    Linux,
    Windows,
}

impl Platform {
    /// Detect the current platform
    pub fn detect() -> Self {
        match std::env::consts::OS {
            "macos" => Platform::MacOS,
            "linux" => Platform::Linux,
            "windows" => Platform::Windows,
            _ => Platform::Linux, // Default fallback
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Platform::MacOS => "macOS",
            Platform::Linux => "Linux",
            Platform::Windows => "Windows",
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait for platform-specific system settings configuration
pub trait SystemSettings {
    /// Get the platform this settings handler is for
    #[allow(dead_code)]
    fn platform(&self) -> Platform;

    /// Apply platform-specific system settings
    fn apply(&self) -> Result<(), String>;

    /// Get a human-readable name for this settings configuration
    fn name(&self) -> String;
}

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

    fn name(&self) -> String {
        "macOS Settings".to_string()
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

/// Factory function to create platform-specific settings handler
pub fn create_platform_settings(platform: Platform) -> Box<dyn SystemSettings> {
    match platform {
        Platform::MacOS => Box::new(MacOSSettings),
        Platform::Linux => Box::new(LinuxSettings),
        Platform::Windows => Box::new(WindowsSettings),
    }
}
