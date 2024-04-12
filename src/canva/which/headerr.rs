use std::ffi::OsString;
use std::fs::Metadata;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::canva::buffer::Buffer;

pub type WhichHeader<W> = fn(&mut Buffer<W>, &Metadata, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_header_relative_path(
        &mut self,
        _meta: &Metadata,
        _root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let mut path = PathBuf::new();
        path.push(parent);

        let path = path.to_owned().into_os_string();
        self.buf_writer.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn write_header_name(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.buf_writer
            .write_all(root.file_name().unwrap().as_encoded_bytes())?;
        Ok(())
    }

    pub fn paint_header(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
        f: WhichHeader<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}
