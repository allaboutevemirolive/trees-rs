use std::{
    fs::Metadata,
    io::{self, Write},
};

use crate::canva::buffer::Buffer;

pub type WhichSize<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_size(&mut self, meta: &Metadata) -> io::Result<()> {
        let size = meta.len();
        self.write_space()?;

        let padded_string = format!("{:^12}", size.to_string());

        self.buf_writer.write_all("│".as_bytes())?;
        self.buf_writer.write_all("\x1B[1;32m".as_bytes())?;
        self.buf_writer.write_all(padded_string.as_bytes())?;
        self.buf_writer.write_all("\x1b[0m".as_bytes())?;
        self.buf_writer.write_all("│".as_bytes())?;

        self.write_space()?;
        Ok(())
    }

    pub fn write_no_size(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }

    pub fn paint_size(&mut self, meta: &Metadata, f: WhichSize<W>) -> io::Result<()> {
        f(self, meta)
    }
}
