use std::io::{self, StdoutLock};

use self::buffer::Buffer;

pub mod ansi;
pub mod buffer;
mod mmap;

#[derive(Debug)]
pub struct Canva<'cv> {
    pub buffer: Buffer<StdoutLock<'cv>>,
}

impl<'a> Canva<'a> {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let buffer = Buffer::new(stdout.lock());

        Self { buffer }
    }

    // pub fn new_in_memory() -> Self {
    //     // let stdout = io::stdout();
    //     let buffer = Buffer::new(Vec::new());
    //     Self { buffer }
    // }
}
