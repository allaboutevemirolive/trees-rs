use std::env;
use std::error::Error;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;

use crate::sort::dent::sort_vector_by_name;
use crate::walk::WalkDir;

// let my_method: DirPath<DirEntry> = Directory::read_visible_entries;
pub type DirPath = fn(&Directory) -> Result<Vec<DirEntry>, Box<dyn Error>>;

pub struct Directory {
    pub path: PathBuf,
}

impl<'pt, 'wd, 'cv, 'cr> Directory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Directory { path: path.into() }
    }

    pub fn read_all_entries(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        let entries = fs::read_dir(&self.path)?
            .map(|entry_result| entry_result.map(|entry| entry))
            .collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
        Ok(entries)
    }

    pub fn read_visible_entries(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    if !entry.file_name().to_string_lossy().starts_with(".") {
                        Some(entry)
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    pub fn read_visible_folders(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let metadata = entry.metadata().ok()?;
                    if metadata.is_dir() && !entry.file_name().to_string_lossy().starts_with(".") {
                        Some(entry)
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(entries)
    }

    pub fn read_all_folders(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
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

    fn inspect_entries(&self, f: DirPath) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        f(self)
    }

    /// `Read` the directory's entries, then `sort` and `enumerate` them.
    ///
    /// We must maintain the order of operations because indexes from the enumeration
    /// will be used in `add_entry_marker` to generate tree branches.
    pub fn iterate_entries(
        &self,
        walk: &'pt mut WalkDir<'wd, 'cv, 'cr>,
    ) -> Result<Vec<(usize, DirEntry)>, Box<dyn Error>> {
        // Read
        let mut entries = self.inspect_entries(walk.cr.dp)?;
        // Sort
        (walk.cr.sd.func1)(&mut entries);
        // Enumerate
        let enumerated_entries = entries.into_iter().enumerate().collect();
        Ok(enumerated_entries)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum WhichPath {
    CurrentDir,
    OtherDir,
}

pub fn is_hidden_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

pub fn get_relative_path(entry: &DirEntry, current_dir: &PathBuf) -> Option<PathBuf> {
    let path = entry.path();
    if let Ok(relative_path) = path.strip_prefix(current_dir) {
        Some(relative_path.to_path_buf())
    } else {
        None
    }
}

/// If no path where given, retrieve current path where shell executed
pub fn get_absolute_current_shell() -> String {
    env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert path to str")
        .to_string()
}

fn extract_paths(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut remaining_args = Vec::new();
    let mut paths = Vec::new();

    for arg in args {
        if Path::new(&arg).exists() {
            paths.push(arg);
        } else {
            remaining_args.push(arg);
        }
    }

    (remaining_args, paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_read_entries() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        // println!("{:?}", temp_dir);
        let directory = Directory::new(temp_dir.path());

        // Create some dummy files in the test directory
        fs::File::create(directory.path.join("file1.txt")).unwrap();
        fs::File::create(directory.path.join("file2.txt")).unwrap();

        let entries = directory.read_all_entries().unwrap();
        // Expecting at least two entries: "." and ".." directories and possibly the files we created
        assert!(entries.len() >= 2);
    }

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

    #[test]
    fn test_is_hidden_file() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let directory = Directory::new(temp_dir.path());

        // Create a dummy hidden file in the test directory
        fs::File::create(directory.path.join(".hidden_file")).unwrap();

        let entries = directory.read_all_entries().unwrap();
        for entry in entries {
            if is_hidden_file(&entry) {
                assert_eq!(entry.file_name().to_string_lossy().starts_with('.'), true);
            } else {
                assert_eq!(entry.file_name().to_string_lossy().starts_with('.'), false);
            }
        }
    }

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
    #[test]
    fn test_extract_paths() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let temp_dir_path = temp_dir.path().to_string_lossy().into_owned();

        let args = vec![
            String::from("arg1"),
            temp_dir_path.clone(),
            String::from("arg2"),
            String::from("./relative/path/to/dir"),
            String::from("dir_name"),
        ];

        let (remaining_args, paths) = extract_paths(args);

        assert_eq!(
            remaining_args,
            vec!["arg1", "arg2", "./relative/path/to/dir", "dir_name"]
        );
        assert_eq!(paths, vec![temp_dir_path]);
    }
}
