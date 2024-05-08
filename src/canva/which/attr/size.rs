use crate::canva::buffer::Buffer;
use std::fs::Metadata;
use std::io::Write;
use std::io::{self, StdoutLock};

pub type FnExtSize<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

pub fn write_no_size(buf: &mut Buffer<StdoutLock>, _meta: &Metadata) -> io::Result<()> {
    Ok(())
}

pub fn write_size(buf: &mut Buffer<StdoutLock>, meta: &Metadata) -> io::Result<()> {
    let size = meta.len();
    // Only 931.32 gigabytes, or 999999999999 bytes, can be supported at most by the padding.
    // If the size is exceeded, the tree output will be distorted and not symmetrical.
    let padded_string = format!("{:^12}", size.to_string());

    buf.buf_writer.write_all(padded_string.as_bytes())?;

    Ok(())
}

pub fn write_size_color(buf: &mut Buffer<StdoutLock>, meta: &Metadata) -> io::Result<()> {
    let size = meta.len();
    // Only 931.32 gigabytes, or 999999999999 bytes, can be supported at most by the padding.
    // If the size is exceeded, the tree output will be distorted and not symmetrical.
    let padded_string = format!("{:^12}", size.to_string());

    buf.buf_writer.write_all("\x1B[1;32m".as_bytes())?;
    buf.buf_writer.write_all(padded_string.as_bytes())?;
    buf.buf_writer.write_all("\x1b[0m".as_bytes())?;

    Ok(())
}

impl<W: Write> Buffer<W> {
    // Print entry's size
    // pub fn paint_size(&mut self, meta: &Metadata, f: FnExtSize<W>) -> io::Result<()> {
    //     f(self, meta)
    // }

    // pub fn write_no_size(&mut self, _meta: &Metadata) -> io::Result<()> {
    //     Ok(())
    // }

    // pub fn write_size(&mut self, meta: &Metadata) -> io::Result<()> {
    //     let size = meta.len();
    //     // Only 931.32 gigabytes, or 999999999999 bytes, can be supported at most by the padding.
    //     // If the size is exceeded, the tree output will be distorted and not symmetrical.
    //     let padded_string = format!("{:^12}", size.to_string());

    //     self.buf_writer.write_all(padded_string.as_bytes())?;

    //     Ok(())
    // }

    // pub fn write_size_color(&mut self, meta: &Metadata) -> io::Result<()> {
    //     let size = meta.len();
    //     // Only 931.32 gigabytes, or 999999999999 bytes, can be supported at most by the padding.
    //     // If the size is exceeded, the tree output will be distorted and not symmetrical.
    //     let padded_string = format!("{:^12}", size.to_string());

    //     self.buf_writer.write_all("\x1B[1;32m".as_bytes())?;
    //     self.buf_writer.write_all(padded_string.as_bytes())?;
    //     self.buf_writer.write_all("\x1b[0m".as_bytes())?;

    //     Ok(())
    // }
}
