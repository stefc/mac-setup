use crate::common::{Log, get_hashset_delta};
use crate::configurators::Configurator;
use crate::detectors::VSCodeDetector;
use crate::detectors::app_detector::AppDetector;
use crate::symlinks::SetupResult;
use std::collections::HashSet;

/// Configurator to ensure some VS Code extensions are installed
pub struct VscodeConfigurator;

fn extensions() -> HashSet<String> {
    HashSet::from([
        "github.copilot-chat".to_string(),
        "ms-dotnettools.csdevkit".to_string(),
        "ms-dotnettools.csharp".to_string(),
        "ms-dotnettools.vscode-dotnet-runtime".to_string(),
        "pflannery.vscode-versionlens".to_string(),
        "rust-lang.rust-analyzer".to_string(),
        "vadimcn.vscode-lldb".to_string(),
        "felip3fdl.warm-burnout".to_string(),
        "isudox.vscode-jetbrains-keybindings".to_string(),
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
        !get_hashset_delta(&expected, &installed).is_empty()
    }

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        if !VSCodeDetector.is_installed() {
            return Ok(());
        }

        let actual = installed_extensions().unwrap_or_default();
        let expected = extensions();
        let missed = get_hashset_delta(&expected, &actual);

        for ext in missed {
            logger.info(&format!("Installing VS Code extension: {}", ext));
            match crate::common::run_command("code", &["--install-extension", &ext]) {
                Ok(Some(_)) => logger.ok_with_highlight("Install extension ->", &ext),
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

fn installed_extensions() -> Option<HashSet<String>> {
    match crate::common::run_command("code", &["--list-extensions"]) {
        Ok(Some(stdout)) => {
            let set = stdout
                .lines()
                .map(|l| l.trim().to_string())
                .collect::<HashSet<String>>();
            Some(set)
        }
        _ => None,
    }
}
