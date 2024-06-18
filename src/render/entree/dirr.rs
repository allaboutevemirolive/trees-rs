use crate::config::root::BaseDirectory;
use crate::render::buffer::Buffer;
use crate::walk::visit::Visitor;

use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type FnOutDir<W> = fn(&mut Buffer<W>, &Visitor, &BaseDirectory) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_dir_relative_path(
        &mut self,
        meta: &Visitor,
        base_dir: &BaseDirectory,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(&base_dir.base_path).unwrap();

        let mut path = PathBuf::new();
        path.push(base_dir.file_name.clone());
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    pub fn write_color_dir_relative_path(
        &mut self,
        meta: &Visitor,
        base_dir: &BaseDirectory,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(&base_dir.base_path).unwrap();

        let mut path = PathBuf::new();
        path.push(base_dir.file_name.clone());
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all("\x1b[0;34m".as_bytes())?;
        self.bufwr.write_all(path.as_encoded_bytes())?;
        self.bufwr.write_all("\x1b[0m".as_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_dir(&mut self, meta: &Visitor, base_dir: &BaseDirectory) -> io::Result<()> {
        self.bufwr.write_all(meta.filename.as_encoded_bytes())?;
        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_dir_color(&mut self, meta: &Visitor, base_dir: &BaseDirectory) -> io::Result<()> {
        self.bufwr.write_all("\x1b[0;34m".as_bytes())?;
        self.bufwr.write_all(meta.filename.as_encoded_bytes())?;
        self.bufwr.write_all("\x1b[0m".as_bytes())?;
        Ok(())
    }

    pub fn print_dir(
        &mut self,
        meta: &Visitor,
        base_dir: &BaseDirectory,
        f: FnOutDir<W>,
    ) -> io::Result<()> {
        f(self, meta, base_dir)
    }
}
