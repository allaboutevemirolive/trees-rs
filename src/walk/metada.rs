use super::buffer;
use super::tail;
use super::WalkDir;
use crate::config::path::Directory;
use crate::error::simple::UResult;
use crate::tree::level::Level;
use std::ffi::OsString;
use std::fs::{DirEntry, FileType, Metadata};
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FileMetadata {
    pub abs: PathBuf,
    pub dent: DirEntry,
    pub meta: Metadata,
    pub name: OsString,
    pub filety: FileType,
    pub lvl: Level,
    pub size: u64,
}

impl<'wd, 'ft, 'cv: 'cr, 'cr: 'cv> FileMetadata {
    pub fn new(dent: DirEntry, level: &Level) -> UResult<Self> {
        let filety = dent.file_type()?;
        let abs = dent.path();
        let name = dent
            .path()
            .file_name()
            .map(|os_str| os_str.to_os_string())
            .unwrap();

        let meta = dent.metadata()?;

        let size = meta.len();

        Ok(Self {
            abs,
            dent,
            meta,
            filety,
            name,
            lvl: *level,
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

    pub fn paint_entry(&self, walk: &'ft mut WalkDir<'wd, 'cv, 'cr>) -> UResult<()> {
        if self.filety.is_dir() {
            buffer::Buffer::paint_entry(
                &mut walk.config.canva.buffer,
                self,
                walk.root,
                walk.parent,
                walk.setting.cr.dir,
            )?;

            buffer::Buffer::write_newline(&mut walk.config.canva.buffer)?;
            tail::Tail::dir_plus_one(&mut walk.config.report.tail);
            Level::plus_one(&mut walk.config.tree.level);

            let mut walk = WalkDir::new(walk.config, walk.root, walk.parent, walk.setting.clone())?;
            let path: Directory = Directory::new(&self.abs)?;

            WalkDir::walk_dir(&mut walk, path)?;
            Level::minus_one(&mut walk.config.tree.level);
        } else {
            buffer::Buffer::paint_entry(
                &mut walk.config.canva.buffer,
                self,
                walk.root,
                walk.parent,
                walk.setting.cr.file,
            )?;

            buffer::Buffer::write_newline(&mut walk.config.canva.buffer)?;
            tail::Tail::file_plus_one(&mut walk.config.report.tail);
        }
        Ok(())
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
