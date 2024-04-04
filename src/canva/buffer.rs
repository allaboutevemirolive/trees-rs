use std::{
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use crate::error::simple::UResult;

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
        self.write_message(&message.2)?;
        self.write_space()?;
        self.write_message("size,")?;
        self.write_space()?;
        self.write_message(&message.3)?;
        self.write_space()?;
        self.write_message("hidden")
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

impl<W: Write> Buffer<W> {
    pub fn write_relative_path(&mut self, relative_path: PathBuf) -> io::Result<()> {
        // ToDO:
        self.buf_writer.write_all("./".as_bytes())?;
        self.buf_writer
            .write_all(relative_path.to_str().unwrap().as_bytes())
    }

    pub fn write_absolute_path(&mut self, absolute_path: PathBuf) -> io::Result<()> {
        self.buf_writer
            .write_all(absolute_path.to_str().unwrap().as_bytes())
    }
}

impl<W: Write> Buffer<W> {
    pub fn paint_text(&mut self, start: &str, text: &str, reset: &str) -> io::Result<()> {
        self.start_color(start)?;
        self.buf_writer.write_all(text.as_bytes())?;
        self.reset_color(reset)
    }

    fn start_color(&mut self, start: &str) -> io::Result<()> {
        self.buf_writer.write_all(start.as_bytes())
    }

    fn reset_color(&mut self, start: &str) -> io::Result<()> {
        self.buf_writer.write_all(start.as_bytes())
    }
}

pub type WhichPaint<W> = fn(&mut Buffer<W>, &str) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn write_dir_name(&mut self, dir: &str) -> io::Result<()> {
        self.buf_writer.write_all(dir.as_bytes())
    }

    pub fn write_dir_name_color(&mut self, dir: &str) -> io::Result<()> {
        self.buf_writer.write_all("\x1b[0;34m".as_bytes())?; // blue
        self.buf_writer.write_all(dir.as_bytes())?;
        self.buf_writer.write_all("\x1b[0m".as_bytes())
    }

    pub fn write_file_name(&mut self, file: &str) -> io::Result<()> {
        self.buf_writer.write_all(file.as_bytes())
    }

    fn write_no_file_ext(&mut self, file: &str) -> io::Result<()> {
        self.buf_writer.write_all("\x1b[0;32m".as_bytes())?; // green
        self.buf_writer.write_all(file.as_bytes())?;
        self.buf_writer.write_all("\x1b[0m".as_bytes())
    }

    pub fn paint(&mut self, dir: &str, f: WhichPaint<W>) -> io::Result<()> {
        f(self, dir)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test test_write_message -- --nocapture
    #[test]
    fn test_write_message() {
        // Create a buffer with an in-memory writer
        let mut buffer = Buffer::new(Vec::new());

        // Write a message to the buffer
        let message = "Hello, world!";
        buffer.as_mut().unwrap().write_message(message).unwrap();

        // Get the contents of the buffer
        let buffer_contents = buffer.unwrap().buf_writer.into_inner().unwrap();

        let output_string = String::from_utf8(buffer_contents).unwrap();

        assert_eq!(output_string, message);
    }

    #[test]
    fn test_buffer_with_stdout() {
        let stdout = io::stdout();
        let buffer = Buffer::new(stdout.lock());

        buffer.unwrap().write_message("Hello, world!").unwrap();
    }

    // use std::io::{Cursor, StdoutLock};

    // // Mock implementation of Write trait for testing purposes
    // struct MockWriter(Vec<u8>);

    // impl Write for MockWriter {
    //     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //         self.0.extend_from_slice(buf);
    //         Ok(buf.len())
    //     }

    //     fn flush(&mut self) -> io::Result<()> {
    //         Ok(())
    //     }
    // }

    // #[test]
    // fn test_write_dir_name() {
    //     let mut buffer = Buffer {
    //         buf_writer: io::BufWriter::new(MockWriter(Vec::new())),
    //     };
    //     buffer.write_dir_name("test_dir").unwrap();
    //     assert_eq!(buffer.buf_writer.get_ref().0, b"test_dir");
    // }

    // cargo test test_paint -- --nocapture
    #[test]
    fn test_paint() {
        let stdout = io::stdout();
        // let buffer = Buffer::new(stdout.lock());

        let mut buffer = Buffer {
            buf_writer: io::BufWriter::new(stdout.lock()),
        };
        // let color = Buffer::write_dir_name_color;
        // let color: PaintText<S> = Buffer::<StdoutLock>::write_dir_name_color;
        buffer
            .paint("Hello World!", Buffer::write_dir_name_color)
            .unwrap();
    }
}
