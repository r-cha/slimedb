use std::io;
use std::io::Write;

fn print_prompt() {
    print!("🦠 > ");
    io::stdout().flush();
}

fn main() -> io::Result<()> {
    // Greet user
    println!("Bootimg 🦠db.");

    // Prepare input buffer
    let mut buffer = String::new();
    let stdin = io::stdin();

    // Await input
    loop {
        print_prompt();
        stdin.read_line(&mut buffer)?;
        if buffer.trim() == ".exit" {
            println!("Exiting.");
            return Ok(());
        }
        println!("Unrecognized command: {}", buffer.trim());
        buffer = String::new();
    }
}
