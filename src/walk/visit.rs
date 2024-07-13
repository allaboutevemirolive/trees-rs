use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::fs::Metadata;
use std::path::PathBuf;

// TODO: Add more extension
static MEDIA_EXTENSIONS: phf::Set<&'static str> = phf::phf_set! {
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp",     // Images
    "mp4", "avi", "mkv", "mov", "flv", "wmv", "webm",       // Videos
    "mp3", "wav", "ogg", "flac", "aac", "m4a"             // Audio
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
    pub fn new(dent: DirEntry) -> anyhow::Result<Self> {
        use anyhow::{anyhow, Context};

        tracing::info!("Collect DirEntry's metadata for {:?}", dent.path());

        let metadata = dent.metadata().context("Failed to get file metadata")?;
        let file_type = dent.file_type().context("Failed to get file type")?;
        let path = dent.path();

        let filename = path
            .file_name()
            .map(OsString::from)
            .ok_or_else(|| anyhow!("Cannot get filename for path '{:?}'", path))?;

        let size = metadata.len();

        let is_media = path
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| MEDIA_EXTENSIONS.contains(&ext.to_lowercase()))
            .unwrap_or(false);

        Ok(Self {
            abs: Some(path),
            dent,
            filety: file_type,
            meta: metadata,
            filename,
            size: Some(size),
            is_media,
        })
    }

    pub fn get_relative_path(&self, current_dir: &PathBuf) -> Option<PathBuf> {
        if let Ok(relative_path) = self.dent.path().strip_prefix(current_dir) {
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

    pub fn filename(&self) -> &OsString {
        &self.filename
    }

    pub fn absolute_path(&self) -> Option<&PathBuf> {
        self.abs.as_ref()
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn metadata(&self) -> &Metadata {
        &self.meta
    }

    pub fn is_media_type(&self) -> bool {
        self.is_media
    }

    pub fn get_target_symlink(&self) -> anyhow::Result<PathBuf> {
        use anyhow::Context;
        let path = self
            .absolute_path()
            .ok_or_else(|| anyhow::anyhow!("Invalid absolute path"))?;

        if !self.is_symlink() {
            return Err(anyhow::anyhow!("Path '{path:?}' is not a symlink"));
        }

        fs::read_link(path.clone())
            .with_context(|| format!("Cannot read target link for symlink '{path:?}'"))
    }
}
