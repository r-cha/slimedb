mod execute;
mod meta;
mod page;
mod row;
mod statement;
pub mod table;

use crate::execute::{execute_statement, ExecuteResult};
use crate::statement::{prepare_statement, PrepareResult, Statement};
use crate::table::Table;

// noop function
fn noop() {}

pub fn process(buffer: String, table: &mut Table) {
    // TODO: remove table reference, should be per-statement
    // Process metacommands
    let input = buffer.trim();
    if buffer.as_bytes()[0] == b'.' {
        match meta::do_command(input) {
            meta::CommandResult::Success => return,
            meta::CommandResult::UnrecognizedCommand => {
                println!("Unrecognized command: {}", input);
                return;
            }
        }
    }

    // Process statements
    let mut statement = Statement::default();
    match prepare_statement(input, &mut statement) {
        PrepareResult::PrepareSuccess => match execute_statement(statement, table) {
            ExecuteResult::ExecuteSuccess => noop(),  // TODO: Do something
            ExecuteResult::ExecuteTableFull => println!("Error: Table full."),
            ExecuteResult::ExecuteFailed => println!("Error: Execute failed."),
        },
        PrepareResult::PrepareSyntaxError => {
            println!("Syntax error. Could not parse statement.")
        }
        PrepareResult::PrepareUnrecognizedStatement => {
            println!("Unrecognized keyword at start of {}", input);
            return;
        }
    }
}
