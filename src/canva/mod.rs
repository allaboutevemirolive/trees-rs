use std::io::{self, StdoutLock};

use crate::error::simple::TResult;

use self::buffer::Buffer;

pub mod ansi;
pub mod buffer;
mod mmap;
pub mod which;

#[derive(Debug)]
pub struct Canva<'cv> {
    pub buffer: Buffer<StdoutLock<'cv>>,
}

impl<'a> Canva<'a> {
    pub fn new() -> TResult<Self> {
        let stdout = io::stdout();
        let buffer = Buffer::new(stdout.lock())?;

        Ok(Self { buffer })
    }
}
