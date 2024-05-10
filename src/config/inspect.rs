use crate::error::simple::TResult;
use crate::report::tail::Tail;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

pub type FnReadDir = fn(PathBuf, &mut Tail) -> TResult<Vec<DirEntry>>;

#[allow(unused_variables)]
pub fn read_all_entries(path: PathBuf, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;

    Ok(entries)
}

pub fn read_visible_entries(path: PathBuf, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
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

pub fn read_visible_folders(path: PathBuf, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
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

#[allow(dead_code)]
pub fn read_all_folders(path: PathBuf) -> TResult<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
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
