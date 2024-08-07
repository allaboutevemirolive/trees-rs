use crate::render::buffer::Buffer;
use chrono::DateTime;
use chrono::Local;
use std::fs::Metadata;
use std::io;
use std::io::Write;

pub type FnExtAccessTime<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    /// Print entry's access-time
    pub fn print_atime(&mut self, meta: &Metadata, f: FnExtAccessTime<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn write_atime(&mut self, meta: &Metadata) -> io::Result<()> {
        if let Ok(created) = meta.accessed() {
            let time = format_system_time(created);
            self.write_space()?;
            self.bufwr.write_all(time.as_bytes())?;
            self.write_space()?;
        } else {
            self.write_space()?;
            self.bufwr.write_all("─────".as_bytes())?;
            self.write_space()?;
        }

        Ok(())
    }

    pub fn write_no_atime(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }
}

// TODO:
fn format_system_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%d-%m-%Y %H:%M").to_string()
}
