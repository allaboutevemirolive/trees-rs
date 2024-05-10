use crate::canva::buffer::Buffer;
use crate::walk::metada::Visitor;
use std::ffi::OsString;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type FnOutFile<W> = fn(&mut Buffer<W>, &Visitor, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_entry_relative_path(
        &mut self,
        meta: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(root).unwrap();

        let mut path = PathBuf::new();
        path.push(parent);
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.buf_writer.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    pub fn write_color_entry_relative_path(
        &mut self,
        meta: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(root).unwrap();

        let mut path = PathBuf::new();
        path.push(parent);
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.buf_writer.write_all("\x1b[0;34m".as_bytes())?;
        self.buf_writer.write_all(path.as_encoded_bytes())?;
        self.buf_writer.write_all("\x1b[0m".as_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_entry(
        &mut self,
        meta: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.buf_writer.write_all(meta.name.as_encoded_bytes())?;
        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_entry_color(
        &mut self,
        meta: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.buf_writer.write_all("\x1b[0;34m".as_bytes())?;
        self.buf_writer.write_all(meta.name.as_encoded_bytes())?;
        self.buf_writer.write_all("\x1b[0m".as_bytes())?;
        Ok(())
    }

    pub fn paint_entry(
        &mut self,
        meta: &Visitor,
        root: &PathBuf,
        parent: &OsString,
        f: FnOutFile<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}
