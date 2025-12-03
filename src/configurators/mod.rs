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

    /// Return affected file paths for logging (tilde-expanded or user-friendly)
    fn affected_files(&self) -> Vec<String> { Vec::new() }
}
