use std::env;

/// Replace occurrences of the user's home directory with a tilde for nicer output.
pub fn replace_home_with_tilde(path_str: String) -> String {
    if let Some(home_dir) = env::var_os("HOME") {
        if let Some(home_str) = home_dir.to_str() {
            return path_str.replace(home_str, "~");
        }
    }
    path_str
}
