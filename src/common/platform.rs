/// Represents the operating system the program is running on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    MacOS,
    Linux,
    Windows,
}

impl Platform {
    pub fn detect() -> Self {
        match std::env::consts::OS {
            "macos" => Platform::MacOS,
            "linux" => Platform::Linux,
            "windows" => Platform::Windows,
            _ => Platform::Linux,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Platform::MacOS => "macOS",
            Platform::Linux => "Linux",
            Platform::Windows => "Windows",
        }
    }

    #[allow(unused_variables)]
    pub fn get_serial_number(&self) -> Option<String> {
        #[cfg(not(target_os = "macos"))]
        {
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
