use super::app::options;
use super::app::tree_app;
use crate::cli::opt::Setting;
use crate::error::simple::TResult;
use crate::walk::Config;

use std::env;
use std::ffi::OsString;
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

    pub fn match_app(
        &mut self,
        setting: &mut Setting<'a>,
        config: &mut Config,
    ) -> TResult<(PathBuf, OsString)> {
        let path_exist = path_exist(&mut self.args, setting);

        #[allow(unused_assignments)]
        let mut path = &PathBuf::new();
        #[allow(unused_assignments)]
        let mut root_filename = OsString::new();

        if path_exist {
            path = &setting.path;
            root_filename = <PathBuf as Clone>::clone(&setting.path).into();
        } else {
            path = &setting.path;
            root_filename = OsString::from(".");
        }

        let matches = tree_app()
            .try_get_matches_from(self.args.clone())
            .unwrap_or_else(|e| e.exit());

        if matches.contains_id(options::miscellaneous::LEVEL) {
            let level: usize = *matches
                .get_one(options::miscellaneous::LEVEL)
                .expect("default");

            config.tree.level.cap = level as i32;
        }

        if matches.get_flag(options::meta::META) {
            setting.cr.with_permission()?;
            setting.cr.with_btime()?;
            setting.cr.with_mtime()?;
            setting.cr.with_atime()?;
            setting.cr.with_size_color()?;
        }

        if matches.get_flag(options::sort::REVERSE) {
            setting.cr.with_reverse_sort_entries()?;
        }

        if matches.get_flag(options::sort::FILEFIRST) {
            setting.cr.with_sort_by_file_first()?;
        }

        if matches.get_flag(options::color::COLOR) {
            setting.cr.with_color_entry()?;
        }

        if matches.get_flag(options::path::RELATIVE) {
            setting.cr.with_relative_path()?;
        }

        if matches.get_flag(options::read::VISIBLE) {
            setting.cr.read_visible_entries()?;
        }

        if matches.get_flag(options::read::ALL) {
            setting.cr.read_all_entries()?;
        }

        if matches.get_flag(options::read::FOLDER) {
            setting.cr.read_visible_folders()?;
        }

        if matches.get_flag(options::meta::PERMISSION) {
            setting.cr.with_permission()?;
        }

        if matches.get_flag(options::meta::BTIME) {
            setting.cr.with_btime()?;
        }

        if matches.get_flag(options::meta::MTIME) {
            setting.cr.with_mtime()?;
        }

        if matches.get_flag(options::meta::ATIME) {
            setting.cr.with_atime()?;
        }

        if matches.get_flag(options::meta::SIZE) {
            setting.cr.with_size_color()?;
        }

        // This statement should revert any color output into colorless
        if matches.get_flag(options::color::COLORLESS) {
            setting.cr.with_colorless_entry()?;

            if matches.get_flag(options::meta::SIZE) {
                setting.cr.with_size()?;
            }

            // If user provide '-m'
            if matches.get_flag(options::meta::META) {
                setting.cr.with_size()?;
            }
        }

        Ok((path.to_path_buf(), root_filename))
    }
}

fn path_exist(args: &mut Vec<OsString>, setting: &mut Setting) -> bool {
    let mut delete_index = None;

    for (index, arg) in args.iter().skip(1).enumerate() {
        if let Some(arg_path) = valid_path(arg) {
            setting.path = arg_path;
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
