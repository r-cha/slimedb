use crate::page::{Page, ROWS_PER_PAGE};
use crate::row::Row;
use crate::pager::{Pager, TABLE_MAX_PAGES};

pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

/// Table is a struct representing a Table in the database
#[derive(Default)]
pub struct Table {
    pub num_rows: usize,
    pub pages: Pager,
}

impl Table {
    /// row_slot returns a mutable pointer to the row at the given row_num
    pub fn row_slot<'a>(&'a mut self, row_num: usize) -> &'a mut Row {
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
