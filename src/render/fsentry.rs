use std::ffi::OsString;
use std::fs::Metadata;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::config::root::{TraversalPathBuilder, WithBasePath};
use crate::render::buffer::Buffer;
use crate::walk::fent::FileEntry;

// Trait definitions for different types of writers
pub trait PathWriter {
    fn write_relative_path<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()>;

    fn write_name<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        _path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()>;
}

pub trait HeaderWriter {
    fn write_relative_path<W: Write>(
        buffer: &mut Buffer<W>,
        _meta: &Metadata,
        _root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()>;

    fn write_name<W: Write>(
        buffer: &mut Buffer<W>,
        _meta: &Metadata,
        root: &PathBuf,
        _parent: &OsString,
    ) -> io::Result<()>;
}

// Implementation of writers
pub struct DirectoryWriter;
pub struct FileWriter;
pub struct SymlinkWriter;
pub struct HeaderWriterImpl;

impl PathWriter for DirectoryWriter {
    fn write_relative_path<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        write_entry_path(buffer, entry, path_builder)
    }

    fn write_name<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        _path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        write_entry_name(buffer, entry)
    }
}

impl PathWriter for FileWriter {
    fn write_relative_path<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        write_entry_path(buffer, entry, path_builder)
    }

    fn write_name<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        _path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        write_entry_name(buffer, entry)
    }
}

impl PathWriter for SymlinkWriter {
    fn write_relative_path<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        write_entry_path(buffer, entry, path_builder)
    }

    fn write_name<W: Write>(
        buffer: &mut Buffer<W>,
        entry: &FileEntry,
        _path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        write_entry_name(buffer, entry)
    }
}

impl HeaderWriter for HeaderWriterImpl {
    fn write_relative_path<W: Write>(
        buffer: &mut Buffer<W>,
        _meta: &Metadata,
        _root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let mut path = PathBuf::new();
        path.push(parent);
        buffer.write_os_string(path.into_os_string())
    }

    fn write_name<W: Write>(
        buffer: &mut Buffer<W>,
        _meta: &Metadata,
        root: &PathBuf,
        _parent: &OsString,
    ) -> io::Result<()> {
        if let Some(name) = root.file_name().or_else(|| root.file_stem()) {
            buffer.bufwr.write_all(name.as_encoded_bytes())
        } else {
            buffer.bufwr.write_all(root.to_string_lossy().as_bytes())
        }
    }
}

// Type aliases for callback functions
pub type FnOutDir<W> =
    fn(&mut Buffer<W>, &FileEntry, &TraversalPathBuilder<WithBasePath>) -> io::Result<()>;
pub type FnOutFile<W> =
    fn(&mut Buffer<W>, &FileEntry, &TraversalPathBuilder<WithBasePath>) -> io::Result<()>;
pub type FnOutSymlink<W> =
    fn(&mut Buffer<W>, &FileEntry, &TraversalPathBuilder<WithBasePath>) -> io::Result<()>;
pub type FnOutHead<W> = fn(&mut Buffer<W>, &Metadata, &PathBuf, &OsString) -> io::Result<()>;

// Helper functions
// fn write_entry_path<W: Write>(
//     buffer: &mut Buffer<W>,
//     entry: &FileEntry,
//     path_builder: &mut TraversalPathBuilder<WithBasePath>,
// ) -> io::Result<()> {
//     use crate::config::root::PathManipulation;
//     let pb = path_builder;
//     let relative_path = entry.relative_path(pb.base_path().unwrap()).unwrap();
//     let extended_path = pb.extend_with_relative_path(relative_path.clone());
//     buffer.write_os_string(extended_path.current_path().clone().into_os_string())
// }
fn write_entry_path<W: Write>(
    buffer: &mut Buffer<W>,
    entry: &FileEntry,
    path_builder: &TraversalPathBuilder<WithBasePath>,
) -> io::Result<()> {
    use crate::config::root::PathManipulation;
    // Clone the path builder
    let mut pb = path_builder.clone();
    let relative_path = entry.relative_path(pb.base_path().unwrap()).unwrap();
    let extended_path = pb.extend_with_relative_path(relative_path.clone()); // Now consumes the clone
    buffer.write_os_string(extended_path.current_path().clone().into_os_string())
}

fn write_entry_name<W: Write>(buffer: &mut Buffer<W>, entry: &FileEntry) -> io::Result<()> {
    buffer.bufwr.write_all(entry.filename().as_encoded_bytes())
}

// Buffer implementation
impl<W: Write> Buffer<W> {
    // Directory methods
    pub fn print_dir(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
        callback: FnOutDir<W>,
    ) -> io::Result<()> {
        callback(self, entry, path_builder)
    }

    pub fn write_dir_relative_path(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        DirectoryWriter::write_relative_path(self, entry, path_builder)
    }

    pub fn write_dir(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        DirectoryWriter::write_name(self, entry, path_builder)
    }

    // File methods
    pub fn print_file(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
        callback: FnOutFile<W>,
    ) -> io::Result<()> {
        callback(self, entry, path_builder)
    }

    pub fn write_entry_relative_path(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        FileWriter::write_relative_path(self, entry, path_builder)
    }

    pub fn write_entry(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        FileWriter::write_name(self, entry, path_builder)
    }

    // Symlink methods
    pub fn print_symlink(
        &mut self,
        entry: &mut FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
        callback: FnOutSymlink<W>,
    ) -> io::Result<()> {
        callback(self, entry, path_builder)
    }

    pub fn write_symlink_relative_path(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        SymlinkWriter::write_relative_path(self, entry, path_builder)
    }

    pub fn write_symlink(
        &mut self,
        entry: &FileEntry,
        path_builder: &TraversalPathBuilder<WithBasePath>,
    ) -> io::Result<()> {
        SymlinkWriter::write_name(self, entry, path_builder)
    }

    // Header methods
    pub fn print_header(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
        callback: FnOutHead<W>,
    ) -> io::Result<()> {
        callback(self, meta, root, parent)
    }

    pub fn write_header_relative_path(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        HeaderWriterImpl::write_relative_path(self, meta, root, parent)
    }

    pub fn write_header_name(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        HeaderWriterImpl::write_name(self, meta, root, parent)
    }
}
