use crate::row::Row;

pub enum StatementType {
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
pub struct Statement {
    pub type_: StatementType,
    pub row_to_insert: Row,
}

pub enum PrepareResult {
    PrepareSuccess,
    PrepareSyntaxError,
    PrepareUnrecognizedStatement,
}

pub fn prepare_statement(input: &str, mut statement: &mut Statement) -> PrepareResult {
    let v: Vec<&str> = input.split(" ").collect();

    match &v[..] {
        ["insert", id, username, email] => {
            statement.type_ = StatementType::InsertStatement;
            statement.row_to_insert.id = match id.parse() {
                Ok(n) => n,
                Err(_) => return PrepareResult::PrepareSyntaxError,
            };

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
