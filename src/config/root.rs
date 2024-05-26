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
    pub fn abs_curr_shell() -> TResult<Self> {
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
}
