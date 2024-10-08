use crate::config::root::TraversalPathBuilder;
use crate::render::buffer::Buffer;
use crate::walk::visit::FileEntry;

use std::io;
use std::io::Write;

pub type FnOutDir<W> = fn(&mut Buffer<W>, &FileEntry, &TraversalPathBuilder) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_dir_relative_path(
        &mut self,
        visit: &FileEntry,
        path_builder: &TraversalPathBuilder,
    ) -> io::Result<()> {
        self.write_os_string(
            path_builder
                .clone()
                .extend_with_relative_from_visitor(visit)
                .unwrap()
                .clone()
                .into_os_string(),
        )?;

        Ok(())
    }

    pub fn write_dir(
        &mut self,
        meta: &FileEntry,
        _path_builder: &TraversalPathBuilder,
    ) -> io::Result<()> {
        self.bufwr.write_all(meta.filename().as_encoded_bytes())?;
        Ok(())
    }

    pub fn print_dir(
        &mut self,
        meta: &FileEntry,
        base_dir: &TraversalPathBuilder,
        f: FnOutDir<W>,
    ) -> io::Result<()> {
        f(self, meta, base_dir)
    }
}
