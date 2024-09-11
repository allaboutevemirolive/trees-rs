use crate::render::buffer::Buffer;
use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type FnOutHead<W> = fn(&mut Buffer<W>, &Metadata, &PathBuf, &OsString) -> io::Result<()>;

#[allow(clippy::ptr_arg)]
impl<W: Write> Buffer<W> {
    pub fn write_header_relative_path(
        &mut self,
        _meta: &Metadata,
        _root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let mut path = PathBuf::new();
        path.push(parent);

        let path = path.to_owned().into_os_string();
        self.bufwr.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    /// Writes the header name (filename or directory name) to the buffer writer.
    ///
    /// This function handles the logic for determining the appropriate name to write:
    ///
    /// 1. File: If the `root` path represents a file, it extracts and writes the filename.
    /// 2. Directory: If `root` is a directory, it extracts and writes the directory name (without the trailing path separator).
    /// 3. Fallback: If neither filename nor directory name can be obtained (e.g., root is "." or ""),
    ///     it converts the entire `root` path to a string and writes that. This handles edge cases that
    ///     could otherwise cause panics.
    ///
    pub fn write_header_name(
        &mut self,
        _meta: &Metadata,
        root: &PathBuf,
        _parent: &OsString,
    ) -> io::Result<()> {
        // Attempt to extract and write the filename
        if let Some(file_name) = root.file_name() {
            self.bufwr.write_all(file_name.as_encoded_bytes())?;

        // If that fails, try extracting and writing the directory name (without the trailing separator)
        } else if let Some(folder_name) = root.file_stem() {
            self.bufwr.write_all(folder_name.as_encoded_bytes())?;

        // If neither works, fall back to writing the entire path as a string
        } else {
            let filename = root.to_string_lossy();
            self.bufwr.write_all(filename.as_bytes())?;
        }

        Ok(())
    }

    pub fn print_header(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
        f: FnOutHead<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}
