use crate::config::root::PathBuilder;
use crate::render::buffer::Buffer;
use crate::walk::visit::Visitor;

use std::io;
use std::io::Write;

pub type FnOutSymlink<W> = fn(&mut Buffer<W>, &Visitor, &PathBuilder) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_symlink_relative_path(
        &mut self,
        visit: &Visitor,
        path_builder: &PathBuilder,
    ) -> io::Result<()> {
        self.write_os_string(
            path_builder
                .clone()
                .append_relative(visit)
                .unwrap()
                .clone()
                .into_os_string(),
        )?;
        Ok(())
    }

    pub fn write_symlink(
        &mut self,
        visit: &Visitor,
        _path_builder: &PathBuilder,
    ) -> io::Result<()> {
        self.write_os_string(visit.filename().clone())?;
        Ok(())
    }

    pub fn print_symlink(
        &mut self,
        visit: &mut Visitor,
        base_dir: &PathBuilder,
        f: FnOutSymlink<W>,
    ) -> io::Result<()> {
        f(self, visit, base_dir)
    }
}
