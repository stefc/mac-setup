use crate::common::Log;
use crate::configurators::Configurator;
use crate::detectors::VSCodeDetector;
use crate::detectors::app_detector::AppDetector;
use crate::symlinks::SetupResult;
use std::cell::OnceCell;
use std::collections::HashSet;

/// Configurator to ensure some VS Code extensions are installed
#[derive(Default)]
pub struct VscodeConfigurator {
    installed_cache: OnceCell<Option<HashSet<String>>>,
}

fn extensions() -> HashSet<String> {
    [
        "github.copilot-chat",
        "ms-dotnettools.csdevkit",
        "ms-dotnettools.csharp",
        "ms-dotnettools.vscode-dotnet-runtime",
        "pflannery.vscode-versionlens",
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb",
        "felip3fdl.warm-burnout",
        "isudox.vscode-jetbrains-keybindings",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

impl Configurator for VscodeConfigurator {
    fn name(&self) -> &'static str {
        "VSCode"
    }

    fn should_run(&self) -> bool {
        if !VSCodeDetector.is_installed() {
            return false;
        }
        let installed = self.installed_extensions().unwrap_or_default();
        let expected = extensions();
        expected.difference(&installed).count() > 0
    }

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        if !VSCodeDetector.is_installed() {
            return Ok(());
        }

        let actual = self.installed_extensions().unwrap_or_default();
        let expected = extensions();

        for ext in expected.difference(&actual) {
            logger.info(&format!("Installing VS Code extension: {}", ext));
            crate::common::run_command("code", &["--install-extension", ext])?;
            logger.ok_with_highlight("Install extension ->", ext);
        }
        Ok(())
    }

    fn affected_files(&self) -> Vec<String> {
        // VS Code changes user extensions; no specific file path returned here
        Vec::new()
    }
}

impl VscodeConfigurator {
    fn installed_extensions(&self) -> Option<HashSet<String>> {
        self.installed_cache
            .get_or_init(|| {
                match crate::common::run_command("code", &["--list-extensions"]) {
                    Ok(stdout) => {
                        let set = stdout
                            .lines()
                            .map(|l| l.trim().to_string())
                            .collect::<HashSet<String>>();
                        Some(set)
                    }
                    _ => None,
                }
            })
            .clone()
    }
}
