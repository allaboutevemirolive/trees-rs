use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

// use once_cell::sync::OnceCell;

use crate::error::simple::TResult;
use crate::walk::visit::Visitor;

/// Struct that store the path where we needs to start traverse
#[derive(Clone)]
pub struct BaseDirectory {
    // If path is not provide in the argument, change this to dot ".", if exist, change this to the path
    file_name: OsString,
    // This is not always absolute path
    base_path: PathBuf,
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

    // pub fn builder(&self) -> PathBuf {
    //     self.builder.clone()
    // }

    /// Sets the file name to "." (dot), indicating the current directory.
    pub fn set_file_name_to_current_dir(&mut self) {
        self.file_name = ".".into()
    }

    /// Retrieves metadata for the base directory.
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.base_path)
    }

    pub fn build(&self) -> TResult<PathBuilder> {
        Ok(PathBuilder {
            builder: PathBuf::with_capacity(5_000),
            base_dir: BaseDirectory {
                file_name: self.filename(),
                base_path: self.base_path(),
            },
        })
    }
}

pub struct PathBuilder {
    builder: PathBuf,
    base_dir: BaseDirectory,
}

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder {
            builder: PathBuf::with_capacity(5_000),
            base_dir: BaseDirectory::from_current_dir().expect("Cannot initialize BaseDirectory"),
        }
    }

    pub fn append_root(&mut self) {
        self.builder.push(self.base_dir.base_path());
    }

    pub fn append_relative(&mut self, visit: Visitor) -> Self {
        let relative_path = visit.get_relative_path(&self.base_dir.base_path()).unwrap();

        self.builder.push(relative_path);

        Self {
            builder: self.builder.clone(),
            base_dir: self.base_dir.clone(),
        }
    }

    pub fn into_os_string(&self) -> OsString {
        self.builder.clone().into_os_string()
    }

    pub fn pop(&mut self) {
        self.builder.pop();
    }

    pub fn clear(&mut self) {
        self.builder.clear();
    }
}
