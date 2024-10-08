use std::ffi::{OsStr, OsString};
use std::fs::{self, DirEntry, FileType, Metadata};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use std::collections::HashSet;

const MEDIA_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        // Images
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "heic", "heif", "raw", "cr2", "nef",
        "arw", "dng", "raf", "rw2", "orw", "svg", "psd", "ai", "eps", "pdf", "xcf",
        // Videos
        "mp4", "avi", "mkv", "mov", "flv", "wmv", "webm", "m4v", "mpg", "mpeg", "3gp", "3g2",
        "m2ts", "mts", "ts", "vob", "ogv", "rm", "rmvb", "asf", "divx", "f4v",
        //
        // Audio
        "mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "alac", "aif", "aiff", "ape", "au", "mka",
        "mid", "midi", "pcm", "dsf", "dff", "mpc", "opus", "ra", "tta", "voc", "wv", "m3u", "m3u8",
        "pls", "cue", //
        // 3D and VR
        "fbx", "obj", "stl", "dae", "3ds", "glb", "gltf", //
        // Subtitles
        "srt", "sub", "sbv", "smi", "ssa", "ass", "vtt",
    ]
    .into_iter()
    .collect()
});

/// Represents a file system entry with associated metadata.
#[derive(Debug)]
pub struct FileEntry {
    absolute_path: PathBuf,
    filename: OsString,
    file_type: FileType,
    metadata: Metadata,
    size: u64,
    is_media: bool,
}

impl FileEntry {
    /// Creates a new `FileEntry` instance from a `DirEntry`.
    pub fn new(dir_entry: DirEntry) -> anyhow::Result<Self> {
        let metadata = dir_entry
            .metadata()
            .context("Failed to get file metadata")?;
        let file_type = dir_entry.file_type().context("Failed to get file type")?;
        let absolute_path = dir_entry.path();
        let filename = absolute_path
            .file_name()
            .map(OsString::from)
            .ok_or_else(|| anyhow!("Cannot get filename for path '{:?}'", absolute_path))?;

        let size = metadata.len();
        let is_media = Self::is_media_file(&absolute_path);

        Ok(Self {
            absolute_path,
            filename,
            file_type,
            metadata,
            size,
            is_media,
        })
    }

    /// Returns the relative path from the specified directory.
    pub fn relative_path<P: AsRef<Path>>(&self, current_dir: P) -> Option<PathBuf> {
        self.absolute_path
            .strip_prefix(current_dir)
            .ok()
            .map(PathBuf::from)
    }

    // File type checks
    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }
    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }
    pub fn is_media_type(&self) -> bool {
        self.is_media
    }

    // Getters
    pub fn filename(&self) -> &OsString {
        &self.filename
    }
    pub fn absolute_path(&self) -> &PathBuf {
        &self.absolute_path
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Resolves the target of a symlink.
    pub fn resolve_symlink(&self) -> Result<PathBuf> {
        if !self.is_symlink() {
            return Err(anyhow!("Path '{:?}' is not a symlink", self.absolute_path));
        }
        fs::read_link(&self.absolute_path).with_context(|| {
            format!(
                "Failed to read symlink target for '{:?}'",
                self.absolute_path
            )
        })
    }

    // Private helper method.
    fn is_media_file(path: &Path) -> bool {
        path.extension()
            .and_then(OsStr::to_str)
            .map(|s| s.to_lowercase())
            .map(|ext| MEDIA_EXTENSIONS.contains(ext.as_str()))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::os::unix::fs::symlink;
    use tempfile::tempdir;

    #[test]
    fn test_visitor_creation() -> Result<()> {
        let temp_dir = tempdir()?;
        let test_file_path = temp_dir.path().join("test.txt");
        File::create(&test_file_path)?;

        let dir_entry = fs::read_dir(temp_dir.path())?.next().unwrap()?;
        let file_entry = FileEntry::new(dir_entry)?;

        assert!(file_entry.is_file());
        assert!(!file_entry.is_dir());
        assert!(!file_entry.is_symlink());
        assert!(!file_entry.is_media_type());

        Ok(())
    }

    #[test]
    fn test_media_detection() -> Result<()> {
        let temp_dir = tempdir()?;
        let media_file_path = temp_dir.path().join("test.jpg");
        File::create(&media_file_path)?;

        let dir_entry = fs::read_dir(temp_dir.path())?.next().unwrap()?;
        let file_entry = FileEntry::new(dir_entry)?;

        assert!(file_entry.is_media_type());
        Ok(())
    }

    #[test]
    fn test_symlink_resolution() -> Result<()> {
        let temp_dir = tempdir()?;
        let target_file = temp_dir.path().join("target.txt");
        let symlink_file = temp_dir.path().join("link.txt");

        File::create(&target_file)?;
        symlink(&target_file, &symlink_file)?;

        let dir_entry = fs::read_dir(temp_dir.path())?
            .find(|e| e.as_ref().unwrap().path() == symlink_file)
            .unwrap()?;

        let file_entry = FileEntry::new(dir_entry)?;
        assert!(file_entry.is_symlink());

        let resolved_path = file_entry.resolve_symlink()?;
        assert_eq!(resolved_path, target_file);

        Ok(())
    }
}
