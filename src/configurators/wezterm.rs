use crate::common::Log;
use crate::configurators::Configurator;
use crate::detectors::app_detector::AppDetector;
use crate::detectors::wezterm::WezTermDetector;
use crate::symlinks::SetupResult;

pub struct WeztermConfigurator;

impl WeztermConfigurator {
    fn is_installed(&self) -> bool {
        WezTermDetector.is_installed()
    }

    fn run_configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        // Ensure `code` CLI is available. If not, we try to proceed but will error when executing.
        if !self.is_installed() {
            return Ok(());
        }

        Ok(())
    }
}

impl Configurator for WeztermConfigurator {
    fn name(&self) -> &'static str {
        "Wezterm"
    }

    fn should_run(&self) -> bool {
        if !self.is_installed() {
            return false;
        }
        true
    }

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()> {
        self.run_configure(logger)
    }

    fn affected_files(&self) -> Vec<String> {
        // VS Code changes user extensions; no specific file path returned here
        Vec::new()
    }
}
