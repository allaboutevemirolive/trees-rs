use crate::canva::buffer::Buffer;
use chrono::DateTime;
use chrono::Local;
use std::fs::Metadata;
use std::io;
use std::io::Write;

pub type WhichModificationTime<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn paint_mtime(&mut self, meta: &Metadata, f: WhichModificationTime<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn write_mtime(&mut self, meta: &Metadata) -> io::Result<()> {
        if let Ok(created) = meta.modified() {
            let time = format_system_time(created);
            self.write_space()?;
            self.buf_writer.write_all(time.as_bytes())?;
            self.write_space()?;
        } else {
            self.write_space()?;
            self.buf_writer.write_all("─────".as_bytes())?;
            self.write_space()?;
        }

        Ok(())
    }

    pub fn write_no_mtime(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }
}

fn format_system_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%e %b %Y %H:%M").to_string()
}
