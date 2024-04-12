use crate::error::simple::UResult;
use chrono::{DateTime, Local};
use std::io::{self, Write};

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
    pub fn write_report(
        &mut self,
        message: (String, String, String, String, String),
    ) -> io::Result<()> {
        self.write_message(&message.0)?;
        self.write_space()?;
        self.write_message("directories,")?;
        self.write_space()?;
        self.write_message(&message.1)?;
        self.write_space()?;
        self.write_message("files,")?;
        self.write_space()?;
        self.write_message(&message.2)?;
        self.write_space()?;
        self.write_message("hidden,")?;
        self.write_space()?;
        self.write_message(&message.3)?;
        self.write_space()?;
        self.write_message("bytes,")?;
        self.write_space()?;
        self.write_message(&message.4)?;
        self.write_space()?;
        self.write_message("gigabytes")?;
        Ok(())
    }
}

impl<W: Write> Buffer<W> {
    #[allow(dead_code)]
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

pub fn format_system_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%e %b %H:%M").to_string()
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

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
