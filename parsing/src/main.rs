
mod handler;
mod brain;
use std::process::ExitCode;

fn main() -> ExitCode {
    match brain::run_loop() {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::from(1)
    }
}
