#![allow(dead_code)]
#![allow(unused_must_use)]

use std::io;
use std::io::Write;
use std::process;

fn print_prompt() {
    print!("🦠 > ");
    io::stdout().flush();
}

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn do_meta_command(input: &str) -> MetaCommandResult {
    if input.trim() == ".exit" {
        println!("Exiting.");
        process::exit(0);
    }
    MetaCommandResult::MetaCommandUnrecognizedCommand
}

enum StatementType {
    InsertStatement,
    SelectStatement,
    UnrecognizedStatement,
}

struct Statement {
    type_: StatementType,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

fn prepare_statement(input: &str, mut statement: &mut Statement) -> PrepareResult {
    if input.starts_with("insert") {
        statement.type_ = StatementType::InsertStatement;
        return PrepareResult::PrepareSuccess;
    } else if input.starts_with("select") {
        statement.type_ = StatementType::SelectStatement;
        return PrepareResult::PrepareSuccess;
    } else {
        return PrepareResult::PrepareUnrecognizedStatement;
    }
}

fn execute_statement(statement: Statement) {
    match statement.type_ {
        StatementType::InsertStatement => println!("This is where we insert."),
        StatementType::SelectStatement => println!("This is where we select."),
        _ => println!("Panic..."),
    }
}

fn main() -> io::Result<()> {
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
            match do_meta_command(input) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command: {}", buffer.trim());
                    continue;
                }
            }
        }

        // Process statements
        let mut statement = Statement {
            type_: StatementType::UnrecognizedStatement,
        };
        match prepare_statement(input, &mut statement) {
            PrepareResult::PrepareSuccess => execute_statement(statement),
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}", buffer);
                continue;
            }
        }
    }
}
