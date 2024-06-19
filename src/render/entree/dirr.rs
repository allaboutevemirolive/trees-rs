use crate::config::root::PathBuilder;
use crate::render::buffer::Buffer;
use crate::walk::visit::Visitor;

use std::io;
use std::io::Write;

pub type FnOutDir<W> = fn(&mut Buffer<W>, &Visitor, &PathBuilder) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_dir_relative_path(
        &mut self,
        visit: &Visitor,
        path_builder: &PathBuilder,
    ) -> io::Result<()> {
        self.bufwr.write_all(
            path_builder
                .clone()
                .append_relative(visit)
                .into_os_string()
                .as_encoded_bytes(),
        )?;

        Ok(())
    }

    pub fn write_dir(&mut self, meta: &Visitor, _path_builder: &PathBuilder) -> io::Result<()> {
        self.bufwr.write_all(meta.filename().as_encoded_bytes())?;
        Ok(())
    }

    pub fn print_dir(
        &mut self,
        meta: &Visitor,
        base_dir: &PathBuilder,
        f: FnOutDir<W>,
    ) -> io::Result<()> {
        f(self, meta, base_dir)
    }
}
