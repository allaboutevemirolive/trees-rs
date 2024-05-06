use super::app::options;
use super::app::tree_app;
use crate::error::simple::TResult;
use crate::walk::GlobalCtxt;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::Metadata;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TArgs {
    pub args: Vec<OsString>,
}

#[allow(dead_code)]
impl<'a> TArgs {
    pub fn new() -> Self {
        let args: Vec<OsString> = env::args_os().collect();
        TArgs { args }
    }

    fn extract_paths(&self) -> (Vec<&OsString>, Vec<&OsString>) {
        let mut remaining_args = Vec::new();
        let mut paths = Vec::new();

        for arg in &self.args {
            if Path::new(arg).exists() {
                paths.push(arg);
            } else {
                remaining_args.push(arg);
            }
        }

        (remaining_args, paths)
    }

    fn assert_single_path(&self) -> Option<&OsString> {
        let (_, paths) = self.extract_paths();
        if paths.len() == 1 {
            Some(paths[0])
        } else {
            None
        }
    }

    pub fn match_app(&mut self, gcx: &mut GlobalCtxt) -> TResult<(PathBuf, OsString, Metadata)> {
        let path_exist = xpath_exist(&mut self.args, gcx);

        #[allow(unused_assignments)]
        let mut fpath = PathBuf::new();
        #[allow(unused_assignments)]
        let mut fname = OsString::new();

        if path_exist {
            fpath = gcx.rpath.fpath.clone();
            fname = <PathBuf as Clone>::clone(&gcx.rpath.fpath).into();
        } else {
            fpath = gcx.rpath.fpath.clone();
            gcx.rpath.fname = OsString::from("."); // relative-path for entries in the absence of provided path by user
            fname = gcx.rpath.fname.clone(); // header
        }

        let matches = tree_app()
            .try_get_matches_from(self.args.clone())
            .unwrap_or_else(|e| e.exit());

        if matches.contains_id(options::miscellaneous::LEVEL) {
            let level: usize = *matches
                .get_one(options::miscellaneous::LEVEL)
                .expect("default");

            gcx.level.cap = level as i32;
        }

        if matches.get_flag(options::meta::META) {
            gcx.rg.with_permission()?;
            gcx.rg.with_btime()?;
            gcx.rg.with_mtime()?;
            gcx.rg.with_atime()?;
            gcx.rg.with_size_color()?;
        }

        if matches.get_flag(options::sort::REVERSE) {
            gcx.rg.with_reverse_sort_entries()?;
        }

        if matches.get_flag(options::sort::FILEFIRST) {
            gcx.rg.with_sort_by_file_first()?;
        }

        if matches.get_flag(options::color::COLOR) {
            gcx.rg.with_color_entry()?;
        }

        if matches.get_flag(options::path::RELATIVE) {
            gcx.rg.with_color_relative_path()?;
        }

        if matches.get_flag(options::read::VISIBLE) {
            gcx.rg.read_visible_entries()?;
        }

        if matches.get_flag(options::read::ALL) {
            gcx.rg.read_all_entries()?;
        }

        if matches.get_flag(options::read::FOLDER) {
            gcx.rg.read_visible_folders()?;
        }

        if matches.get_flag(options::meta::PERMISSION) {
            gcx.rg.with_permission()?;
        }

        if matches.get_flag(options::meta::BTIME) {
            gcx.rg.with_btime()?;
        }

        if matches.get_flag(options::meta::MTIME) {
            gcx.rg.with_mtime()?;
        }

        if matches.get_flag(options::meta::ATIME) {
            gcx.rg.with_atime()?;
        }

        if matches.get_flag(options::meta::SIZE) {
            gcx.rg.with_size_color()?;
        }

        // This statement should revert any color output into colorless
        if matches.get_flag(options::color::COLORLESS) {
            // Default entries state
            gcx.rg.with_colorless_entry()?;

            if matches.get_flag(options::meta::SIZE) {
                gcx.rg.with_size()?;
            }

            if matches.get_flag(options::meta::META) {
                gcx.rg.with_permission()?;
                gcx.rg.with_btime()?;
                gcx.rg.with_mtime()?;
                gcx.rg.with_atime()?;
                gcx.rg.with_size()?; // no color
            }

            if matches.get_flag(options::path::RELATIVE) {
                gcx.rg.with_relative_path()?;
            }

            if matches.get_flag(options::path::RELATIVE) {
                gcx.rg.with_relative_path()?;
            }
        }

        let fmeta = fs::metadata(fpath.clone())?;

        Ok((fpath.to_path_buf(), fname, fmeta))
    }
}

fn xpath_exist(args: &mut Vec<OsString>, gcx: &mut GlobalCtxt) -> bool {
    let mut delete_index = None;

    for (index, arg) in args.iter().skip(1).enumerate() {
        if let Some(arg_path) = valid_path(arg) {
            gcx.rpath.fpath = arg_path.clone();
            gcx.rpath.fname = arg_path.into_os_string();
            delete_index = Some(index + 1);
            break;
        }
    }

    if let Some(index) = delete_index {
        args.remove(index);
        true
    } else {
        false
    }
}

fn valid_path(arg: &OsString) -> Option<PathBuf> {
    let path = Path::new(arg);
    if path.is_dir() || path.is_file() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_extract_paths() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_dir_path = temp_dir.path();

        let file1_path = temp_dir_path.join("existing.txt");
        let mut file1 = File::create(&file1_path).expect("Failed to create file1");
        writeln!(file1, "Some content").expect("Failed to write to file1");

        let file2_path = temp_dir_path.join("nonexistent.txt");
        let mut file2 = File::create(&file2_path).expect("Failed to create file2");
        writeln!(file2, "Some content").expect("Failed to write to file2");

        let args = vec![OsString::from(file2_path), OsString::from(file1_path)];
        let tree_args = TArgs { args };
        let (remaining, paths) = tree_args.extract_paths();

        assert_eq!(remaining.len(), 0);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_new() {
        let tree_args = TArgs::new();
        assert!(tree_args.args.len() >= 1);
    }

    #[test]
    fn test_assert_single_path() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_dir_path = temp_dir.path();

        let file1_path = temp_dir_path.join("existing.txt");
        let mut file1 = File::create(&file1_path).expect("Failed to create file1");
        writeln!(file1, "Some content").expect("Failed to write to file1");

        let args = vec![OsString::from(file1_path.clone())];
        let tree_args = TArgs { args };
        assert!(tree_args.assert_single_path().is_some());

        let args = vec![
            OsString::from(file1_path.clone()),
            OsString::from(file1_path),
        ];
        let tree_args = TArgs { args };
        assert!(tree_args.assert_single_path().is_none());
    }
}
