use crate::error::simple::TResult;
use crate::error::simple::TSimpleError;

use phf::phf_set;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::fs::Metadata;
use std::path::PathBuf;

static MEDIA_EXTENSIONS: phf::Set<&'static str> = phf_set! {
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp",     // Images
    "mp4", "avi", "mkv", "mov", "flv", "wmv", "webm",       // Videos
    "mp3", "wav", "ogg", "flac", "aac", "m4a"             // Audio
    // "pdf",                                                // Documents
    // "txt", "doc", "docx", "xls", "xlsx", "ppt", "pptx"     // More documents
};

#[derive(Debug)]
pub struct Visitor {
    abs: Option<PathBuf>,
    dent: DirEntry,
    filename: OsString,
    filety: FileType,
    meta: Metadata,
    size: Option<u64>,
    is_media: bool,
}

impl Visitor {
    pub fn new(dent: DirEntry) -> TResult<Self> {
        let filety = dent.file_type()?;

        let filename = dent
            .path()
            .file_name()
            .map(|os_str| os_str.to_os_string())
            .expect("Cannot get filename");

        let meta = dent.metadata()?;

        let size = meta.len();

        Ok(Self {
            abs: Some(dent.path()),
            dent,
            filety,
            meta,
            filename,
            size: Some(size),
            is_media: false,
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

    pub fn filename(&self) -> OsString {
        self.filename.clone()
    }

    pub fn absolute_path(&self) -> Option<PathBuf> {
        self.abs.clone()
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn metadata(&self) -> Metadata {
        self.meta.clone()
    }

    pub fn is_media_type(&mut self) -> bool {
        if let Some(ext) = self
            .absolute_path()
            .expect("Cannot get absolute path")
            .extension()
            .and_then(OsStr::to_str)
        {
            MEDIA_EXTENSIONS.contains(ext)
        } else {
            false
        }
    }

    pub fn get_target_symlink(&self) -> Result<PathBuf, TSimpleError> {
        if !self.is_symlink() {
            return Err(TSimpleError::new(2, "Visitor is not a symlink".to_string()));
        }

        fs::read_link(self.absolute_path().expect("Invalid absolute path."))
            .map_err(|_| TSimpleError::new(2, "Cannot read target link to symlink"))
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
