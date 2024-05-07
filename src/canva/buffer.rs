use crate::error::simple::TResult;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Buffer<W: Write> {
    pub buf_writer: io::BufWriter<W>,
}

pub trait IntoBranch<W: Write> {
    fn write_branch(&mut self, message: &str) -> io::Result<()>;
}

impl<W: Write> IntoBranch<W> for Buffer<W> {
    fn write_branch(&mut self, message: &str) -> io::Result<()> {
        self.buf_writer.write_all(message.as_bytes())
    }
}

impl<W: Write> Buffer<W> {
    pub fn new(writer: W) -> TResult<Self> {
        let buf_writer = io::BufWriter::new(writer);
        Ok(Buffer { buf_writer })
    }

    pub fn write_message(&mut self, message: &str) -> io::Result<()> {
        self.buf_writer.write_all(message.as_bytes())
    }
}

impl<W: Write> Buffer<W> {
    pub fn write_newline(&mut self) -> io::Result<()> {
        self.buf_writer.write_all("\n".as_bytes())
    }

    pub fn write_space(&mut self) -> io::Result<()> {
        self.buf_writer.write_all(" ".as_bytes())
    }
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
