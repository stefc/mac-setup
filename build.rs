use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

fn main() {
    if let Err(err) = run() {
        println!("cargo:warning=build.rs failed: {}", err);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let target_dir = find_target_dir()?;
    prepare_assets(&target_dir)?;
    Ok(())
}

/// Finds the Cargo `target` directory path from the `OUT_DIR` environment variable.
/// Cargo sets `OUT_DIR` to a path like `.../target/debug/build/<pkg-name>-<hash>/out`.
fn find_target_dir() -> io::Result<PathBuf> {
    let out_dir = env::var("OUT_DIR").map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "OUT_DIR environment variable not set")
    })?;

    Path::new(&out_dir)
        .ancestors()
        .nth(3) // Traverse up three levels: out -> build -> profile -> target
        .map(PathBuf::from)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Failed to determine target directory from OUT_DIR",
            )
        })
}

/// Prepares runtime configuration assets.
/// It creates the destination directory and copies each asset.
fn prepare_assets(target_dir: &Path) -> io::Result<()> {
    // Define the destination directory for config files within the `target` folder.
    let config_dest = target_dir.join("config");
    fs::create_dir_all(&config_dest)?;

    // List of assets to be copied from `config/` in the project root.
    const ASSETS: &[&str] = &[
        ".wezterm.lua",
        "stefc.zsh-theme",
        "code.settings.json",
        "yazi.theme.toml",
        "helix.config.toml",
    ];

    for &asset_name in ASSETS {
        let source_path = Path::new("config").join(asset_name);
        let dest_path = config_dest.join(asset_name);

        // Instruct Cargo to re-run this script if the source asset changes.
        println!("cargo:rerun-if-changed=config/{}", asset_name);

        // Perform the copy and provide feedback.
        let _copied = copy_if_newer(&source_path, &dest_path)
            .expect(&format!("Failed to check or copy asset {}", asset_name));
    }

    const HELIX_ASSETS: &[&str] = &["warm-burnout-dark.toml", "warm-burnout-light.toml"];
    for &asset_name in HELIX_ASSETS {
        let source_path = Path::new("config").join("helix-theme").join(asset_name);
        let dest_path = config_dest.join("helix-theme").join(asset_name);

        // Perform the copy and provide feedback.
        let _copied = copy_if_newer(&source_path, &dest_path)
            .expect(&format!("Failed to check or copy asset {}", asset_name));
    }
    const WEZTERM_ASSETS: &[&str] = &["warm-burnout-dark.toml", "warm-burnout-light.toml"];
    for &asset_name in WEZTERM_ASSETS {
        let source_path = Path::new("config").join("wezterm-theme").join(asset_name);
        let dest_path = config_dest.join("wezterm-theme").join(asset_name);

        // Perform the copy and provide feedback.
        let _copied = copy_if_newer(&source_path, &dest_path)
            .expect(&format!("Failed to check or copy asset {}", asset_name));
    }

    Ok(())
}

fn get_modified_time(path: &Path) -> io::Result<SystemTime> {
    Ok(fs::metadata(path)?.modified()?)
}

fn copy_if_newer(source: &PathBuf, dest: &PathBuf) -> io::Result<bool> {
    if !source.exists() {
        return Ok(false);
    }

    let should_copy = if !dest.exists() {
        true
    } else {
        let src_time =
            get_modified_time(source).expect("Failed to get modification time for source");

        let dest_time =
            get_modified_time(dest).expect("Failed to get modification time for destination");
        src_time > dest_time
    };

    if should_copy {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, dest)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
