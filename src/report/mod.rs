use crate::error::simple::UResult;

use self::{head::Head, tail::Tail};

mod head;
pub mod tail;

#[derive(Debug)]
pub struct Report {
    pub head: Head,
    pub tail: Tail,
}

impl Report {
    /// We set directories to 1 to include the current directory
    pub fn new() -> UResult<Self> {
        Ok(Self {
            head: Head::CurrentDir,
            tail: Tail {
                directories: 1,
                files: 0,
                size: 0,
                hidden_files: 0,
            },
        })
    }

    pub fn get_tail(&self) -> (String, String, String, String, String) {
        let bytes = self.tail.size;

        let gigabytes = bytes as f64 / 1_073_741_824.0;

        (
            self.tail.directories.to_string(),
            self.tail.files.to_string(),
            // self.tail.size.to_string(),
            self.tail.hidden_files.to_string(),
            self.tail.size.to_string(),
            gigabytes.to_string(),
        )
    }
}
