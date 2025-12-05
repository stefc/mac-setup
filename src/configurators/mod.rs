pub mod zshrc;
pub mod yazi;

pub use zshrc::ZshrcConfigurator;
pub use yazi::YaziConfigurator;

use crate::{symlinks::SetupResult, common::Log};

/// Trait for configuration tasks
pub trait Configurator {
    /// Returns the name of the configurator for logging
    fn name(&self) -> &'static str;
    
    /// Check if this configurator should run
    fn should_run(&self) -> bool;
    
    /// Execute the configuration
    fn configure(&self) -> SetupResult<()>;

    /// Default run helper: checks `should_run()` and calls `configure()`.
    fn run(&self) -> SetupResult<()> {
        if !self.should_run() {
            println!("Skipping {}", self.name());
            return Ok(());
        }
        self.configure()
    }

    /// Return affected file paths for logging (tilde-expanded or user-friendly)
    fn affected_files(&self) -> Vec<String> { Vec::new() }
}

pub fn run_configurators(logger: &mut dyn Log) -> SetupResult<()> {
    logger.info("▶ Configuration");
    let configurators: Vec<Box<dyn Configurator>> = vec![
        Box::new(YaziConfigurator),
        Box::new(ZshrcConfigurator::default()),
    ];
    let mut affected = 0usize;
    for configurator in configurators {
        logger.info(&format!("Checking {}...", configurator.name()));
        // Use the centralized run helper which handles should_run() and skipping
        configurator.run()?;
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
