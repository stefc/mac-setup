use crate::common::Log;
use crate::configurators::Configurator;
use crate::symlinks::SetupResult;
use crate::detectors::VSCodeDetector;
use crate::detectors::app_detector::AppDetector;

/// Configurator to ensure some VS Code extensions are installed
pub struct VscodeConfigurator;

impl VscodeConfigurator {
    fn is_installed(&self) -> bool {
        VSCodeDetector.is_installed()
    }

    /// Get currently installed VS Code extensions (cached from a single CLI call)
    fn installed_extensions(&self) -> Option<std::collections::HashSet<String>> {
        match crate::common::run_command("code", &["--list-extensions"]) {
            Ok(Some(stdout)) => {
                let set = stdout
                    .lines()
                    .map(|l| l.trim().to_string())
                    .collect::<std::collections::HashSet<String>>();
                Some(set)
            }
            _ => None,
        }
    }

    /// List of extensions to ensure installed. Update this list as needed.
    fn extensions(&self) -> Vec<&'static str> {
        vec![
            "github.copilot-chat",
            "ms-dotnettools.csdevkit",
            "ms-dotnettools.csharp",
            "ms-dotnettools.vscode-dotnet-runtime",
            "pflannery.vscode-versionlens",
            "rust-lang.rust-analyzer",
            "vadimcn.vscode-lldb"
        ]
    }

    /// Check whether an extension is already installed using a pre-fetched set
    fn is_extension_installed_in_set(&self, set: &std::collections::HashSet<String>, ext: &str) -> bool {
        set.contains(ext)
    }

    fn run_configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        // Ensure `code` CLI is available. If not, we try to proceed but will error when executing.
        if !self.is_installed() {
            return Ok(());
        }

        // Buffer installed extensions to avoid repeated CLI calls
        let installed = self.installed_extensions().unwrap_or_default();

        for ext in self.extensions() {
            if self.is_extension_installed_in_set(&installed, ext) {
                logger.info(&format!("Extension already installed: {}", ext));
                continue;
            }

            logger.info(&format!("Installing VS Code extension: {}", ext));
            match crate::common::run_command("code", &["--install-extension", ext]) {
                Ok(Some(_)) =>  logger.ok_with_highlight("Install extension ->", ext),
                Ok(None) => {
                    return Err(crate::common::SetupError::CommandFailed { command: format!("code --install-extension {}", ext), exit_code: None });
                }
                Err(e) => return Err(crate::common::SetupError::Io(e)),
            }
        }

        Ok(())
    }
}

impl Configurator for VscodeConfigurator {
    fn name(&self) -> &'static str {
        "VSCode"
    }

    fn should_run(&self) -> bool {
        if !self.is_installed() {
            return false;
        }
        // Run if at least one extension is missing, using a single buffered list
        let installed = self.installed_extensions().unwrap_or_default();
        self.extensions().into_iter().any(|e| !self.is_extension_installed_in_set(&installed, e))
    }

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        self.run_configure(logger)
    }

    fn affected_files(&self) -> Vec<String> {
        // VS Code changes user extensions; no specific file path returned here
        Vec::new()
    }
}
