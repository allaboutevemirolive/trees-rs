use crate::error::simple::TResult;
use crate::error::simple::TSimpleError;

use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::fs::Metadata;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Visitor {
    pub abs: PathBuf,
    pub dent: DirEntry,
    pub filety: FileType,
    pub meta: Metadata,
    pub filename: OsString,
    pub size: u64,
}

impl Visitor {
    pub fn new(dent: DirEntry) -> TResult<Self> {
        let filety = dent.file_type()?;
        let abs = dent.path();
        let filename = dent
            .path()
            .file_name()
            .map(|os_str| os_str.to_os_string())
            .expect("Failed to get file name");

        let meta = dent.metadata()?;

        let size = meta.len();

        Ok(Self {
            abs,
            dent,
            filety,
            meta,
            filename,
            size,
        })
    }

    pub fn get_relative_path(&self, current_dir: &PathBuf) -> Option<PathBuf> {
        let path = self.dent.path();
        if let Ok(relative_path) = path.strip_prefix(current_dir) {
            Some(relative_path.to_path_buf())
        } else {
            None
        }
    }

    pub fn is_symlink(&self) -> bool {
        self.filety.is_symlink()
    }

    pub fn is_dir(&self) -> bool {
        self.filety.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.filety.is_file()
    }

    pub fn get_target_symlink(&mut self) -> Result<PathBuf, TSimpleError> {
        if !self.is_symlink() {
            return Err(TSimpleError {
                code: 1,
                message: "Visitor is not a symlink".to_string(),
            });
        }

        if let Ok(link_target) = fs::read_link(self.abs.clone()) {
            Ok(link_target)
        } else {
            Err(TSimpleError::new(1, "Cannot read target link to symlink"))
        }
    }
}

#[cfg(test)]
mod metada_test {

    #[allow(unused_imports)]
    use super::*;
    use std::fs::{self, File};
    use std::io;
    #[allow(unused_imports)]
    use std::io::Write;
    #[allow(unused_imports)]
    use std::os::unix::fs::PermissionsExt;
    #[allow(unused_imports)]
    use tempfile::tempdir;

    #[allow(dead_code)]
    fn create_temp_file_with_permissions(
        temp_dir: &tempfile::TempDir,
        permissions: u32,
    ) -> io::Result<DirEntry> {
        let file_path = temp_dir.path().join("test_file.txt");
        let file = File::create(&file_path)?;
        file.set_permissions(fs::Permissions::from_mode(permissions))?;
        let dir_entry = fs::read_dir(temp_dir.path())?.next().unwrap()?;
        Ok(dir_entry)
    }
}
