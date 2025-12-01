use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    print_current_working_directory();
    print_executable_directory();

    // create config for wezterm
    if is_wezterm_app_installed() {
        println!("WezTerm is installed, creating symlink for wezterm config...");
        create_wezterm_symlink_shell();
    } else {
        println!("WezTerm is not installed, skipping wezterm config symlink creation.");
        return;
    }
}

fn replace_home_with_tilde(path_str: String) -> String {
    if let Some(home_dir) = env::var_os("HOME") {
        if let Some(home_str) = home_dir.to_str() {
            return path_str.replace(home_str, "~");
        }
    }
    path_str
}

fn print_current_working_directory() {
    match env::current_dir() {
        Ok(path) => {
            let path_str = replace_home_with_tilde(path.display().to_string());
            println!("Current working directory: {}", path_str);
        },
        Err(e) => eprintln!("Failed to get current working directory: {}", e),
    }
}

fn print_executable_directory() {
    match env::current_exe() {
        Ok(exe_path) => {
            if let Some(exe_dir) = exe_path.parent() {
                let path_str = replace_home_with_tilde(exe_dir.display().to_string());
                println!("Executable directory: {}", path_str);
            } else {
                eprintln!("Failed to get parent directory of executable");
            }
        },
        Err(e) => eprintln!("Failed to get executable path: {}", e),
    }
}

fn is_wezterm_app_installed() -> bool {
    Path::new("/Applications/WezTerm.app").exists()
}

fn create_wezterm_symlink_shell() {
    // Execute the shell command directly
    let command = "ln -fsv $(pwd)/config/.wezterm.lua ~/.wezterm.lua";
    
    println!("Executing: sh -c \"{}\"", command);
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    
    // Print stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        let stdout_str = replace_home_with_tilde(stdout.to_string());
        print!("{}", stdout_str);
    }
    
    // Print stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        let stderr_str = replace_home_with_tilde(stderr.to_string());
        eprint!("{}", stderr_str);
    }
    
    if output.status.success() {
        println!("Symlink created successfully");
    } else {
        eprintln!("Command failed with exit code: {:?}", output.status.code());
    }
}
