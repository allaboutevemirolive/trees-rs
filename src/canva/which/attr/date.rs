use std::{
    fs::Metadata,
    io::{self, Write},
};

use crate::canva::buffer::{format_system_time, Buffer};

pub type WhichDate<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_date(&mut self, meta: &Metadata) -> io::Result<()> {
        let created = meta.created()?;
        let time = format_system_time(created);
        self.write_space()?;
        self.buf_writer.write_all(time.as_bytes())?;
        self.write_space()
    }

    pub fn write_no_date(&mut self, _meta: &Metadata) -> io::Result<()> {
        self.buf_writer.write_all("".as_bytes())
    }

    pub fn paint_date(&mut self, meta: &Metadata, f: WhichDate<W>) -> io::Result<()> {
        f(self, meta)
    }
}