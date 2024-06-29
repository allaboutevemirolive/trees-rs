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

        let is_media: bool;

        if let Some(ext) = dent.path().extension().and_then(OsStr::to_str) {
            is_media = MEDIA_EXTENSIONS.contains(ext);
        } else {
            is_media = false;
        }

        // dbg!("Hello");

        Ok(Self {
            abs: Some(dent.path()),
            dent,
            filety,
            meta,
            filename,
            size: Some(size),
            is_media,
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

    pub fn is_media_type(&self) -> bool {
        self.is_media
    }

    // pub fn check_media_type(&mut self) {
    //     if let Some(ext) = self
    //         .absolute_path()
    //         .expect("Cannot get absolute path")
    //         .extension()
    //         .and_then(OsStr::to_str)
    //     {
    //         self.is_media = MEDIA_EXTENSIONS.contains(ext);
    //     } else {
    //         self.is_media = false;
    //     }
    // }

    pub fn get_target_symlink(&self) -> Result<PathBuf, TSimpleError> {
        if !self.is_symlink() {
            return Err(TSimpleError::new(2, "Visitor is not a symlink".to_string()));
        }

        fs::read_link(self.absolute_path().expect("Invalid absolute path."))
            .map_err(|_| TSimpleError::new(2, "Cannot read target link to symlink"))
    }
}
