use crate::page::Page;

pub const TABLE_MAX_PAGES: usize = 32;

#[derive(Default)]
pub struct Pager {
    pub file_descriptor: i32,
    pub file_length: usize,
    pub pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Pager {
    /// pager_open opens the database file, keeps track of its size, and initializes the page cache to all NULLs
    pub fn pager_open(filename: &str) -> Pager {
        let file_descriptor = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Error opening file");

        let file_length = std::fs::metadata(filename)
            .expect("Error getting file metadata")
            .len() as usize;

        let mut pages = [None; TABLE_MAX_PAGES];

        let num_pages = file_length / std::mem::size_of::<Page>();

        for i in 0..num_pages {
            let mut page = Page::default();
            let bytes_read = unsafe {
                libc::pread(
                    file_descriptor,
                    &mut page as *mut Page as *mut libc::c_void,
                    std::mem::size_of::<Page>(),
                    (i * std::mem::size_of::<Page>()) as libc::off_t,
                )
            };

            if bytes_read == -1 {
                panic!("Error reading file: {}", std::io::Error::last_os_error());
            }

            pages[i] = Some(page);
        }

        Pager {
            file_descriptor,
            file_length,
            pages,
        }
    }
}
