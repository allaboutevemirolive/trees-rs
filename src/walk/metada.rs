use crate::{
    config::path::{get_relative_path, Directory},
    error::simple::{UResult, USimpleError},
    tree::level::Level,
};

use super::{WalkDir, WalkDirOption};

// use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self, DirEntry, FileType};
use std::io::{self};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
// use std::time::SystemTime;

#[derive(Debug)]
pub struct FileMetadata {
    pub absolute_path: PathBuf,
    // pub checksum: OsString,
    // pub creation_date: SystemTime,
    pub dir_entry: DirEntry,
    // pub file_attributes: HashMap<String, bool>,
    pub file_name: OsString,
    // pub file_size: u64,
    pub file_type: FileType,
    // pub last_modified_date: SystemTime,
    pub level: Level,
    // pub metadata_tags: HashMap<String, String>,
    // pub owner: OsString,
    // pub permissions: OsString,
    // pub relative_path: PathBuf,
}

impl<'wd, 'ft, 'cv: 'cr, 'cr: 'cv> FileMetadata {
    // entry.file_type().unwrap() => file_type
    // entry.path() => full_path
    /// `current_dir` is the original shell session where this program is executed.
    pub fn new(dir_entry: DirEntry, current_dir: &PathBuf, level: &Level) -> UResult<Self> {
        let file_type = dir_entry.file_type()?;
        let absolute_path = dir_entry.path();
        // let relative_path = get_relative_path(&dir_entry, current_dir).unwrap_or(PathBuf::new());
        let file_name = dir_entry
            .path()
            .file_name()
            .map(|os_str| os_str.to_os_string())
            .unwrap();

        // let metadata = fs::symlink_metadata(dir_entry.path())?;

        Ok(Self {
            dir_entry,
            file_name,
            // file_size: metadata.len(),
            file_type,
            level: *level,
            // creation_date: SystemTime::now(),
            // last_modified_date: SystemTime::now(),
            // owner: OsString::new(),
            // permissions: OsString::new(),
            // metadata_tags: HashMap::new(),
            // checksum: OsString::new(),
            absolute_path,
            // relative_path,
            // file_attributes: HashMap::new(),
        })
    }

    pub fn get_relative_path(&self, current_dir: &PathBuf) -> Option<PathBuf> {
        let path = self.dir_entry.path();
        if let Ok(relative_path) = path.strip_prefix(current_dir) {
            Some(relative_path.to_path_buf())
        } else {
            None
        }
    }

    // fn with_file_path(mut self, file_path: DirEntry) -> Self {
    //     let full_path = file_path
    //         .path()
    //         .file_name()
    //         .expect("Error retrive filename")
    //         .to_str()
    //         .expect("Error convert OsStr to &str")
    //         .to_string();
    //     self
    // }

    // fn with_file_name(mut self, file_path: PathBuf) -> Self {
    //     if let Some(file_name) = file_path.file_name() {
    //         self.file_name = file_name.to_os_string();
    //     }
    //     self
    // }

    // fn with_file_type(mut self, entry: DirEntry) -> Self {
    //     let file_type = entry.file_type().unwrap();
    //     self.file_type = file_type;
    //     self
    // }

    // fn with_size(mut self, full_path: PathBuf) -> Self {
    //     let metadata = fs::symlink_metadata(&full_path).unwrap();
    //     self.file_size = metadata.len();
    //     self
    // }

    // fn with_permissions(mut self, full_path: &Path) -> Self {
    //     let metadata = fs::metadata(&full_path).unwrap();
    //     let permissions = metadata.permissions();
    //     let mode = permissions.mode();
    //     let octal_perms = format!("{:03o}", mode & 0o777);
    //     self.permissions = OsString::from(octal_perms);
    //     self
    // }

    pub fn get_symbolic_permissions(&self) -> io::Result<OsString> {
        let metadata = self.dir_entry.metadata()?;
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        let file_type = if metadata.is_dir() { 'd' } else { '.' };

        let symbolic_permissions = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            file_type,
            if mode & 0o400 != 0 { 'r' } else { '-' },
            if mode & 0o200 != 0 { 'w' } else { '-' },
            if mode & 0o100 != 0 { 'x' } else { '-' },
            if mode & 0o40 != 0 { 'r' } else { '-' },
            if mode & 0o20 != 0 { 'w' } else { '-' },
            if mode & 0o10 != 0 { 'x' } else { '-' },
            if mode & 0o4 != 0 { 'r' } else { '-' },
            if mode & 0o2 != 0 { 'w' } else { '-' },
            if mode & 0o1 != 0 { 'x' } else { '-' },
        );

        Ok(OsString::from(symbolic_permissions))
    }

    pub fn paint_entry(&self, walk: &'ft mut WalkDir<'wd, 'cv, 'cr>) -> UResult<()> {
        if self.file_type.is_dir() {
            walk.config.canva.buffer.paint_entry(
                &self,
                &walk.root,
                &walk.parent,
                walk.setting.cr.we,
            )?;

            walk.config.canva.buffer.write_newline()?;
            walk.config.report.tail.dir_plus_one();
            walk.config.tree.level.plus_one();

            let walk_opts = WalkDirOption { flag: 1 };
            let mut walk = WalkDir::new(
                walk_opts,
                walk.config,
                walk.root,
                walk.parent,
                walk.setting.clone(),
            )?;
            let path = Directory::new(&self.absolute_path)?;

            walk.walk_dir(path)?;
            walk.config.tree.level.minus_one();
        } else {
            walk.config.canva.buffer.paint_entry(
                &self,
                &walk.root,
                &walk.parent,
                walk.setting.cr.we,
            )?;

            walk.config.canva.buffer.write_newline()?;
            walk.config.report.tail.file_plus_one();
        }
        Ok(())
    }
}

#[cfg(test)]
mod metada_test {

    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

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

    // #[test]
    // fn test_get_symbolic_permissions() {
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     let permissions = 0o755; // rwxr-xr-x
    //     let entry = create_temp_file_with_permissions(&temp_dir, permissions)
    //         .expect("Failed to create temporary file with permissions");

    //     let result = get_symbolic_permissions(&entry).expect("Failed to get symbolic permissions");

    //     assert_eq!(result, "rwxr-xr-x");

    //     temp_dir
    //         .close()
    //         .expect("Failed to close temporary directory");
    // }

    // #[test]
    // fn test_get_symbolic_permissions() {
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     let file_path = temp_dir.path().join("test_file.txt");

    //     let file = File::create(&file_path).expect("Failed to create file");

    //     // Set some permissions
    //     let metadata = file.metadata().expect("Failed to get metadata");
    //     let mut permissions = metadata.permissions();
    //     permissions.set_mode(0o755);
    //     file.set_permissions(permissions)
    //         .expect("Failed to set permissions");

    //     let result = get_symbolic_permissions(file_path.to_str().unwrap())
    //         .expect("Failed to get symbolic permissions");

    //     assert_eq!(result, "rwxr-xr-x");

    //     // Clean up: Delete the temporary directory and its contents
    //     temp_dir
    //         .close()
    //         .expect("Failed to close temporary directory");
    // }

    // use super::*;
    // use std::fs;
    // use std::os::unix::fs::PermissionsExt;
    // use std::time::{Duration, UNIX_EPOCH};

    // #[test]
    // fn test_new() {
    //     let metadata = FileMetadata::new();
    //     assert_eq!(metadata.file_name, "");
    //     assert_eq!(metadata.file_size, 0);
    //     // assert_eq!(metadata.file_type, FileTy::Unknown);
    //     assert_eq!(metadata.owner, "");
    //     assert_eq!(metadata.permissions, "");
    //     assert_eq!(metadata.metadata_tags.len(), 0);
    //     assert_eq!(metadata.checksum, "");
    //     assert_eq!(metadata.file_path, PathBuf::new());
    //     assert_eq!(metadata.file_attributes.len(), 0);
    // }

    // #[test]
    // fn test_with_file_path() {
    //     let mut metadata = FileMetadata::new();
    //     let file_path = PathBuf::from("/path/to/file");
    //     metadata = metadata.with_file_path(file_path.clone());
    //     assert_eq!(metadata.file_path, file_path);
    // }

    // #[test]
    // fn test_with_file_name() {
    //     let mut metadata = FileMetadata::new();
    //     let file_path = PathBuf::from("/path/to/file.txt");
    //     metadata = metadata.with_file_name(file_path.clone());
    //     assert_eq!(metadata.file_name, "file.txt");
    // }

    // #[test]
    // fn test_with_file_type() {
    //     let mut metadata = FileMetadata::new();
    //     let file_path = PathBuf::from("/path/to/file");
    //     let dir_entry = fs::metadata(&file_path).unwrap();
    //     metadata = metadata.with_file_type(dir_entry);
    //     assert_eq!(metadata.file_type, FileTy::File);
    // }

    // #[test]
    // fn test_with_size() {
    //     let mut metadata = FileMetadata::new();
    //     let file_path = PathBuf::from("/path/to/file.txt");
    //     metadata = metadata.with_size(file_path.clone());
    //     let expected_size = fs::metadata(&file_path).unwrap().len();
    //     assert_eq!(metadata.file_size, expected_size);
    // }

    // #[test]
    // fn test_with_permissions() {
    //     let mut metadata = FileMetadata::new();
    //     let file_path = Path::new("/path/to/file.txt");
    //     metadata = metadata.with_permissions(file_path);
    //     let expected_perms = format!(
    //         "{:03o}",
    //         fs::metadata(&file_path).unwrap().permissions().mode() & 0o777
    //     );
    //     assert_eq!(metadata.permissions, expected_perms);
    // }

    // #[test]
    // fn test_with_file_path() {
    //     let sample_path = PathBuf::from("/path/to/sample/file.txt");
    //     let metadata = FileMetadata::new();
    //     let metadata_with_path = metadata.with_file_path(sample_path.clone());
    //     assert_eq!(metadata_with_path.file_path, sample_path);
    // }
}
