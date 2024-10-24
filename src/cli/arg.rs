use super::app::cli_options;
use super::app::tree_app;

use crate::config::root::TraversalBase;
use crate::report::stats::ReportMode;
use crate::walk::tr::TreeCtxt;

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct TreeArgs {
    args: Vec<OsString>,
}

impl TreeArgs {
    pub fn new() -> Self {
        tracing::info!("Initializing TreeArguments");
        let args: Vec<OsString> = env::args_os().collect();
        Self { args }
    }

    pub fn match_app(
        &mut self,
        tr: &mut TreeCtxt,
        base_dir: &mut TraversalBase,
    ) -> anyhow::Result<ReportMode> {
        tracing::info!("Filter arguments and get report mode");
        self.handle_base_directory(base_dir);

        let matches = self.get_argument_matches()?;
        self.apply_level_settings(tr, &matches);
        self.apply_meta_settings(tr, &matches)?;
        self.apply_sort_settings(tr, &matches)?;
        self.apply_path_settings(tr, &matches)?;
        self.apply_read_settings(tr, &matches)?;
        self.apply_display_settings(tr, &matches)?;

        Ok(self.determine_report_mode(&matches))
    }

    fn handle_base_directory(&mut self, base_dir: &mut TraversalBase) {
        let path_exist = self.extract_and_update_base_dir(base_dir);
        if !path_exist {
            base_dir.set_from_args(false);
            base_dir.set_file_name_to_current_dir();
        } else {
            base_dir.set_from_args(true);
        }
    }

    fn get_argument_matches(&self) -> clap::error::Result<clap::ArgMatches> {
        tree_app().try_get_matches_from(self.args.clone())
    }

    fn apply_level_settings(&self, tr: &mut TreeCtxt, matches: &clap::ArgMatches) {
        if let Some(level) = matches.get_one::<usize>(cli_options::misc::LEVEL) {
            tr.level.set_capacity(*level as u32);
        }
    }

    fn apply_meta_settings(
        &self,
        tr: &mut TreeCtxt,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        if matches.get_flag(cli_options::meta::META) {
            tr.rg.with_all_metadata();
        } else {
            // Safe to set this by default as we will iterate user preferences.
            tr.rg.without_metadata();
        }

        // Individual meta settings
        if matches.get_flag(cli_options::meta::PERMISSION) {
            tr.rg.with_permission();
        }
        if matches.get_flag(cli_options::meta::BTIME) {
            tr.rg.with_birth_time();
        }
        if matches.get_flag(cli_options::meta::MTIME) {
            tr.rg.with_modification_time();
        }
        if matches.get_flag(cli_options::meta::ATIME) {
            tr.rg.with_access_time();
        }
        if matches.get_flag(cli_options::meta::SIZE) {
            tr.rg.with_size();
        }

        Ok(())
    }

    fn apply_sort_settings(
        &self,
        tr: &mut TreeCtxt,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        if matches.get_flag(cli_options::sort::REVERSE) {
            tr.rg.with_reverse_sort_entries()?;
        }
        if matches.get_flag(cli_options::sort::FILES_FIRST) {
            tr.rg.with_sort_by_file_first()?;
        }
        if matches.get_flag(cli_options::sort::SORT) {
            tr.rg.with_sort_entries()?;
        }

        Ok(())
    }

    fn apply_path_settings(
        &self,
        tr: &mut TreeCtxt,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        if matches.get_flag(cli_options::path::RELATIVE) {
            tr.rg.with_relative_paths();
        }
        // TODO: Implement absolute path handling
        Ok(())
    }

    fn apply_read_settings(
        &self,
        tr: &mut TreeCtxt,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        if matches.get_flag(cli_options::read::VISIBLE) {
            tr.rg.read_visible_entries()?;
        }
        if matches.get_flag(cli_options::read::ALL) {
            tr.rg.read_all_entries()?;
        }
        if matches.get_flag(cli_options::read::FOLDER) {
            tr.rg.read_visible_folders()?;
        }
        Ok(())
    }

    fn apply_display_settings(
        &self,
        tr: &mut TreeCtxt,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        if matches.get_flag(cli_options::color::COLOR) {
            tr.rg.with_color()?;
        }
        if matches.get_flag(cli_options::branch::NO_BRANCH) {
            tr.branch.no_branch();
        }
        if matches.get_flag(cli_options::color::NO_COLOR) {
            tr.rg.with_no_color()?;
        }
        Ok(())
    }

    fn determine_report_mode(&self, matches: &clap::ArgMatches) -> ReportMode {
        if matches.get_flag(cli_options::report::YIELD) {
            ReportMode::Exhaustive
        } else {
            ReportMode::Default
        }
    }

    fn extract_and_update_base_dir(&mut self, base_dir: &mut TraversalBase) -> bool {
        let mut delete_index = None;

        for (index, arg) in self.args.iter().skip(1).enumerate() {
            if let Some(arg_path) = self.valid_path(arg) {
                base_dir.set_base_path(arg_path.clone());
                base_dir.set_file_name(arg_path.into_os_string());
                delete_index = Some(index + 1);
                break;
            }
        }

        if let Some(index) = delete_index {
            self.args.remove(index);
            true
        } else {
            false
        }
    }

    fn valid_path(&self, arg: &OsString) -> Option<PathBuf> {
        let path = Path::new(arg);
        if path.is_dir() || path.is_file() {
            Some(path.to_path_buf())
        } else {
            None
        }
    }
}
