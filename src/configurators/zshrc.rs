use std::fs;
use std::path::PathBuf;
use std::env;
use crate::symlinks::{SetupResult, SetupError};
use crate::configurators::Configurator;

/// Configurator for .zshrc file
pub struct ZshrcConfigurator {
    theme: String,
    plugins: Vec<String>,
    env_vars: Vec<(String, String)>,
}

impl Default for ZshrcConfigurator {
    fn default() -> Self {
        Self {
            theme: "stefc".to_string(),
            plugins: vec!["z".to_string(), "gh".to_string()],
            env_vars: vec![("HOMEBREW_NO_AUTO_UPDATE".to_string(), "1".to_string())],
        }
    }
}

impl ZshrcConfigurator {
    /// Get the path to .zshrc in the user's home directory
    fn get_zshrc_path() -> SetupResult<PathBuf> {
        env::var_os("HOME")
            .map(|home| {
                let mut path = PathBuf::from(home);
                path.push(".zshrc");
                path
            })
            .ok_or_else(|| SetupError::IoError("HOME environment variable not set".to_string()))
    }

    /// Check if .zshrc exists
    fn exists(&self) -> bool {
        Self::get_zshrc_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    /// Configure .zshrc with the specified theme, plugins, and environment variables
    fn run_configure(&self) -> SetupResult<()> {
        let zshrc_path = Self::get_zshrc_path()?;
        
        println!("Configuring .zshrc at {:?}...", zshrc_path);

        // Read the current content
        let content = fs::read_to_string(&zshrc_path)
            .map_err(|e| SetupError::IoError(format!("Failed to read .zshrc: {}", e)))?;

        // Modify the content
        let plugins_refs: Vec<&str> = self.plugins.iter().map(|s| s.as_str()).collect();
        let env_vars_refs: Vec<(&str, &str)> = self.env_vars.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        let new_content = self.modify_zshrc_content(&content, &self.theme, &plugins_refs, &env_vars_refs);

        // Write back to disk
        fs::write(&zshrc_path, new_content)
            .map_err(|e| SetupError::IoError(format!("Failed to write .zshrc: {}", e)))?;

        println!(".zshrc configured successfully");
        println!("  - Theme set to: {}", self.theme);
        println!("  - Plugins: {}", self.plugins.join(", "));
        for (key, value) in &self.env_vars {
            println!("  - Export {}={}", key, value);
        }

        Ok(())
    }

    /// Modify the .zshrc content by updating theme, plugins, and adding environment variables
    fn modify_zshrc_content(&self, content: &str, theme: &str, plugins_to_add: &[&str], env_vars: &[(&str, &str)]) -> String {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        // Update or add ZSH_THEME
        self.update_or_add_line(&mut lines, "ZSH_THEME", &format!("ZSH_THEME=\"{}\"", theme));
        
        // Extend plugins instead of replacing them
        self.extend_plugins(&mut lines, plugins_to_add);
        
        // Add environment variables if not present
        for (key, value) in env_vars {
            let export_line = format!("export {}={}", key, value);
            if !lines.iter().any(|line| {
                let trimmed = line.trim();
                trimmed.starts_with(&format!("export {}", key)) || trimmed.starts_with(&format!("{}=", key))
            }) {
                lines.push(String::new()); // Add blank line for readability
                lines.push(format!("# Added by mac-setup"));
                lines.push(export_line);
            }
        }
        
        lines.join("\n") + "\n"
    }

    /// Extend the plugins list with new plugins (avoiding duplicates)
    fn extend_plugins(&self, lines: &mut Vec<String>, plugins_to_add: &[&str]) {
        // Find the plugins line
        if let Some(pos) = lines.iter().position(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with('#') && trimmed.starts_with("plugins=")
        }) {
            // Parse existing plugins
            let line = &lines[pos];
            let mut existing_plugins = Vec::new();
            
            // Extract plugins from plugins=(plugin1 plugin2 ...)
            if let Some(start) = line.find('(') {
                if let Some(end) = line.find(')') {
                    let plugins_str = &line[start + 1..end];
                    existing_plugins = plugins_str
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                }
            }
            
            // Add new plugins if not already present
            for plugin in plugins_to_add {
                if !existing_plugins.iter().any(|p| p == plugin) {
                    existing_plugins.push(plugin.to_string());
                }
            }
            
            // Update the line
            lines[pos] = format!("plugins=({})", existing_plugins.join(" "));
        } else {
            // If plugins line doesn't exist, create it with the new plugins
            let plugins_line = format!("plugins=({})", plugins_to_add.join(" "));
            let insert_pos = lines.iter().position(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            }).unwrap_or(0);
            
            lines.insert(insert_pos, plugins_line);
        }
    }

    /// Update an existing line or add a new one
    fn update_or_add_line(&self, lines: &mut Vec<String>, prefix: &str, new_line: &str) {
        // Find the line that starts with the prefix (ignoring leading whitespace and comments)
        if let Some(pos) = lines.iter().position(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with('#') && trimmed.starts_with(prefix)
        }) {
            lines[pos] = new_line.to_string();
        } else {
            // If not found, add it (we'll add it before the first non-comment, non-empty line if possible)
            let insert_pos = lines.iter().position(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            }).unwrap_or(0);
            
            lines.insert(insert_pos, new_line.to_string());
        }
    }
}

impl Configurator for ZshrcConfigurator {
    fn name(&self) -> &str {
        "ZSH"
    }

    fn should_run(&self) -> bool {
        self.exists()
    }

    fn configure(&self) -> SetupResult<()> {
        self.run_configure()
    }
}
