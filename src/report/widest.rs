use std::ffi::OsString;
use std::path::PathBuf;

pub struct WidestDirectory {
    pub count: usize,
    pub dirname: OsString,
    pub path: PathBuf,
}
