use crate::canva::buffer::Buffer;
use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub type FnOutHead<W> = fn(&mut Buffer<W>, &Metadata, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    #[allow(unused_variables)]
    #[allow(clippy::ptr_arg)]
    pub fn write_header_relative_path(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let mut path = PathBuf::new();
        path.push(parent);

        let path = path.to_owned().into_os_string();
        self.buf_writer.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn write_header_name(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        // Passing "../../" will result in panick thus we will convert pathbuf to string directly
        // if we cannot retrieve the filename().
        //
        // thread 'main' panicked at src/canva/which/headd.rs:35:41:
        // called `Option::unwrap()` on a `None` value
        // self.buf_writer
        //     .write_all(root.file_name().unwrap_or_default().as_encoded_bytes())?;
        if let Some(file_name) = root.file_name() {
            self.buf_writer.write_all(file_name.as_encoded_bytes())?;
        } else if let Some(folder_name) = root.file_stem() {
            self.buf_writer.write_all(folder_name.as_encoded_bytes())?;
        } else {
            let path_buf = PathBuf::from(root);
            let filename: String = path_buf.to_string_lossy().to_string();

            self.buf_writer.write_all(filename.as_bytes())?;
        }

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn write_color_header_name(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        // Passing "../../" will result in panick thus we will convert pathbuf to string directly
        // if we cannot retrieve the filename().
        //
        // thread 'main' panicked at src/canva/which/headd.rs:35:41:
        // called `Option::unwrap()` on a `None` value
        // self.buf_writer
        //     .write_all(root.file_name().unwrap_or_default().as_encoded_bytes())?;
        if let Some(file_name) = root.file_name() {
            self.buf_writer.write_all("\x1b[0;34m".as_bytes())?;
            self.buf_writer.write_all(file_name.as_encoded_bytes())?;
            self.buf_writer.write_all("\x1b[0m".as_bytes())?;
        } else if let Some(folder_name) = root.file_stem() {
            self.buf_writer.write_all("\x1b[0;34m".as_bytes())?;
            self.buf_writer.write_all(folder_name.as_encoded_bytes())?;
            self.buf_writer.write_all("\x1b[0m".as_bytes())?;
        } else {
            let path_buf = PathBuf::from(root);
            let filename: String = path_buf.to_string_lossy().to_string();
            self.buf_writer.write_all("\x1b[0;34m".as_bytes())?;
            self.buf_writer.write_all(filename.as_bytes())?;
            self.buf_writer.write_all("\x1b[0m".as_bytes())?;
        }

        Ok(())
    }

    pub fn paint_header(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
        f: FnOutHead<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}
