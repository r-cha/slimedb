#![allow(dead_code)]
#![allow(unused_must_use)]

use std::io;
use std::io::Write;

mod meta;

const ROWS_PER_PAGE: usize = 32;
const TABLE_MAX_PAGES: usize = 32;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Clone, Copy, Debug)]
struct Row {
    id: u32,
    username: [u8; 32],
    email: [u8; 255],
}

impl Default for Row {
    fn default() -> Self {
        Self {
            id: 0,
            username: [0; 32],
            email: [0; 255],
        }
    }
}

struct Page {
    rows: [Row; ROWS_PER_PAGE],
}

impl Default for Page {
    fn default() -> Self {
        Self {
            rows: [Row::default(); ROWS_PER_PAGE],
        }
    }
}

/// Table is a struct representing a Table in the database
#[derive(Default)]
struct Table {
    num_rows: usize,
    pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Table {
    /// row_slot returns a mutable pointer to the row at the given row_num
    fn row_slot<'a>(&'a mut self, row_num: usize) -> &'a mut Row {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;

        if self.pages[page_num].is_none() {
            self.pages[page_num] = Some(Page::default());
        }

        let page = self.pages[page_num].as_mut().unwrap();
        let row = &mut page.rows[row_offset];
        row
    }
}

enum StatementType {
    InsertStatement,
    SelectStatement,
    UnrecognizedStatement,
}

impl Default for StatementType {
    fn default() -> StatementType {
        Self::UnrecognizedStatement
    }
}

#[derive(Default)]
struct Statement {
    type_: StatementType,
    row_to_insert: Row,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

fn prepare_statement(input: &str, mut statement: &mut Statement) -> PrepareResult {
    let v: Vec<&str> = input.split(" ").collect();

    match &v[..] {
        ["insert", id, username, email] => {
            statement.type_ = StatementType::InsertStatement;
            statement.row_to_insert.id = id.parse().unwrap();

            let username_chars: Vec<u8> = username.bytes().collect();
            let email_chars: Vec<u8> = email.bytes().collect();

            statement.row_to_insert.username[..username_chars.len()]
                .copy_from_slice(&username_chars);
            statement.row_to_insert.email[..email_chars.len()].copy_from_slice(&email_chars);
        }
        ["select", ..] => statement.type_ = StatementType::SelectStatement,
        _ => return PrepareResult::PrepareUnrecognizedStatement,
    }

    PrepareResult::PrepareSuccess
}

enum ExecuteResult {
    ExecuteTableFull,
    ExecuteSuccess,
}

fn execute_insert(statement: Statement, table: &mut Table) -> ExecuteResult {
    use crate::ExecuteResult::*;

    if table.num_rows >= TABLE_MAX_ROWS {
        return ExecuteTableFull;
    }

    let row_to_insert: &Row = &statement.row_to_insert;
    let loc: &mut Row = table.row_slot(table.num_rows);

    *loc = *row_to_insert;
    table.num_rows += 1;

    ExecuteSuccess
}

fn execute_select(statement: Statement, table: &mut Table) {
    let nrows = table.num_rows;
    for n in 0..nrows {
        let row = table.row_slot(n);
        println!("{:?}", row);
    }
    // a noop for now to use `statement`
    statement.type_;
}

fn execute_statement(statement: Statement, table: &mut Table) {
    match statement.type_ {
        StatementType::InsertStatement => {
            println!("This is where we insert.");
            execute_insert(statement, table);
        }
        StatementType::SelectStatement => {
            println!("This is where we select.");
            execute_select(statement, table);
        }
        _ => println!("Panic..."),
    }
}

fn main() -> io::Result<()> {
    // Build the base table really quick
    // TODO: let statements specify a table?
    let mut table = Table::default();

    // Greet user
    println!("Booting 🦠db.");
    fn print_prompt() {
        print!("🦠 > ");
        io::stdout().flush();
    }

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
            PrepareResult::PrepareSuccess => {
                execute_statement(statement, &mut table);
                println!("Executed.")
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
        }
    }
}
