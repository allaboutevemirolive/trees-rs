use chrono::{DateTime, Local};
use std::ffi::OsString;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::{
    io::{self, Write},
    path::PathBuf,
};

use crate::error::simple::UResult;
use crate::walk::metada::FileMetadata;

#[derive(Debug)]
pub struct Buffer<W: Write> {
    pub buf_writer: io::BufWriter<W>,
}

impl<W: Write> Buffer<W> {
    pub fn new(writer: W) -> UResult<Self> {
        let buf_writer = io::BufWriter::new(writer);
        Ok(Buffer { buf_writer })
    }

    pub fn write_message(&mut self, message: &str) -> io::Result<()> {
        self.buf_writer.write_all(message.as_bytes())
    }

    pub fn write_branch(&mut self, message: &str) -> io::Result<()> {
        self.buf_writer.write_all(message.as_bytes())
    }

    /// 41 directories, 480 files, 0 size, 0 hidden
    pub fn write_report(&mut self, message: (String, String, String, String)) -> io::Result<()> {
        self.write_message(&message.0)?;
        self.write_space()?;
        self.write_message("directories,")?;
        self.write_space()?;
        self.write_message(&message.1)?;
        self.write_space()?;
        self.write_message("files,")?;
        self.write_space()?;
        self.write_message(&message.3)?;
        self.write_space()?;
        self.write_message("hidden, ")?;
        self.write_space()?;
        self.write_message(&message.2)?;
        self.write_space()?;
        self.write_message("size")?;
        Ok(())
    }
}

impl<W: Write> Buffer<W> {
    pub fn write_separator(&mut self) -> io::Result<()> {
        self.buf_writer.write_all(", ".as_bytes())
    }

    pub fn write_newline(&mut self) -> io::Result<()> {
        self.buf_writer.write_all("\n".as_bytes())
    }

    pub fn write_space(&mut self) -> io::Result<()> {
        self.buf_writer.write_all(" ".as_bytes())
    }
}

pub type WhichHeader<W> = fn(&mut Buffer<W>, &Metadata, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
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
        self.buf_writer
            .write_all(root.file_name().unwrap().as_encoded_bytes())?;
        Ok(())
    }

    pub fn paint_header(
        &mut self,
        meta: &Metadata,
        root: &PathBuf,
        parent: &OsString,
        f: WhichHeader<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}

pub type WhichHeaderDate<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_header_date(&mut self, meta: &Metadata) -> io::Result<()> {
        let created = meta.created()?;
        let time = format_system_time(created);
        self.write_space()?;
        self.buf_writer.write_all(time.as_bytes())?;
        self.write_space()
    }

    pub fn write_no_header_date(&mut self, _meta: &Metadata) -> io::Result<()> {
        self.buf_writer.write_all("".as_bytes())
    }

    pub fn paint_header_date(&mut self, meta: &Metadata, f: WhichHeaderDate<W>) -> io::Result<()> {
        f(self, meta)
    }
}

pub type WhichHeaderAttribute<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_header_attribute(&mut self, meta: &Metadata) -> io::Result<()> {
        let permissions = meta.permissions();
        let mode = permissions.mode();

        let file_type = if meta.is_dir() { 'd' } else { '.' };

        let symbolic_permissions = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            file_type,
            if mode & 0o400 != 0 { 'r' } else { '-' },
            if mode & 0o200 != 0 { 'w' } else { '-' },
            if mode & 0o100 != 0 { 'x' } else { '-' },
            if mode & 0o40 != 0 { 'r' } else { '-' },
            if mode & 0o20 != 0 { 'w' } else { '-' },
            if mode & 0o10 != 0 { 'x' } else { '-' },
            if mode & 0o4 != 0 { 'r' } else { '-' },
            if mode & 0o2 != 0 { 'w' } else { '-' },
            if mode & 0o1 != 0 { 'x' } else { '-' },
        );

        self.write_space()?;
        self.buf_writer.write_all(symbolic_permissions.as_bytes())?;
        self.write_space()
    }

    pub fn write_no_header_attribute(&mut self, _meta: &Metadata) -> io::Result<()> {
        self.buf_writer.write_all("".as_bytes())
    }

    pub fn paint_header_attribute(
        &mut self,
        meta: &Metadata,
        f: WhichHeaderAttribute<W>,
    ) -> io::Result<()> {
        f(self, meta)
    }
}

pub type WhichFile<W> = fn(&mut Buffer<W>, &FileMetadata, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_file_relative_path(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(root).unwrap();

        let mut path = PathBuf::new();
        path.push(parent);
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.buf_writer.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn write_filename(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.buf_writer
            .write_all(meta.file_name.as_encoded_bytes())?;
        Ok(())
    }

    pub fn paint_file(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
        f: WhichFile<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}

pub type WhichEntry<W> = fn(&mut Buffer<W>, &FileMetadata, &PathBuf, &OsString) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_relative_path(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        let relative_path = meta.get_relative_path(root).unwrap();

        let mut path = PathBuf::new();
        path.push(parent);
        path.push(relative_path);

        let path = path.to_owned().into_os_string();
        self.buf_writer.write_all(path.as_encoded_bytes())?;

        Ok(())
    }

    pub fn write_dirname(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.buf_writer
            .write_all(meta.file_name.as_encoded_bytes())?;
        Ok(())
    }

    pub fn write_dirname_color(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
    ) -> io::Result<()> {
        self.buf_writer.write_all("\x1b[0;34m".as_bytes())?;
        self.buf_writer
            .write_all(meta.file_name.as_encoded_bytes())?;
        self.buf_writer.write_all("\x1b[0m".as_bytes())?;
        Ok(())
    }

    pub fn paint_entry(
        &mut self,
        meta: &FileMetadata,
        root: &PathBuf,
        parent: &OsString,
        f: WhichEntry<W>,
    ) -> io::Result<()> {
        f(self, meta, root, parent)
    }
}

pub type WhichAttribute<W> = fn(&mut Buffer<W>, &FileMetadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_attribute(&mut self, meta: &FileMetadata) -> io::Result<()> {
        let symbolic_permissions = meta.get_symbolic_permissions()?;
        self.write_space()?;
        self.buf_writer
            .write_all(symbolic_permissions.as_encoded_bytes())?;
        self.write_space()
    }

    pub fn write_no_attribute(&mut self, _meta: &FileMetadata) -> io::Result<()> {
        self.buf_writer.write_all("".as_bytes())
    }

    pub fn paint_attribute(&mut self, meta: &FileMetadata, f: WhichAttribute<W>) -> io::Result<()> {
        f(self, meta)
    }
}

pub type WhichDate<W> = fn(&mut Buffer<W>, &FileMetadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_date(&mut self, meta: &FileMetadata) -> io::Result<()> {
        let created = meta.dir_entry.metadata()?.created()?;
        let time = format_system_time(created);
        self.write_space()?;
        self.buf_writer.write_all(time.as_bytes())?;
        self.write_space()
    }

    pub fn write_no_date(&mut self, _meta: &FileMetadata) -> io::Result<()> {
        self.buf_writer.write_all("".as_bytes())
    }

    pub fn paint_date(&mut self, meta: &FileMetadata, f: WhichDate<W>) -> io::Result<()> {
        f(self, meta)
    }
}

fn format_system_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%e %b %H:%M").to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test test_write_message -- --nocapture
    #[test]
    fn test_write_message() {
        let mut buffer = Buffer::new(Vec::new());

        let message = "Hello, world!";
        buffer.as_mut().unwrap().write_message(&message).unwrap();

        let buffer_contents = buffer.unwrap().buf_writer.into_inner().unwrap();
        let output_string = String::from_utf8(buffer_contents).unwrap();

        assert_eq!(OsString::from(output_string), message);
    }

    #[test]
    fn test_buffer_with_stdout() {
        let stdout = io::stdout();
        let buffer = Buffer::new(stdout.lock());

        buffer.unwrap().write_message("Hello, world!").unwrap();
    }
}
