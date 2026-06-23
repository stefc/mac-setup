// no local io usage

mod common;
mod configurators;
mod detectors;
mod environment;
mod settings;
mod symlinks;
use common::{Log, MemoryLogger, Platform, render_ui};
use settings::apply_system_settings;
use symlinks::{SetupResult, setup};

fn main() {
    let mut logger = MemoryLogger::default();

    let res = execute(&mut logger);

    // Render a simple Ratatui UI summarizing the outcome
    let snapshot = logger.snapshot();
    if let Err(e) = render_ui(&snapshot, res.as_ref().err().map(|e| e.to_string())) {
        eprintln!("Failed to render UI: {}", e);
        std::process::exit(1);
    }
}

fn execute(logger: &mut dyn Log) -> SetupResult<()> {
    let platform = Platform::detect();
    environment::log_environment_info(logger, &platform);

    // Apply platform-specific system settings
    apply_system_settings(logger, &platform)?;

    configurators::run_configurators(logger)?;
    setup::setup_symlinks(logger)?;

    Ok(())
}
