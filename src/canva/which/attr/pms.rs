use crate::canva::buffer::Buffer;
use std::fs::Metadata;
use std::io::Write;
use std::io::{self, StdoutLock};
use std::os::unix::fs::PermissionsExt;

pub type FnExtPermission<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

pub fn write_no_permission(_buf: &mut Buffer<StdoutLock>, _meta: &Metadata) -> io::Result<()> {
    Ok(())
}

pub fn write_permission(buf: &mut Buffer<StdoutLock>, meta: &Metadata) -> io::Result<()> {
    let mode = meta.permissions().mode();

    buf.write_space()?;

    if meta.is_dir() {
        buf.buf_writer.write_all("d".as_bytes())?;
    } else {
        buf.buf_writer.write_all(".".as_bytes())?;
    }

    if mode & 0o400 != 0 {
        buf.buf_writer.write_all("r".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o200 != 0 {
        buf.buf_writer.write_all("w".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o100 != 0 {
        buf.buf_writer.write_all("x".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o40 != 0 {
        buf.buf_writer.write_all("r".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o20 != 0 {
        buf.buf_writer.write_all("w".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o10 != 0 {
        buf.buf_writer.write_all("x".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o4 != 0 {
        buf.buf_writer.write_all("r".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o2 != 0 {
        buf.buf_writer.write_all("w".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    if mode & 0o1 != 0 {
        buf.buf_writer.write_all("x".as_bytes())?;
    } else {
        buf.buf_writer.write_all("-".as_bytes())?;
    }

    buf.write_space()
}

// impl<W: Write> Buffer<W> {
//     /// Print entry's permission
//     pub fn paint_permission(&mut self, meta: &Metadata, f: FnExtPermission<W>) -> io::Result<()> {
//         f(self, meta)
//     }

//     pub fn write_no_permission(&mut self, _meta: &Metadata) -> io::Result<()> {
//         Ok(())
//     }

//     pub fn write_permission(&mut self, meta: &Metadata) -> io::Result<()> {
//         let mode = meta.permissions().mode();

//         self.write_space()?;

//         if meta.is_dir() {
//             self.buf_writer.write_all("d".as_bytes())?;
//         } else {
//             self.buf_writer.write_all(".".as_bytes())?;
//         }

//         if mode & 0o400 != 0 {
//             self.buf_writer.write_all("r".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o200 != 0 {
//             self.buf_writer.write_all("w".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o100 != 0 {
//             self.buf_writer.write_all("x".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o40 != 0 {
//             self.buf_writer.write_all("r".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o20 != 0 {
//             self.buf_writer.write_all("w".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o10 != 0 {
//             self.buf_writer.write_all("x".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o4 != 0 {
//             self.buf_writer.write_all("r".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o2 != 0 {
//             self.buf_writer.write_all("w".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         if mode & 0o1 != 0 {
//             self.buf_writer.write_all("x".as_bytes())?;
//         } else {
//             self.buf_writer.write_all("-".as_bytes())?;
//         }

//         self.write_space()
//     }
// }
