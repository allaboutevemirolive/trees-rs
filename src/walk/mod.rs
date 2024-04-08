use crate::{
    canva::*, cli::opt::Setting, config::path::Directory, error::simple::UResult, report::*,
    tree::Tree,
};

use std::{ffi::OsString, path::PathBuf};

pub mod metada;
use self::metada::*;

#[derive(Debug)]
pub struct WalkDir<'wd, 'cv, 'st> {
    pub opts: WalkDirOption,
    pub config: &'wd mut WalkDirConfig<'cv>,
    pub root: &'wd PathBuf,
    pub parent: &'wd OsString,
    pub setting: Setting<'st>,
}

#[derive(Debug)]
pub struct WalkDirConfig<'cv> {
    pub tree: Tree,
    pub canva: Canva<'cv>,
    pub report: Report,
}

impl<'cv> WalkDirConfig<'cv> {
    pub fn new(tree: Tree, canva: Canva<'cv>, report: Report) -> UResult<Self> {
        Ok(Self {
            tree,
            canva,
            report,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WalkDirOption {
    pub flag: i32,
}

impl<'wd, 'cv: 'st, 'st: 'cv> WalkDir<'wd, 'cv, 'st> {
    pub fn new(
        opts: WalkDirOption,
        config: &'wd mut WalkDirConfig<'cv>,
        root: &'wd PathBuf,
        parent: &'wd OsString,
        setting: Setting<'st>,
    ) -> UResult<Self> {
        Ok(Self {
            opts,
            config,
            root,
            parent,
            setting,
        })
    }

    pub fn walk_dir(&mut self, path: Directory) -> UResult<()> {
        let entries = path.iterate_entries(self)?;
        let entries_len = entries.len();

        for (idx, entry) in entries {
            let meta = FileMetadata::new(entry, &self.config.tree.level)?;

            // Print file attributes
            self.config
                .canva
                .buffer
                .paint_attribute(&meta, self.setting.cr.wa)?;

            self.config
                .canva
                .buffer
                .paint_date(&meta, self.setting.cr.wd)?;

            self.config.tree.nod.mark_entry(idx, entries_len);

            for (value, has_next) in self.config.tree.nod.into_iter() {
                self.config.tree.branch.paint_branch(
                    value,
                    has_next,
                    &mut self.config.canva.buffer,
                )?;
            }

            // Print filename
            meta.paint_entry(self)?;

            self.config.tree.nod.pop();
        }
        Ok(())
    }

    pub fn report(&mut self) -> UResult<()> {
        self.config.canva.buffer.write_newline()?;
        let report = self.config.report.get_tail();
        self.config.canva.buffer.write_report(report)?;
        self.config.canva.buffer.write_newline()?;
        Ok(())
    }
}
