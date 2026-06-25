use crate::common::run_command;

pub fn is_program_in_path(program: &str) -> bool {
    run_command("which", &[program]).is_ok()
}
