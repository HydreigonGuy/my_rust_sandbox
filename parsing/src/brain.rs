use std::io;
use std::io::Write;

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
            a => println!("{}", a)
        }
        input.clear();
    }
    Ok(())
}
