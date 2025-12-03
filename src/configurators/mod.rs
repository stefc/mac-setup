pub mod zshrc;
pub mod yazi;

pub use zshrc::ZshrcConfigurator;
pub use yazi::YaziConfigurator;

use crate::symlinks::SetupResult;

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
