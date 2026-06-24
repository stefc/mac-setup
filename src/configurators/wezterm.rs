use crate::common::Log;
use crate::configurators::Configurator;
use crate::detectors::app_detector::AppDetector;
use crate::detectors::wezterm::WezTermDetector;
use crate::symlinks::SetupResult;

pub struct WeztermConfigurator;

impl Configurator for WeztermConfigurator {
    fn name(&self) -> &'static str {
        "Wezterm"
    }

    fn should_run(&self) -> bool {
        WezTermDetector.is_installed()
    }

    fn configure(&self, _: &mut dyn Log) -> SetupResult<()> {
        Ok(())
    }

    fn affected_files(&self) -> Vec<String> {
        Vec::new()
    }
}
