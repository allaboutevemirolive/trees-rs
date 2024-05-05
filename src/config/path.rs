use crate::error::simple::TResult;
use crate::report::tail::Tail;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

pub type FnReadDir = fn(&Directory, &mut Tail) -> TResult<Vec<DirEntry>>;

pub struct Directory {
    pub path: PathBuf,
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Directory {
    pub fn new(path: impl Into<PathBuf>) -> TResult<Self> {
        Ok(Directory { path: path.into() })
    }

    pub fn read_all_entries(&self, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
        let entries =
            fs::read_dir(&self.path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;

        Ok(entries)
    }

    pub fn read_visible_entries(&self, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    if !entry.file_name().to_string_lossy().starts_with('.') {
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
    pub fn read_visible_excl_path(&self, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let entry_path = entry.path();
                    // TODO: Implement this feature
                    let excluded_path = PathBuf::new();
                    if !entry.file_name().to_string_lossy().starts_with('.')
                        || entry_path != excluded_path
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
    pub fn read_visible_ext_files_and_folders(&self, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy().to_string();
                    if file_name_str.starts_with('.') {
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

    pub fn read_visible_folders(&self, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
        let entries = fs::read_dir(&self.path)?
            .filter_map(|entry_result| {
                entry_result.ok().and_then(|entry| {
                    let metadata = entry.metadata().ok()?;
                    if metadata.is_dir() && !entry.file_name().to_string_lossy().starts_with('.') {
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

    pub fn read_all_folders(&self) -> TResult<Vec<DirEntry>> {
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

    pub fn inspect_entries(&self, tail: &mut Tail, f: FnReadDir) -> TResult<Vec<DirEntry>> {
        f(self, tail)
    }
}

/// If no path where given, retrieve current path where shell executed
pub fn get_absolute_current_shell() -> TResult<OsString> {
    Ok(env::current_dir()
        .expect("Failed to get current directory")
        .into_os_string())
}

#[cfg(test)]
mod tests {

    use std::fs;
    // cargo test test_test -- --nocapture
    #[test]
    fn test_relative_path() {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");

        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
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
}
