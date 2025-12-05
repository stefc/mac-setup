use std::env;
use std::path::Path;

/// Replaces the home directory part of a path with a tilde.
pub fn replace_home_with_tilde(path: &Path) -> String {
    if let Some(home_dir) = env::var_os("HOME") {
        let home_path = Path::new(&home_dir);
        if let Ok(stripped_path) = path.strip_prefix(home_path) {
            return format!("~/{}", stripped_path.display());
        }
    }
    path.display().to_string()
}
