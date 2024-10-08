use crate::config::root::TraversalPathBuilder;
use crate::render::buffer::Buffer;
use crate::walk::visit::FileEntry;

use std::io;
use std::io::Write;

pub type FnOutFile<W> = fn(&mut Buffer<W>, &FileEntry, &TraversalPathBuilder) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_entry_relative_path(
        &mut self,
        visit: &FileEntry,
        path_builder: &TraversalPathBuilder,
    ) -> io::Result<()> {
        // No need to pop since we clone path_builder
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

    pub fn write_entry(
        &mut self,
        visit: &FileEntry,
        _path_builder: &TraversalPathBuilder,
    ) -> io::Result<()> {
        self.write_os_string(visit.filename().clone())?;
        Ok(())
    }

    pub fn print_file(
        &mut self,
        meta: &FileEntry,
        base_dir: &TraversalPathBuilder,
        f: FnOutFile<W>,
    ) -> io::Result<()> {
        f(self, meta, base_dir)
    }
}
