use crate::render::buffer::Buffer;
use std::fs::Metadata;
use std::io;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

pub type FnExtPermission<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;

impl<W: Write> Buffer<W> {
    /// Print entry's permission
    pub fn print_permission(&mut self, meta: &Metadata, f: FnExtPermission<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn write_no_permission(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }

    #[cfg(unix)]
    pub fn write_permission(&mut self, meta: &Metadata) -> io::Result<()> {
        let mode = meta.permissions().mode();

        self.write_space()?;

        if meta.is_dir() {
            self.bufwr.write_all("d".as_bytes())?;
        } else {
            self.bufwr.write_all(".".as_bytes())?;
        }

        if mode & 0o400 != 0 {
            self.bufwr.write_all("r".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o200 != 0 {
            self.bufwr.write_all("w".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o100 != 0 {
            self.bufwr.write_all("x".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o40 != 0 {
            self.bufwr.write_all("r".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o20 != 0 {
            self.bufwr.write_all("w".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o10 != 0 {
            self.bufwr.write_all("x".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o4 != 0 {
            self.bufwr.write_all("r".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o2 != 0 {
            self.bufwr.write_all("w".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        if mode & 0o1 != 0 {
            self.bufwr.write_all("x".as_bytes())?;
        } else {
            self.bufwr.write_all("-".as_bytes())?;
        }

        self.write_space()
    }

    #[cfg(windows)]
    pub fn write_permission(&mut self, meta: &Metadata) -> io::Result<()> {
        use std::os::windows::fs::MetadataExt;
        use winapi::um::winnt::{GENERIC_EXECUTE, GENERIC_READ, GENERIC_WRITE};

        self.write_space()?;

        if meta.is_dir() {
            self.bufwr.write_all("d".as_bytes())?;
        } else {
            self.bufwr.write_all(".".as_bytes())?;
        }

        // Get the security descriptor of the file
        let security_descriptor = meta.get_security_descriptor()?;

        // Check permissions for the owner
        let access_rights = security_descriptor.get_access_rights()?;
        self.bufwr.write_all(if access_rights & GENERIC_READ != 0 {
            "r".as_bytes()
        } else {
            "-".as_bytes()
        })?;
        self.bufwr
            .write_all(if access_rights & GENERIC_WRITE != 0 {
                "w".as_bytes()
            } else {
                "-".as_bytes()
            })?;
        self.bufwr
            .write_all(if access_rights & GENERIC_EXECUTE != 0 {
                "x".as_bytes()
            } else {
                "-".as_bytes()
            })?;

        // Check permissions for the group
        let access_rights = security_descriptor.get_group_access_rights()?;
        self.bufwr.write_all(if access_rights & GENERIC_READ != 0 {
            "r".as_bytes()
        } else {
            "-".as_bytes()
        })?;
        self.bufwr
            .write_all(if access_rights & GENERIC_WRITE != 0 {
                "w".as_bytes()
            } else {
                "-".as_bytes()
            })?;
        self.bufwr
            .write_all(if access_rights & GENERIC_EXECUTE != 0 {
                "x".as_bytes()
            } else {
                "-".as_bytes()
            })?;

        // Check permissions for others
        let access_rights = security_descriptor.get_others_access_rights()?;
        self.bufwr.write_all(if access_rights & GENERIC_READ != 0 {
            "r".as_bytes()
        } else {
            "-".as_bytes()
        })?;
        self.bufwr
            .write_all(if access_rights & GENERIC_WRITE != 0 {
                "w".as_bytes()
            } else {
                "-".as_bytes()
            })?;
        self.bufwr
            .write_all(if access_rights & GENERIC_EXECUTE != 0 {
                "x".as_bytes()
            } else {
                "-".as_bytes()
            })?;

        self.write_space()
    }
}
