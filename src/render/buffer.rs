use std::ffi::OsString;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Buffer<W: Write> {
    pub bufwr: io::BufWriter<W>,
}

pub trait IntoBranch<W: Write> {
    fn print_branch(&mut self, message: &str) -> io::Result<()>;
}

impl<W: Write> IntoBranch<W> for Buffer<W> {
    fn print_branch(&mut self, message: &str) -> io::Result<()> {
        self.bufwr.write_all(message.as_bytes())
    }
}

impl<W: Write> Buffer<W> {
    pub fn new(writer: W) -> anyhow::Result<Self> {
        let bufwr = io::BufWriter::new(writer);
        Ok(Buffer { bufwr })
    }

    pub fn write_message(&mut self, message: &str) -> io::Result<()> {
        self.bufwr.write_all(message.as_bytes())
    }

    pub fn write_os_string(&mut self, message: OsString) -> io::Result<()> {
        self.bufwr.write_all(message.as_encoded_bytes())
    }
}

impl<W: Write> Buffer<W> {
    pub fn newline(&mut self) -> io::Result<()> {
        self.bufwr.write_all("\n".as_bytes())
    }

    pub fn write_space(&mut self) -> io::Result<()> {
        self.bufwr.write_all(" ".as_bytes())
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
