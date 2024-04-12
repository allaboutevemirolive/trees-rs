use crate::error::simple::UResult;
use crate::error::simple::USimpleError;
use crate::report::tail::Tail;
use crate::walk::WalkDir;
use std::env;
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;

pub type WhichReader = fn(&Directory, &mut Tail) -> UResult<Vec<DirEntry>>;

pub struct Directory {
    pub path: PathBuf,
}

impl<'pt, 'wd, 'cv, 'cr> Directory {
    pub fn new(path: impl Into<PathBuf>) -> UResult<Self> {
        Ok(Directory { path: path.into() })
    }

    #[allow(unused_variables)]
    pub fn read_all_entries(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .map(|entry_result| entry_result.map(|entry| entry))
            .collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
        Ok(entries)
    }

    pub fn read_visible_entries(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    if !entry.file_name().to_string_lossy().starts_with(".") {
                        Some(entry)
                    } else {
                        tail.hid_plus_one();
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    /// Read visible entries (directories and files) but exclude entry (directory and file) that match the given path.
    pub fn read_visible_excl_path(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let entry_path = entry.path();
                    // TODO: Implement this feature
                    let excluded_path = PathBuf::new();
                    if !entry.file_name().to_string_lossy().starts_with(".")
                        || !(entry_path == excluded_path)
                    {
                        Some(entry)
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    /// Read visible entries(files) that match the given specific file extension.
    pub fn read_visible_ext_files_and_folders(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy().to_string();
                    if file_name_str.starts_with(".") {
                        None
                    } else if entry.path().is_dir() {
                        Some(entry)
                    } else if let Some(extension) = Path::new(&file_name_str).extension() {
                        if extension == "rs" {
                            Some(entry)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    pub fn read_visible_folders(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let metadata = entry.metadata().ok()?;
                    if metadata.is_dir() && !entry.file_name().to_string_lossy().starts_with(".") {
                        Some(entry)
                    } else {
                        tail.hid_plus_one();
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    pub fn read_all_folders(&self) -> UResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let metadata = entry.metadata().ok()?;
                    if metadata.is_dir() {
                        Some(entry)
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    fn inspect_entries(&self, tail: &mut Tail, f: WhichReader) -> UResult<Vec<DirEntry>> {
        f(self, tail)
    }

    /// `Read` the directory's entries, then `sort` and `enumerate` them.
    ///
    /// We must maintain the order of operations because indexes from the enumeration
    /// will be used in `add_entry_marker` to generate tree branches.
    pub fn iterate_entries(
        &self,
        walk: &'pt mut WalkDir<'wd, 'cv, 'cr>,
    ) -> UResult<Vec<(usize, DirEntry)>> {
        // Read
        let mut entries = self
            .inspect_entries(&mut walk.config.report.tail, walk.setting.cr.wr)
            .map_err(|err| {
                USimpleError::new(1, format!("Failed to inspect directory entries: {}", err))
            })?;

        // Sort
        (walk.setting.cr.ws)(&mut entries);

        // Enumerate
        let enumerated_entries = entries.into_iter().enumerate().collect();

        Ok(enumerated_entries)
    }
}

/// If no path where given, retrieve current path where shell executed
pub fn get_absolute_current_shell() -> UResult<OsString> {
    Ok(env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string())
}

// fn extract_paths(args: Vec<String>) -> (Vec<String>, Vec<String>) {
//     let mut remaining_args = Vec::new();
//     let mut paths = Vec::new();

//     for arg in args {
//         if Path::new(&arg).exists() {
//             paths.push(arg);
//         } else {
//             remaining_args.push(arg);
//         }
//     }

//     (remaining_args, paths)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // #[test]
    // fn test_read_entries() {
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     // println!("{:?}", temp_dir);
    //     let directory = Directory::new(temp_dir.path()).unwrap();

    //     // Create some dummy files in the test directory
    //     fs::File::create(directory.path.join("file1.txt")).unwrap();
    //     fs::File::create(directory.path.join("file2.txt")).unwrap();

    //     let entries = directory.read_all_entries().unwrap();
    //     // Expecting at least two entries: "." and ".." directories and possibly the files we created
    //     assert!(entries.len() >= 2);
    // }

    // #[test]
    // fn test_iterate_entries() {
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     let directory = Directory::new(temp_dir.path());

    //     // Create some dummy files in the test directory
    //     fs::File::create(directory.path.join("file1.txt")).unwrap();
    //     fs::File::create(directory.path.join("file2.txt")).unwrap();

    //     let enumerated_entries = directory.iterate_entries().unwrap();
    //     // Expecting at least two entries: "." and ".." directories and possibly the files we created
    //     assert!(enumerated_entries.len() >= 2);

    //     // Iterating over enumerated_entries
    //     for (index, entry) in enumerated_entries {
    //         assert!(entry.file_name().to_string_lossy().contains("file"));
    //     }
    // }

    // #[test]
    // fn test_is_hidden_file() {
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     let directory = Directory::new(temp_dir.path()).unwrap();

    //     // Create a dummy hidden file in the test directory
    //     fs::File::create(directory.path.join(".hidden_file")).unwrap();

    //     let entries = directory.read_all_entries().unwrap();
    //     for entry in entries {
    //         if is_hidden_file(&entry) {
    //             assert_eq!(entry.file_name().to_string_lossy().starts_with('.'), true);
    //         } else {
    //             assert_eq!(entry.file_name().to_string_lossy().starts_with('.'), false);
    //         }
    //     }
    // }

    // cargo test test_test -- --nocapture
    #[test]
    fn test_relative_path() {
        // Get the current directory
        let current_dir = std::env::current_dir().expect("Failed to get current directory");

        // Get the relative paths of files in the current directory
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    // Convert the full path to a relative path
                    let relative_path = path
                        .strip_prefix(&current_dir)
                        .expect("Failed to get relative path")
                        .to_path_buf();
                    println!("./{}", relative_path.display());
                    println!("{}", path.display());
                }
            }
        }
    }

    // cargo test test_extract_paths -- --nocapture
    // #[test]
    // fn test_extract_paths() {
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     let temp_dir_path = temp_dir.path().to_string_lossy().into_owned();

    //     let args = vec![
    //         String::from("arg1"),
    //         temp_dir_path.clone(),
    //         String::from("arg2"),
    //         String::from("./relative/path/to/dir"),
    //         String::from("dir_name"),
    //     ];

    //     let (remaining_args, paths) = extract_paths(args);

    //     assert_eq!(
    //         remaining_args,
    //         vec!["arg1", "arg2", "./relative/path/to/dir", "dir_name"]
    //     );
    //     assert_eq!(paths, vec![temp_dir_path]);
    // }
}
