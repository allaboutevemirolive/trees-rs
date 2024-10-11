use glob::Pattern;

use crate::report::stats::DirectoryStats;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub type FnReadDir = fn(PathBuf, &mut DirectoryStats) -> anyhow::Result<Vec<DirEntry>>;

/// Helper function to filter hidden entries
#[inline(always)]
fn is_not_hidden(entry: &DirEntry, dir_stats: &mut DirectoryStats) -> bool {
    if entry.file_name().to_string_lossy().starts_with('.') {
        dir_stats.hidden_add_one();
        false
    } else {
        true
    }
}

/// Helper function to check if an entry is a directory
#[inline(always)]
fn is_directory(entry: &DirEntry) -> bool {
    entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
}

/// Read all directory entries
pub fn read_all_entries(
    path: PathBuf,
    _dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;

    Ok(entries)
}

/// Read visible (non-hidden) directory entries
pub fn read_visible_entries(
    path: PathBuf,
    dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry_result| {
            entry_result
                .ok()
                .filter(|entry| is_not_hidden(entry, dir_stats))
        })
        .collect();

    Ok(entries)
}

/// Read visible (non-hidden) directory entries
pub fn read_entries_with_ignore_glob(
    path: PathBuf,
    dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry_result| {
            entry_result
                .ok()
                .filter(|entry| is_not_hidden(entry, dir_stats))
        })
        .collect();

    Ok(entries)
}

/// Read visible (non-hidden) folders only
pub fn read_visible_folders(
    path: PathBuf,
    dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry_result| {
            entry_result
                .ok()
                .filter(|entry| is_directory(entry) && is_not_hidden(entry, dir_stats))
        })
        .collect();

    Ok(entries)
}

/// Read all folders (including hidden)
#[allow(dead_code)]
pub fn read_all_folders(path: PathBuf) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry_result| entry_result.ok().filter(|entry| is_directory(entry)))
        .collect();

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_relative_path() {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");

        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
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
