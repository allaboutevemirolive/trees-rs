use crate::config::root::PathBuilder;
use crate::render::buffer::Buffer;
use crate::walk::visit::Visitor;

use std::io;
use std::io::Write;

pub type FnOutFile<W> = fn(&mut Buffer<W>, &Visitor, &PathBuilder) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_entry_relative_path(
        &mut self,
        visit: &Visitor,
        path_builder: &PathBuilder,
    ) -> io::Result<()> {
        // No need to pop since we clone path_builder
        self.write_os_string(path_builder.clone().append_relative(visit).to_os_string())?;

        Ok(())
    }

    pub fn write_entry(&mut self, visit: &Visitor, _path_builder: &PathBuilder) -> io::Result<()> {
        self.write_os_string(visit.filename().clone())?;
        Ok(())
    }

    pub fn print_file(
        &mut self,
        meta: &Visitor,
        base_dir: &PathBuilder,
        f: FnOutFile<W>,
    ) -> io::Result<()> {
        f(self, meta, base_dir)
    }
}
