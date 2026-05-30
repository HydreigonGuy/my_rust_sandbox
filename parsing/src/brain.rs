use std::io;
use std::io::Write;
use crate::handler;

pub fn run_loop() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdouthandle = stdout.lock();
    let mut input = String::new();
    let mut active: bool = true;

    while active {
        stdouthandle.write_all(b"> ")?;
        stdouthandle.flush()?;
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "exit" => active = false,
            a => handler::handle_request(a.to_string())
        }
        input.clear();
    }
    Ok(())
}
