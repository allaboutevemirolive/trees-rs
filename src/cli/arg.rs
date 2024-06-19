use super::app::options;
use super::app::tree_app;
use crate::config::root::BaseDirectory;
use crate::error::simple::TResult;
use crate::walk::trctxt::TreeCtxt;

use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TreeArgs {
    pub args: Vec<OsString>,
}

#[allow(dead_code)]
impl TreeArgs {
    pub fn new() -> Self {
        let args: Vec<OsString> = env::args_os().collect();
        TreeArgs { args }
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

    pub fn match_app(&mut self, tr: &mut TreeCtxt, base_dir: &mut BaseDirectory) -> TResult<()> {
        let path_exist = extract_and_update_base_dir(&mut self.args, base_dir);

        if !path_exist {
            base_dir.set_path_from_cwd();
            base_dir.set_file_name_to_current_dir();
        } else {
            base_dir.set_path_from_args();
        }

        let matches = tree_app()
            .try_get_matches_from(self.args.clone())
            .unwrap_or_else(|e| e.exit());

        if matches.contains_id(options::miscellaneous::LEVEL) {
            let level: usize = *matches
                .get_one(options::miscellaneous::LEVEL)
                .expect("default");

            tr.level.with_cap(level as i32);
        }

        if matches.get_flag(options::meta::META) {
            tr.rg.with_permission()?;
            tr.rg.with_btime()?;
            tr.rg.with_mtime()?;
            tr.rg.with_atime()?;
            tr.rg.with_size()?;
        }

        if matches.get_flag(options::sort::REVERSE) {
            tr.rg.with_reverse_sort_entries()?;
        }

        if matches.get_flag(options::sort::FILEFIRST) {
            tr.rg.with_sort_by_file_first()?;
        }

        if matches.get_flag(options::color::COLOR) {
            tr.rg.with_entry()?;
        }

        if matches.get_flag(options::path::RELATIVE) {
            tr.rg.with_relative_path()?;
        }

        // TODO
        // if matches.get_flag(options::path::ABSOLUTE) {}

        if matches.get_flag(options::read::VISIBLE) {
            tr.rg.read_visible_entries()?;
        }

        if matches.get_flag(options::read::ALL) {
            tr.rg.read_all_entries()?;
        }

        if matches.get_flag(options::read::FOLDER) {
            tr.rg.read_visible_folders()?;
        }

        if matches.get_flag(options::meta::PERMISSION) {
            tr.rg.with_permission()?;
        }

        if matches.get_flag(options::meta::BTIME) {
            tr.rg.with_btime()?;
        }

        if matches.get_flag(options::meta::MTIME) {
            tr.rg.with_mtime()?;
        }

        if matches.get_flag(options::meta::ATIME) {
            tr.rg.with_atime()?;
        }

        if matches.get_flag(options::meta::SIZE) {
            tr.rg.with_size()?;
        }

        if matches.get_flag(options::branch::NOBRANCH) {
            tr.branch.no_branch();
        }

        if matches.get_flag(options::color::COLORLESS) {
            tr.file_colors.disable_color();
        }

        Ok(())
    }
}

/// By default, Tree-rs detects the first path it finds in the argument.
// TODO: Check if the path if after tree-rs argument, then we skip
// since it maybe not the path we are looking for.
fn extract_and_update_base_dir(args: &mut Vec<OsString>, base_dir: &mut BaseDirectory) -> bool {
    let mut delete_index = None;

    for (index, arg) in args.iter().skip(1).enumerate() {
        if let Some(arg_path) = valid_path(arg) {
            base_dir.with_base_path(arg_path.clone());
            base_dir.with_filename(arg_path.into_os_string());
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
        let tree_args = TreeArgs { args };
        let (remaining, paths) = tree_args.extract_paths();

        assert_eq!(remaining.len(), 0);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_new() {
        let tree_args = TreeArgs::new();
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
        let tree_args = TreeArgs { args };
        assert!(tree_args.assert_single_path().is_some());

        let args = vec![
            OsString::from(file1_path.clone()),
            OsString::from(file1_path),
        ];
        let tree_args = TreeArgs { args };
        assert!(tree_args.assert_single_path().is_none());
    }
}
