use crate::canva::buffer::Buffer;
use crate::config::root::RootPath;
use crate::walk::visit::Visitor;

use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type FnOutFile<W> = fn(&mut Buffer<W>, &Visitor, &RootPath) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_entry_relative_path(
        &mut self,
        meta: &Visitor,
        rpath: &RootPath,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(&rpath.fpath).unwrap();

        let mut path = PathBuf::new();
        path.push(rpath.fname.clone());
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    pub fn write_color_entry_relative_path(
        &mut self,
        meta: &Visitor,
        rpath: &RootPath,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(&rpath.fpath).unwrap();

        let mut path = PathBuf::new();
        path.push(rpath.fname.clone());
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all("\x1b[0;34m".as_bytes())?;
        self.bufwr.write_all(path.as_encoded_bytes())?;
        self.bufwr.write_all("\x1b[0m".as_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_entry(&mut self, meta: &Visitor, rpath: &RootPath) -> io::Result<()> {
        self.bufwr.write_all(meta.filename.as_encoded_bytes())?;
        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_entry_color(&mut self, meta: &Visitor, rpath: &RootPath) -> io::Result<()> {
        self.bufwr.write_all("\x1b[0;34m".as_bytes())?;
        self.bufwr.write_all(meta.filename.as_encoded_bytes())?;
        self.bufwr.write_all("\x1b[0m".as_bytes())?;
        Ok(())
    }

    pub fn print_file(
        &mut self,
        meta: &Visitor,
        rpath: &RootPath,
        f: FnOutFile<W>,
    ) -> io::Result<()> {
        f(self, meta, rpath)
    }
}
