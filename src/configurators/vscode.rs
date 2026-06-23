use std::collections::HashSet;
use crate::common::Log;
use crate::configurators::Configurator;
use crate::detectors::VSCodeDetector;
use crate::detectors::app_detector::AppDetector;
use crate::symlinks::SetupResult;

/// Configurator to ensure some VS Code extensions are installed
pub struct VscodeConfigurator;

fn extensions() -> HashSet<&'static str> {
    HashSet::from([
        "github.copilot-chat",
        "ms-dotnettools.csdevkit",
        "ms-dotnettools.csharp",
        "ms-dotnettools.vscode-dotnet-runtime",
        "pflannery.vscode-versionlens",
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "felip3fdl.warm-burnout",
        "isudox.vscode-jetbrains-keybindings",
    ])
}

impl Configurator for VscodeConfigurator {
    fn name(&self) -> &'static str {
        "VSCode"
    }

    fn should_run(&self) -> bool {
        if !VSCodeDetector.is_installed() {
            return false;
        }
        let installed = installed_extensions().unwrap_or_default();
        let expected = extensions();
        expected.difference(&installed).next().is_some()
    }

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        if !VSCodeDetector.is_installed() {
            return Ok(());
        }

        let actual = installed_extensions().unwrap_or_default();
        let expected = extensions();
        let missed = expected.difference(&actual);

        for ext in missed {
            logger.info(&format!("Installing VS Code extension: {}", ext));
            match crate::common::run_command("code", &["--install-extension", ext]) {
                Ok(Some(_)) => logger.ok_with_highlight("Install extension ->", ext),
                Ok(None) => {
                    return Err(crate::common::SetupError::CommandFailed {
                        command: format!("code --install-extension {}", ext),
                        exit_code: None,
                    });
                }
                Err(e) => return Err(crate::common::SetupError::Io(e)),
            }
        }
        Ok(())
    }

    fn affected_files(&self) -> Vec<String> {
        // VS Code changes user extensions; no specific file path returned here
        Vec::new()
    }
}

fn installed_extensions() -> Option<HashSet<&'static str>> {
    match crate::common::run_command("code", &["--list-extensions"]) {
        Ok(Some(stdout)) => {
            let stdout_leak: &'static str = Box::leak(stdout.into_boxed_str());
            let set = stdout_leak
                .lines()
                .map(|l| l.trim())
                .collect::<HashSet<&'static str>>();
            Some(set)
        }
        _ => None,
    }
}
