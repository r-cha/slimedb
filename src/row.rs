use std::fmt;

#[derive(Clone, Copy)]
pub struct Row {
    pub id: u32,
    pub username: [u8; 32],
    pub email: [u8; 255],
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let username = std::str::from_utf8(&self.username)
            .unwrap_or("<invalid utf8>")
            .trim_end_matches('\0');
        let email = std::str::from_utf8(&self.email)
            .unwrap_or("<invalid utf8>")
            .trim_end_matches('\0');

        f.debug_struct("Row")
            .field("id", &self.id)
            .field("username", &username)
            .field("email", &email)
            .finish()
    }
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
