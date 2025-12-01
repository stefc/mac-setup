use std::env;
use std::fs;
use std::path::Path;

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

    // Copy .wezterm.lua to target/config/
    let source = Path::new("config/.wezterm.lua");
    let dest = config_dest.join(".wezterm.lua");
    
    fs::copy(source, &dest).expect("Failed to copy .wezterm.lua");
    
    println!("cargo:rerun-if-changed=config/.wezterm.lua");
    println!("cargo:warning=Copied .wezterm.lua to {}", dest.display());
}
