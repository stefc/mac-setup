pub mod vscode;
pub mod yazi;
pub mod zshrc;

use crate::{common::Log, symlinks::SetupResult};
pub use vscode::VscodeConfigurator;
pub use yazi::YaziConfigurator;
pub use zshrc::ZshrcConfigurator;

pub trait Configurator {
    fn name(&self) -> &'static str;

    fn should_run(&self) -> bool;

    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()>;

    fn run(&self, logger: &mut dyn Log) -> SetupResult<()> {
        if !self.should_run() {
            logger.info(&format!("Skipping {}...", self.name()));
            return Ok(());
        }
        self.configure(logger)
    }

    fn affected_files(&self) -> Vec<String> {
        Vec::new()
    }
}

pub fn run_configurators(logger: &mut dyn Log) -> SetupResult<()> {
    logger.info("▶ Configuration");
    let yazi = YaziConfigurator::default();
    let vscode = VscodeConfigurator::default();
    let zshrc = ZshrcConfigurator::default();
    let configurators: [&dyn Configurator; 3] = [&yazi, &vscode, &zshrc];
    let mut affected = 0usize;
    for configurator in configurators {
        let needs_run = configurator.should_run();
        configurator.run(logger)?;
        if needs_run {
            let files = configurator.affected_files();
            for file in files {
                logger.ok_with_highlight("Configured successfully ->", &file);
            }
            affected += 1;
        }
    }

    logger.add_group("Configurators", affected);

    Ok(())
}
