use std::ffi::OsString;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Buffer<W: Write> {
    pub bufwr: io::BufWriter<W>,
}

pub trait IntoBranch<W: Write> {
    fn print_branch(&mut self, message: &str) -> io::Result<()>;
}

impl<W: Write> IntoBranch<W> for Buffer<W> {
    fn print_branch(&mut self, message: &str) -> io::Result<()> {
        self.write_message(message)
    }
}

impl<W: Write> Buffer<W> {
    /// Initializes a new `Buffer` with a given writer.
    pub fn new(writer: W) -> anyhow::Result<Self> {
        tracing::info!("Initializing Buffer");
        Ok(Self::create_buffer(writer))
    }

    /// Writes a message to the buffer.
    pub fn write_message(&mut self, message: &str) -> io::Result<()> {
        self.write_bytes(message.as_bytes())
    }

    /// Writes an `OsString` to the buffer.
    pub fn write_os_string(&mut self, message: OsString) -> io::Result<()> {
        self.write_bytes(message.as_encoded_bytes())
    }

    /// Writes a newline to the buffer.
    pub fn newline(&mut self) -> io::Result<()> {
        self.write_bytes("\n".as_bytes())
    }

    /// Writes a space to the buffer.
    pub fn write_space(&mut self) -> io::Result<()> {
        self.write_bytes(" ".as_bytes())
    }

    pub fn code_block(&mut self) -> io::Result<()> {
        self.write_bytes("```".as_bytes())
    }

    /// Helper method to write bytes to the buffer.
    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.bufwr.write_all(bytes)
    }

    /// Helper method to create a `BufWriter`.
    fn create_buffer(writer: W) -> Buffer<W> {
        Buffer {
            bufwr: io::BufWriter::new(writer),
        }
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
        buffer.as_mut().unwrap().write_message(message).unwrap();

        let buffer_contents = buffer.unwrap().bufwr.into_inner().unwrap();
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
