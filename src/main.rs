// no local io usage

mod common;
mod configurators;
mod detectors;
mod environment;
mod logging;
mod settings;
mod symlinks;
use logging::{render_ui, Log, MemoryLogger};
use settings::{apply_system_settings, Platform};
use symlinks::{setup, SetupResult};

fn main() {
    let mut logger = MemoryLogger::default();

    let res = execute(&mut logger);

    // Render a simple Ratatui UI summarizing the outcome
    let snapshot = logger.snapshot();
    if let Err(e) = render_ui(&snapshot, res.as_ref().err().map(|e| e.to_string())) {
        eprintln!("Failed to render UI: {}", e);
    }

    // Also print a plain-text summary to stdout for non-TUI contexts
    if snapshot.groups.is_empty() {
        // Fallback: no groups recorded
        println!("Summary: no changes");
    } else {
        let parts: Vec<String> = snapshot
            .groups
            .iter()
            .map(|g| format!("{}: {}", g.title, g.affected_count))
            .collect();
        println!("Summary — {}", parts.join(" · "));
    }

    if let Err(e) = res {
        eprintln!("Setup failed: {}", e);
        std::process::exit(1);
    }
}

fn execute(logger: &mut dyn Log) -> SetupResult<()> {
    let platform = Platform::detect();
    environment::log_environment_info(logger, &platform);

    // Apply platform-specific system settings
    apply_system_settings(logger, platform)?;

    configurators::run_configurators(logger)?;
    setup::setup_symlinks(logger)?;

    Ok(())
}

