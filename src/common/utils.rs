use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn replace_home_with_tilde(path: &Path) -> String {
    if let Some(home_dir) = env::var_os("HOME") {
        let home_path = Path::new(&home_dir);
        if let Ok(stripped_path) = path.strip_prefix(home_path) {
            return format!("~/{}", stripped_path.display());
        }
    }
    path.display().to_string()
}

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

pub fn get_hashset_delta<T: Eq + std::hash::Hash + Clone>(
    expected: &HashSet<T>,
    installed: &HashSet<T>,
) -> HashSet<T> {
    expected.difference(installed).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hashset_delta() {
        let expected = HashSet::from(["a", "b", "c"]);
        let installed = HashSet::from(["b", "d"]);
        let delta = get_hashset_delta(&expected, &installed);
        let expected_delta = HashSet::from(["a", "c"]);
        assert_eq!(delta, expected_delta);
    }
}
