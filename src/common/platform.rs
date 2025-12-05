/// Represents the operating system the program is running on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    MacOS,
    Linux,
    Windows,
}

impl Platform {
    /// Detect the current platform from compile-time constants.
    ///
    /// This returns a best-effort mapping of `std::env::consts::OS` to
    /// the `Platform` enum. Unknown platforms fall back to `Linux`.
    pub fn detect() -> Self {
        match std::env::consts::OS {
            "macos" => Platform::MacOS,
            "linux" => Platform::Linux,
            "windows" => Platform::Windows,
            _ => Platform::Linux,
        }
    }

    /// Human-friendly name for the platform.
    pub fn as_str(&self) -> &str {
        match self {
            Platform::MacOS => "macOS",
            Platform::Linux => "Linux",
            Platform::Windows => "Windows",
        }
    }

    /// Attempt to retrieve the machine serial number on macOS.
    ///
    /// On non-macOS targets this always returns `None`. On macOS it runs
    /// `ioreg -l` and extracts the `IOPlatformSerialNumber` if available.
    ///
    /// This method intentionally returns `Option<String>` so callers can
    /// simply ignore the value when it's not present.
    #[allow(unused_variables)]
    pub fn get_serial_number(&self) -> Option<String> {
        #[cfg(not(target_os = "macos"))]
        {
            // Serial numbers are only supported on macOS in this codebase.
            None
        }

        #[cfg(target_os = "macos")]
        {
            use regex::Regex;

            if *self != Platform::MacOS {
                return None;
            }

            // Prefer the small helper in `common::run_command` which returns
            // Ok(Some(stdout)) when the command succeeded.
            match crate::common::run_command("ioreg", &["-l"]) {
                Ok(Some(stdout)) => {
                    // Look for: "IOPlatformSerialNumber" = "C02..."
                    let re = Regex::new(r#""IOPlatformSerialNumber""\s*=\s*"([^"]+)""#).ok()?;
                    re.captures(&stdout)
                        .and_then(|cap| cap.get(1))
                        .map(|m| m.as_str().to_string())
                }
                _ => None,
            }
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
