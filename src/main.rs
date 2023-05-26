#![allow(unused_must_use)]

use std::io;
use std::io::Write;

use slimedb::process;
use slimedb::table::Table;

fn print_prompt() {
    print!("🦠 > ");
    io::stdout().flush();
}

fn main() -> io::Result<()> {
    // Build the base table really quick
    let mut table = Table::default();

    // Greet user
    println!("Booting 🦠db.");

    // Prepare input buffer
    let mut buffer: String;
    let stdin = io::stdin();

    // Await input
    loop {
        print_prompt();
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        process(buffer, &mut table)
    }
}
