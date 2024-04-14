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

#[allow(unused_variables)]
#[allow(dead_code)]
impl<'pt, 'wd, 'cv, 'cr> Directory {
    pub fn new(path: impl Into<PathBuf>) -> UResult<Self> {
        Ok(Directory { path: path.into() })
    }

    pub fn read_all_entries(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
        let entries =
            fs::read_dir(&self.path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;

        Ok(entries)
    }

    pub fn read_visible_entries(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
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
    pub fn read_visible_excl_path(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
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
    pub fn read_visible_ext_files_and_folders(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
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

    pub fn read_visible_folders(&self, tail: &mut Tail) -> UResult<Vec<DirEntry>> {
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
            .inspect_entries(&mut walk.config.report.tail, walk.setting.cr.read)
            .map_err(|err| {
                USimpleError::new(1, format!("Failed to inspect directory entries: {}", err))
            })?;

        // Sort
        (walk.setting.cr.sort)(&mut entries);

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

#[cfg(test)]
mod tests {
    // use super::*;
    use std::fs;
    // use tempfile::tempdir;

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
