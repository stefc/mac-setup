pub mod zshrc;
pub mod yazi;
pub mod vscode;

pub use zshrc::ZshrcConfigurator;
pub use yazi::YaziConfigurator;
pub use vscode::VscodeConfigurator;

use crate::{symlinks::SetupResult, common::Log};

/// Trait for configuration tasks
pub trait Configurator {
    /// Returns the name of the configurator for logging
    fn name(&self) -> &'static str;
    
    /// Check if this configurator should run
    fn should_run(&self) -> bool;
    
    /// Execute the configuration
    fn configure(&self, logger: &mut dyn Log) -> SetupResult<()>;

    /// Default run helper: checks `should_run()` and calls `configure()`.
    fn run(&self, logger: &mut dyn Log) -> SetupResult<()> {
        if !self.should_run() {
            logger.info(&format!("Skipping {}...", self.name()));
            return Ok(());
        }
        self.configure(logger)
    }

    /// Return affected file paths for logging (tilde-expanded or user-friendly)
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
        
        // Use the centralized run helper which handles should_run() and skipping
        configurator.run(logger)?;
        let files = configurator.affected_files();
        for file in files {
            logger.ok_with_highlight("Configured successfully ->", &file);
        }
        // Count only those that actually ran; `should_run()` is checked inside `run()`
        if configurator.should_run() {
            affected += 1;
        }
    }

    logger.add_group("Configurators", affected);

    Ok(())
}
