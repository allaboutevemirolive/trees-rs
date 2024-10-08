use crate::config::root::TraversalPathBuilder;
use crate::render::buffer::Buffer;
use crate::walk::visit::FileEntry;

use std::io;
use std::io::Write;

pub type FnOutSymlink<W> = fn(&mut Buffer<W>, &FileEntry, &TraversalPathBuilder) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_symlink_relative_path(
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

    pub fn write_symlink(
        &mut self,
        visit: &FileEntry,
        _path_builder: &TraversalPathBuilder,
    ) -> io::Result<()> {
        self.write_os_string(visit.filename().clone())?;
        Ok(())
    }

    pub fn print_symlink(
        &mut self,
        visit: &mut FileEntry,
        base_dir: &TraversalPathBuilder,
        f: FnOutSymlink<W>,
    ) -> io::Result<()> {
        f(self, visit, base_dir)
    }
}
