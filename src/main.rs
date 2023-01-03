#![allow(dead_code)]
#![allow(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;

mod meta;

const USERNAME_LENGTH: usize = 32;
const EMAIL_LENGTH: usize = 32; // TODO: idk man, nothin works for 255

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    id: u32,
    username: [char; USERNAME_LENGTH],
    email: [char; EMAIL_LENGTH],
}

// src: https://stackoverflow.com/a/70222282
macro_rules! size_of_attribute {
    ($s:ident :: $attr:ident) => {{
        let m = core::mem::MaybeUninit::<$s>::uninit();
        // TODO: Understand this
        let p = unsafe { core::ptr::addr_of!((*(&m as *const _ as *const $s)).$attr) };

        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        size_of_raw(p)
    }};
}
const ID_SIZE: usize = size_of_attribute!(Row::id);
const USERNAME_SIZE: usize = size_of_attribute!(Row::username);
const EMAIL_SIZE: usize = size_of_attribute!(Row::email);
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

impl Default for Row {
    fn default() -> Row {
        Row {
            id: 0,
            username: ['0'; USERNAME_LENGTH],
            email: ['0'; EMAIL_LENGTH],
        }
    }
}

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 32; // Arbitrary
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;
#[derive(Default)]
struct Page {
    // TODO: I don't think this works the same as the C code because
    // `&rows` is the memory location, not `&pages`?
    // But for now it works and makes things easy.
    rows: [Row; ROWS_PER_PAGE],
}

#[derive(Default)]
struct Table {
    num_rows: usize,
    pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Table {
    fn row_slot(mut self, row_num: usize) -> *mut Row {
        let page_num = row_num / ROWS_PER_PAGE;
        let mut page = match self.pages[page_num] {
            Some(p) => p,
            None => {
                let p = Page::default();
                self.pages[page_num] = Some(p);
                p
            }
        };
        let row_offset = row_num % ROWS_PER_PAGE;
        page.rows[row_offset]
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
            statement.row_to_insert.username = username
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("username length"); // TODO: Safely unwrap/pad/clamp
            statement.row_to_insert.email = email
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("email length");
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

fn execute_insert(statement: Statement, table: Table) -> ExecuteResult {
    use crate::ExecuteResult::*;

    if table.num_rows >= TABLE_MAX_ROWS.try_into().unwrap() {
        return ExecuteTableFull;
    }

    let row_to_insert: *const Row = &(statement.row_to_insert);
    let loc: mut Row = table.row_slot(table.num_rows);

    row_to_insert.copy_to(loc, count);
    table.num_rows += 1;

    ExecuteSuccess
}

fn execute_select(statement: Statement, table: Table) {
    let mut row: Row;
    let nrows = table.num_rows;
    for n in 0..nrows {
        row = table.row_slot(n);
        println!("{}", row);
    }
}

fn execute_statement(statement: Statement, table: Table) {
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
                execute_statement(statement);
                println!("Executed.")
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
        }
    }
}
