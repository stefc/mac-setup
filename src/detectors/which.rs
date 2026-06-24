use crate::common::run_command;

pub fn is_program_in_path(program: &str) -> bool {
    match run_command("which", &[program]) {
        Ok(_) => true,
        Err(_) => false,
    }
}