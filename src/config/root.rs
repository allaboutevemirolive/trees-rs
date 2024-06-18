use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::simple::TResult;

/// Struct that store the path where we needs to start traverse
pub struct RootPath {
    pub fdot: OsString,
    pub fname: OsString,
    pub fpath: PathBuf,
}

impl RootPath {
    pub fn from_current_dir() -> TResult<Self> {
        let path_dir = env::current_dir()
            .expect("Failed to get current directory")
            .into_os_string();

        let mut fpath = PathBuf::new();
        fpath.push(path_dir);
        let fname = fpath
            .file_name()
            .expect("Cannot retrieve file name for the starting point path.")
            .to_os_string();
        let fdot = OsString::from(".");

        Ok(Self { fdot, fname, fpath })
    }

    pub fn filename(&self) -> OsString {
        self.fname.clone()
    }

    pub fn with_filename(&mut self, fname: OsString) {
        self.fname = fname;
    }

    pub fn absolute_path(&self) -> PathBuf {
        self.fpath.clone()
    }
}

// pub struct BaseDirectory {
//     base_path: PathBuf,
//     relative_path: Option<OsString>,
// }

// impl BaseDirectory {
//     pub fn new<P: AsRef<Path>>(path: P) -> Self {
//         Self {
//             base_path: path.as_ref().to_path_buf(),
//             relative_path: None,
//         }
//     }

//     pub fn from_current_dir() -> Result<Self, std::io::Error> {
//         std::env::current_dir().map(Self::new)
//     }

//     pub fn current_path(&self) -> PathBuf {
//         if let Some(entry) = &self.relative_path {
//             self.base_path.join(entry)
//         } else {
//             self.base_path.clone()
//         }
//     }

//     pub fn filename(&self) -> Option<&OsString> {
//         self.relative_path.as_ref()
//     }

//     pub fn enter_dir(&mut self, dir_name: OsString) {
//         self.relative_path = Some(dir_name);
//     }

//     pub fn exit_dir(&mut self) {
//         self.relative_path = None;
//     }
// }
