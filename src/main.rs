#![allow(dead_code)]
#![allow(unused_must_use)]

use std::io;
use std::io::Write;

mod execute;
mod meta;
mod page;
mod row;
mod statement;
mod table;

use crate::execute::{execute_statement, ExecuteResult};
use crate::statement::{prepare_statement, PrepareResult, Statement};
use crate::table::Table;

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

        // Process metacommands
        let input = buffer.trim();
        if buffer.as_bytes()[0] == b'.' {
            match meta::do_command(input) {
                meta::CommandResult::Success => continue,
                meta::CommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command: {}", input);
                    continue;
                }
            }
        }

        // Process statements
        let mut statement = Statement::default();
        match prepare_statement(input, &mut statement) {
            PrepareResult::PrepareSuccess => match execute_statement(statement, &mut table) {
                ExecuteResult::ExecuteSuccess => println!("Executed."),
                ExecuteResult::ExecuteTableFull => println!("Error: Table full."),
                ExecuteResult::ExecuteFailed => println!("Error: Execute failed."),
            },
            PrepareResult::PrepareSyntaxError => {
                println!("Syntax error. Could not parse statement.")
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
        }
    }
}
