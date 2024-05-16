use crate::canva::buffer::Buffer;
use crate::walk::visit::Visitor;
use std::ffi::OsString;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type FnOutSymlink<W> = fn(&mut Buffer<W>, &mut Visitor, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_symlink_relative_path(
        &mut self,
        visit: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let relative_path = visit.get_relative_path(root).unwrap();

        let mut path = PathBuf::new();
        path.push(parent);
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    pub fn write_color_symlink_relative_path(
        &mut self,
        visit: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let relative_path = visit.get_relative_path(root).unwrap();

        let mut path = PathBuf::new();
        path.push(parent);
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all("\x1b[0;34m".as_bytes())?;
        self.bufwr.write_all(path.as_encoded_bytes())?;
        self.bufwr.write_all("\x1b[0m".as_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_symlink(
        &mut self,
        visit: &mut Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.bufwr.write_all(visit.filename.as_encoded_bytes())?;

        let target_link = visit
            .get_target_symlink()
            .expect("Cannot get target link.")
            .into_os_string();

        self.bufwr.write_all(" -> ".as_bytes())?;

        self.bufwr.write_all(target_link.as_encoded_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_symlink_color(
        &mut self,
        visit: &Visitor,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.bufwr.write_all("\x1b[0;34m".as_bytes())?;
        self.bufwr.write_all(visit.filename.as_encoded_bytes())?;
        self.bufwr.write_all("\x1b[0m".as_bytes())?;
        Ok(())
    }

    pub fn paint_symlink(
        &mut self,
        visit: &mut Visitor,
        root: &PathBuf,
        parent: &OsString,
        f: FnOutSymlink<W>,
    ) -> io::Result<()> {
        f(self, visit, root, parent)
    }
}
