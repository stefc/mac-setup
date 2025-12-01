use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    // Get the output directory (where the binary will be built)
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find target directory");

    // Create config directory in target
    let config_dest = target_dir.join("config");
    fs::create_dir_all(&config_dest).expect("Failed to create config directory");

    // Helper: copy source -> dest only if source is newer than dest, or dest missing.
    
    // Copy .wezterm.lua to target/config/ only when newer
    let source = Path::new("config/.wezterm.lua");
    let dest = config_dest.join(".wezterm.lua");
    println!("cargo:rerun-if-changed=config/.wezterm.lua");
    match copy_if_newer(source, &dest) {
        Ok(true) => println!("cargo:warning=Copied .wezterm.lua to {}", dest.display()),
        Ok(false) => println!("cargo:warning=Skipped copying .wezterm.lua; destination is newer or source missing"),
        Err(e) => println!("cargo:warning=Error copying .wezterm.lua: {}", e),
    }

    // Copy stefc.zsh-theme to target/config/ only when newer
    let theme_source = Path::new("config/stefc.zsh-theme");
    let theme_dest = config_dest.join("stefc.zsh-theme");
    println!("cargo:rerun-if-changed=config/stefc.zsh-theme");
    match copy_if_newer(theme_source, &theme_dest) {
        Ok(true) => println!("cargo:warning=Copied stefc.zsh-theme to {}", theme_dest.display()),
        Ok(false) => println!("cargo:warning=Skipped copying stefc.zsh-theme; destination is newer or source missing"),
        Err(e) => println!("cargo:warning=Error copying stefc.zsh-theme: {}", e),
    }
}

fn copy_if_newer(source: &Path, dest: &Path) -> io::Result<bool> {
    if !source.exists() {
        return Ok(false);
    }

    let should_copy = if !dest.exists() {
        true
    } else {
        let src_meta = fs::metadata(source)?;
        let dst_meta = fs::metadata(dest)?;
        match (src_meta.modified(), dst_meta.modified()) {
            (Ok(s), Ok(d)) => s > d,
            // If we can't get modified times, default to copying to be safe
            _ => true,
        }
    };

    if should_copy {
        fs::copy(source, dest)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
