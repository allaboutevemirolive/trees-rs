use crate::render::buffer::Buffer;
use std::io;
use std::io::Write;

pub type FnColor<W> = fn(&mut Buffer<W>) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    pub fn reset_color(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1b[0m")
    }

    pub fn bold_red(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1b[41m")
    }

    pub fn underlined_blue(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1b[4;34m")
    }

    pub fn yellow(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1b[0;33m")
    }

    pub fn blue(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1b[0;34m")
    }

    pub fn green(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1B[1;32m")
    }

    pub fn purple(&mut self) -> io::Result<()> {
        self.bufwr.write_all(b"\x1B[1;35m")
    }

    pub fn no_color(&mut self) -> io::Result<()> {
        Ok(())
    }
}
