use crate::row::Row;

pub const ROWS_PER_PAGE: usize = 32;

pub struct Page {
    pub rows: [Row; ROWS_PER_PAGE],
}

impl Default for Page {
    fn default() -> Self {
        Self {
            rows: [Row::default(); ROWS_PER_PAGE],
        }
    }
}
