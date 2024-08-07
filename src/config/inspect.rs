use crate::report::stats::DirectoryStats;

use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

pub type FnReadDir = fn(PathBuf, &mut DirectoryStats) -> anyhow::Result<Vec<DirEntry>>;

pub fn read_all_entries(
    path: PathBuf,
    _dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;

    Ok(entries)
}

pub fn read_visible_entries(
    path: PathBuf,
    dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry_result| {
            entry_result.ok().and_then(|entry| {
                if !entry.file_name().to_string_lossy().starts_with('.') {
                    Some(entry)
                } else {
                    dir_stats.hidden_add_one();
                    None
                }
            })
        })
        .collect();
    Ok(entries)
}

// anyhow::Result<()>
pub fn read_visible_folders(
    path: PathBuf,
    dir_stats: &mut DirectoryStats,
) -> anyhow::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry_result| {
            entry_result.ok().and_then(|entry| {
                let metadata = entry.metadata().ok()?;
                if metadata.is_dir() && !entry.file_name().to_string_lossy().starts_with('.') {
                    Some(entry)
                } else {
                    dir_stats.hidden_add_one();
                    None
                }
            })
        })
        .collect();
    Ok(entries)
}

#[allow(dead_code)]
pub fn read_all_folders(path: PathBuf) -> anyhow::Result<Vec<DirEntry>> {
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

#[cfg(test)]
mod tests {

    use std::fs;
    // cargo test test_test -- --nocapture
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
