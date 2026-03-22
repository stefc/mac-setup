pub mod zshrc;
pub mod yazi;
pub mod vscode;

pub use zshrc::ZshrcConfigurator;
pub use yazi::YaziConfigurator;
pub use vscode::VscodeConfigurator;

use crate::{symlinks::SetupResult, common::Log};

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

    fn affected_files(&self) -> Vec<String> { Vec::new() }
}

pub fn run_configurators(logger: &mut dyn Log) -> SetupResult<()> {
    logger.info("▶ Configuration");
    let configurators: Vec<Box<dyn Configurator>> = vec![
        Box::new(YaziConfigurator),
        Box::new(VscodeConfigurator),
        Box::new(ZshrcConfigurator::default()),
    ];
    let mut affected = 0usize;
    for configurator in configurators {

        configurator.run(logger)?;
        let files = configurator.affected_files();
        for file in files {
            logger.ok_with_highlight("Configured successfully ->", &file);
        }
        if configurator.should_run() {
            affected += 1;
        }
    }

    logger.add_group("Configurators", affected);

    Ok(())
}
