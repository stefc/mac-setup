use std::env;
use std::path::Path;
use std::process::Command;

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

/// Run a command and return Ok(Some(stdout)) on success, Ok(None) if the
/// command executed but returned non-success exit status, or Err on IO error.
///
/// `program` is the executable name and `args` are its arguments.
pub fn run_command(program: &str, args: &[&str]) -> Result<Option<String>, std::io::Error> {
    let mut cmd = Command::new(program);
    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(Some(stdout))
    } else {
        Ok(None)
    }
}
