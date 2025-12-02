use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// SOLID reminders for this build script:
//
// 1. Single Responsibility Principle (SRP):
//    - `main` handles the script's execution lifecycle and top-level errors.
//    - `run` orchestrates the high-level steps of the build process.
//    - `find_target_dir` is solely responsible for locating the `target` directory.
//    - `prepare_assets` is responsible for managing all asset-related tasks.
//    - `copy_if_newer` has the single job of conditionally copying a file.
//
// 2. Open/Closed Principle (OCP):
//    - `prepare_assets` is open to extension: new assets can be added to the `ASSETS`
//      array without modifying the logic of the functions themselves.
//
// By adhering to these principles, the script becomes more modular, easier to
// understand, and safer to change.

/// Main entry point for the build script.
fn main() {
    if let Err(err) = run() {
        // Emit a Cargo warning for visibility and exit with a non-zero status code.
        println!("cargo:warning=build.rs failed: {}", err);
        std::process::exit(1);
    }
}

/// Orchestrates the build process by calling specialized functions for each task.
fn run() -> io::Result<()> {
    let target_dir = find_target_dir()?;
    prepare_assets(&target_dir)?;
    Ok(())
}

/// Finds the Cargo `target` directory path from the `OUT_DIR` environment variable.
/// Cargo sets `OUT_DIR` to a path like `.../target/debug/build/<pkg-name>-<hash>/out`.
fn find_target_dir() -> io::Result<PathBuf> {
    let out_dir = env::var("OUT_DIR")
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "OUT_DIR environment variable not set"))?;

    Path::new(&out_dir)
        .ancestors()
        .nth(3) // Traverse up three levels: out -> build -> profile -> target
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to determine target directory from OUT_DIR"))
}

/// Prepares runtime configuration assets.
/// It creates the destination directory and copies each asset.
fn prepare_assets(target_dir: &Path) -> io::Result<()> {
    // Define the destination directory for config files within the `target` folder.
    let config_dest = target_dir.join("config");
    fs::create_dir_all(&config_dest)?;

    // List of assets to be copied from `config/` in the project root.
    const ASSETS: &[&str] = &[".wezterm.lua", "stefc.zsh-theme"];

    for &asset_name in ASSETS {
        let source_path = Path::new("config").join(asset_name);
        let dest_path = config_dest.join(asset_name);

        // Instruct Cargo to re-run this script if the source asset changes.
        println!("cargo:rerun-if-changed=config/{}", asset_name);

        // Perform the copy and provide feedback.
        let copied = copy_if_newer(&source_path, &dest_path)
            .expect(&format!("Failed to check or copy asset {}", asset_name));

        if copied {
            println!("cargo:warning=Copied {} to {}", asset_name, dest_path.display());
        } else {
            println!(
                "cargo:warning=Skipped copying {}; destination is newer or source is missing",
                asset_name
            );
        }
    }

    Ok(())
}

/// Copies a file from `source` to `dest` only if the source is newer than the destination.
///
/// Returns `Ok(true)` if the file was copied, `Ok(false)` if skipped, and `Err` on failure.
fn copy_if_newer(source: &Path, dest: &Path) -> io::Result<bool> {
    if !source.exists() {
        return Ok(false); // Nothing to do if the source doesn't exist.
    }

    // Check if a copy is necessary.
    let should_copy = if !dest.exists() {
        true // Copy if destination doesn't exist.
    } else {
        // Copy if source modification time is newer than destination.
        let src_meta = fs::metadata(source)?;
        let dest_meta = fs::metadata(dest)?;
        let src_time = src_meta
            .modified()
            .expect("Failed to get modification time for source");
        let dest_time = dest_meta
            .modified()
            .expect("Failed to get modification time for destination");
        src_time > dest_time
    };

    if should_copy {
        // Ensure the parent directory of the destination exists.
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, dest)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
