use crate::render::buffer::Buffer;
use std::fs::Metadata;
use std::io;
use std::io::Write;

pub type FnExtSize<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    /// Print entry's size
    pub fn print_size(&mut self, meta: &Metadata, f: FnExtSize<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn write_no_size(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }

    pub fn write_size(&mut self, meta: &Metadata) -> io::Result<()> {
        let size = meta.len();
        // Only 931.32 gigabytes, or 999999999999 bytes, can be supported at most by the padding.
        // If the size is exceeded, the tree output will be distorted and not symmetrical.
        let padded_string = format!("{:^12}", size.to_string());

        self.bufwr.write_all(padded_string.as_bytes())?;

        Ok(())
    }
}
