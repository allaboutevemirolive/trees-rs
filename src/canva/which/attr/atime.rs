use crate::canva::buffer::Buffer;
use chrono::DateTime;
use chrono::Local;
use std::fs::Metadata;
use std::io;
use std::io::StdoutLock;
use std::io::Write;

pub type FnExtAccessTime<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

pub fn write_atime(buf: &mut Buffer<StdoutLock>, meta: &Metadata) -> io::Result<()> {
    if let Ok(created) = meta.accessed() {
        let time = format_system_time(created);
        buf.write_space()?;
        buf.buf_writer.write_all(time.as_bytes())?;
        buf.write_space()?;
    } else {
        buf.write_space()?;
        buf.buf_writer.write_all("─────".as_bytes())?;
        buf.write_space()?;
    }

    Ok(())
}

pub fn write_no_atime(buf: &mut Buffer<StdoutLock>, _meta: &Metadata) -> io::Result<()> {
    Ok(())
}

fn format_system_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%d-%m-%Y %H:%M").to_string()
}
