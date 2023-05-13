use crate::row::Row;
use crate::statement::{Statement, StatementType};
use crate::table::{Table, TABLE_MAX_ROWS};

pub enum ExecuteResult {
    ExecuteTableFull,
    ExecuteSuccess,
    ExecuteFailed,
}

fn execute_insert(statement: Statement, table: &mut Table) -> ExecuteResult {
    if table.num_rows >= TABLE_MAX_ROWS {
        return ExecuteResult::ExecuteTableFull;
    }

    let row_to_insert: &Row = &statement.row_to_insert;
    let loc: &mut Row = table.row_slot(table.num_rows);

    *loc = *row_to_insert;
    table.num_rows += 1;

    ExecuteResult::ExecuteSuccess
}

fn execute_select(statement: Statement, table: &mut Table) -> ExecuteResult {
    let nrows = table.num_rows;
    for n in 0..nrows {
        let row = table.row_slot(n);
        println!("{:?}", row);
    }
    // a noop for now to use `statement`
    statement.type_;

    ExecuteResult::ExecuteSuccess
}

pub fn execute_statement(statement: Statement, table: &mut Table) -> ExecuteResult {
    match statement.type_ {
        StatementType::InsertStatement => {
            return execute_insert(statement, table);
        }
        StatementType::SelectStatement => {
            return execute_select(statement, table);
        }
        StatementType::UnrecognizedStatement => return ExecuteResult::ExecuteFailed,
    }
}
