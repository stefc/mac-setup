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

    #[cfg(target_os = "macos")]
    pub fn get_serial_number(&self) -> Option<String> {
        use regex::Regex;
        match self {
            Platform::MacOS => {
                if let Ok(Some(stdout)) = crate::common::run_command("ioreg", &["-l"]) {
                    let re = Regex::new(r#""IOPlatformSerialNumber""\s*=\s*"([^"]+)""#).ok()?;
                    re.captures(&stdout)
                        .and_then(|cap| cap.get(1))
                        .map(|m| m.as_str().to_string())
                } else {
                    None
                }
            }
            Platform::Linux | Platform::Windows => None,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
