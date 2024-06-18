use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Struct that store the path where we needs to start traverse
pub struct BaseDirectory {
    pub file_name: OsString,
    // This is not always absolute path
    pub base_path: PathBuf,
}

impl BaseDirectory {
    pub fn from_current_dir() -> io::Result<Self> {
        let base_path = env::current_dir()?;

        Ok(Self {
            file_name: base_path
                .file_name()
                .map(OsString::from)
                .unwrap_or_else(|| OsString::from(".")),
            base_path,
        })
    }

    pub fn filename(&self) -> OsString {
        self.file_name.clone()
    }

    pub fn with_filename(&mut self, file_name: OsString) {
        self.file_name = file_name;
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_path.clone()
    }

    pub fn with_base_path(&mut self, base_path: PathBuf) {
        self.base_path = base_path
    }

    /// Sets the file name to "." (dot), indicating the current directory.
    pub fn set_file_name_to_current_dir(&mut self) {
        self.file_name = ".".into()
    }

    /// Retrieves metadata for the base directory.
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.base_path)
    }
}
