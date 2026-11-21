use crate::{
    common::{Log, replace_home_with_tilde},
    configurators::Configurator,
    symlinks::SetupResult,
};
use std::{env, fs, path::PathBuf};

/// Configurator for .zshrc file
pub struct ZshrcConfigurator {
    theme: &'static str,
    plugins: &'static [&'static str],
    env_vars: &'static [(&'static str, &'static str)],
    functions: &'static [(&'static str, &'static str)],
}

const KC_FUNCTION: &str = r#"# Generic getter
kc() {
  security find-generic-password -a "$USER" -s "$1" -w 2>/dev/null \
    || { echo "kc: no keychain item '$1'" >&2; return 1; }
}"#;

const KC_SET_FUNCTION: &str = r#"# Generic setter (prompts for the secret)
kc-set() {
  security add-generic-password -U -a "$USER" -s "$1" -w
}"#;

impl Default for ZshrcConfigurator {
    fn default() -> Self {
        Self {
            theme: "stefc",
            plugins: &["z", "gh"],
            env_vars: &[("HOMEBREW_NO_AUTO_UPDATE", "1"), ("EDITOR", "hx")],
            functions: &[("kc", KC_FUNCTION), ("kc-set", KC_SET_FUNCTION)],
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
            .ok_or_else(|| {
                crate::common::SetupError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "HOME environment variable not set",
                ))
            })
    }

    /// Check if .zshrc exists
    fn exists(&self) -> bool {
        Self::get_zshrc_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    /// Configure .zshrc with the specified theme, plugins, and environment variables
    fn run_configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        let zshrc_path = Self::get_zshrc_path()?;

        logger.info(&format!("Configuring .zshrc at {:?}...", zshrc_path));

        // Read the current content
        let content = fs::read_to_string(&zshrc_path)?;

        // Modify the content
        let new_content = self.modify_zshrc_content(
            &content,
            self.theme,
            self.plugins,
            self.env_vars,
            self.functions,
        );

        // Write back to disk
        fs::write(&zshrc_path, new_content)?;

        logger.ok_with_highlight(
            "Configured .zshrc at ->",
            &replace_home_with_tilde(&zshrc_path),
        );

        logger.info(".zshrc configured successfully");
        logger.info(&format!("  - Theme set to: {}", self.theme));
        logger.info("  - DISABLE_UNTRACKED_FILES_DIRTY=\"true\"");
        logger.info(&format!("  - Plugins: {}", self.plugins.join(", ")));
        for (key, value) in self.env_vars {
            logger.info(&format!("  - Export {}={}", key, value));
        }
        for (name, _) in self.functions {
            logger.info(&format!("  - Function {}()", name));
        }

        Ok(())
    }

    /// Modify the .zshrc content by updating theme, plugins, and adding environment variables and functions
    fn modify_zshrc_content(
        &self,
        content: &str,
        theme: &str,
        plugins_to_add: &[&str],
        env_vars: &[(&str, &str)],
        functions: &[(&str, &str)],
    ) -> String {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Update or add ZSH_THEME
        self.update_or_add_line(&mut lines, "ZSH_THEME", &format!("ZSH_THEME=\"{}\"", theme));

        // Update or add DISABLE_UNTRACKED_FILES_DIRTY
        self.update_or_add_line(
            &mut lines,
            "DISABLE_UNTRACKED_FILES_DIRTY",
            "DISABLE_UNTRACKED_FILES_DIRTY=\"true\"",
        );

        // Extend plugins instead of replacing them
        self.extend_plugins(&mut lines, plugins_to_add);

        // Add environment variables if not present
        for (key, value) in env_vars {
            let export_line = format!("export {}={}", key, value);
            if !lines.iter().any(|line| {
                let trimmed = line.trim();
                trimmed.starts_with(&format!("export {}", key))
                    || trimmed.starts_with(&format!("{}=", key))
            }) {
                lines.push(String::new()); // Add blank line for readability
                lines.push("# Added by mac-setup".to_string());
                lines.push(export_line);
            }
        }

        // Add shell functions if not present
        for (name, body) in functions {
            let func_prefix = format!("{}()", name);
            if !lines.iter().any(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with('#') && trimmed.starts_with(&func_prefix)
            }) {
                lines.push(String::new());
                for func_line in body.lines() {
                    lines.push(func_line.to_string());
                }
            }
        }

        lines.join("\n") + "\n"
    }

    /// Extend the plugins list with new plugins (avoiding duplicates)
    fn extend_plugins(&self, lines: &mut Vec<String>, plugins_to_add: &[&str]) {
        let pos = lines
            .iter()
            .position(|l| l.trim().starts_with("plugins=") && !l.trim().starts_with('#'));
        let mut plugins: Vec<String> = pos
            .map(|p| &lines[p])
            .and_then(|line| line.split_once('('))
            .and_then(|(_, rest)| rest.split_once(')'))
            .map(|(pl_str, _)| pl_str.split_whitespace().map(String::from).collect())
            .unwrap_or_default();

        for &p in plugins_to_add {
            if !plugins.iter().any(|existing| existing == p) {
                plugins.push(p.to_string());
            }
        }

        let new_line = format!("plugins=({})", plugins.join(" "));
        if let Some(p) = pos {
            lines[p] = new_line;
        } else {
            let insert_pos = lines
                .iter()
                .position(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                .unwrap_or(0);
            lines.insert(insert_pos, new_line);
        }
    }

    /// Update an existing line or add a new one
    fn update_or_add_line(&self, lines: &mut Vec<String>, prefix: &str, new_line: &str) {
        if let Some(pos) = lines
            .iter()
            .position(|l| l.trim().starts_with(prefix) && !l.trim().starts_with('#'))
        {
            lines[pos] = new_line.to_string();
        } else {
            let insert_pos = lines
                .iter()
                .position(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                .unwrap_or(0);
            lines.insert(insert_pos, new_line.to_string());
        }
    }
}

impl Configurator for ZshrcConfigurator {
    fn name(&self) -> &'static str {
        "ZSH"
    }

    fn should_run(&self) -> bool {
        self.exists()
    }

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        self.run_configure(logger)
    }

    fn affected_files(&self) -> Vec<String> {
        Self::get_zshrc_path()
            .map(|path| vec![replace_home_with_tilde(&path)])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_zsh_configurator() -> ZshrcConfigurator {
        ZshrcConfigurator::default()
    }

    #[test]
    fn test_extend_plugins_add_to_existing() {
        let configurator = new_zsh_configurator();
        let mut lines = vec!["plugins=(git zsh-syntax-highlighting)".to_string()];
        configurator.extend_plugins(&mut lines, &["z", "gh"]);
        assert_eq!(lines, vec!["plugins=(git zsh-syntax-highlighting z gh)"]);
    }

    #[test]
    fn test_extend_plugins_add_with_duplicates() {
        let configurator = new_zsh_configurator();
        let mut lines = vec!["plugins=(git z)".to_string()];
        configurator.extend_plugins(&mut lines, &["z", "gh"]);
        assert_eq!(lines, vec!["plugins=(git z gh)"]);
    }

    #[test]
    fn test_extend_plugins_no_existing_plugins_line() {
        let configurator = new_zsh_configurator();
        let mut lines = vec!["some line".to_string()];
        configurator.extend_plugins(&mut lines, &["z", "gh"]);
        assert_eq!(lines, vec!["plugins=(z gh)", "some line"]);
    }

    #[test]
    fn test_extend_plugins_empty_initial_plugins() {
        let configurator = new_zsh_configurator();
        let mut lines = vec!["plugins=()".to_string()];
        configurator.extend_plugins(&mut lines, &["z", "gh"]);
        assert_eq!(lines, vec!["plugins=(z gh)"]);
    }

    #[test]
    fn test_update_or_add_line_updates_existing() {
        let configurator = new_zsh_configurator();
        let mut lines = vec!["ZSH_THEME=\"old_theme\"".to_string()];
        configurator.update_or_add_line(&mut lines, "ZSH_THEME", "ZSH_THEME=\"new_theme\"");
        assert_eq!(lines, vec!["ZSH_THEME=\"new_theme\""]);
    }

    #[test]
    fn test_update_or_add_line_adds_new() {
        let configurator = new_zsh_configurator();
        let mut lines = vec!["some_other_line=\"value\"".to_string()];
        configurator.update_or_add_line(&mut lines, "ZSH_THEME", "ZSH_THEME=\"new_theme\"");
        assert_eq!(
            lines,
            vec!["ZSH_THEME=\"new_theme\"", "some_other_line=\"value\""]
        );
    }

    #[test]
    fn test_update_or_add_line_ignores_commented_line() {
        let configurator = new_zsh_configurator();
        let mut lines = vec![
            "# ZSH_THEME=\"old_theme\"".to_string(),
            "other_line=\"value\"".to_string(),
        ];
        configurator.update_or_add_line(&mut lines, "ZSH_THEME", "ZSH_THEME=\"new_theme\"");
        assert_eq!(
            lines,
            vec![
                "# ZSH_THEME=\"old_theme\"",
                "ZSH_THEME=\"new_theme\"",
                "other_line=\"value\""
            ]
        );
    }

    #[test]
    fn test_modify_zshrc_appends_functions_once() {
        let configurator = new_zsh_configurator();
        let initial = "ZSH_THEME=\"robbyrussell\"\nplugins=(git)\n";
        let once = configurator.modify_zshrc_content(
            initial,
            configurator.theme,
            configurator.plugins,
            configurator.env_vars,
            configurator.functions,
        );
        assert!(once.contains(KC_FUNCTION));
        assert!(once.contains(KC_SET_FUNCTION));
        assert!(once.contains("DISABLE_UNTRACKED_FILES_DIRTY=\"true\""));

        let twice = configurator.modify_zshrc_content(
            &once,
            configurator.theme,
            configurator.plugins,
            configurator.env_vars,
            configurator.functions,
        );
        assert_eq!(once, twice);
    }
}
