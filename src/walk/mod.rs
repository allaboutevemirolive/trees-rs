use crate::canva::*;
use crate::cli::opt::Setting;
use crate::config::path::Directory;
use crate::error::simple::UResult;
use crate::report::*;
use crate::tree::Tree;
use std::ffi::OsString;
use std::path::PathBuf;
pub mod metada;
use self::metada::*;

#[derive(Debug)]
pub struct WalkDir<'wd, 'cv, 'st> {
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

impl<'wd, 'cv: 'st, 'st: 'cv> WalkDir<'wd, 'cv, 'st> {
    pub fn new(
        config: &'wd mut WalkDirConfig<'cv>,
        root: &'wd PathBuf,
        parent: &'wd OsString,
        setting: Setting<'st>,
    ) -> UResult<Self> {
        Ok(Self {
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
            let fmeta = FileMetadata::new(entry, &self.config.tree.level)?;

            self.config.report.tail.add_size(fmeta.size);

            // Print entry's permission
            self.config
                .canva
                .buffer
                .paint_permission(&fmeta.meta, self.setting.cr.wa)?;

            // Print entry's creation-date
            self.config
                .canva
                .buffer
                .paint_date(&fmeta.meta, self.setting.cr.wd)?;

            // Mark node
            self.config.tree.nod.mark_entry(idx, entries_len);

            // Print branch based on node
            for (value, has_next) in self.config.tree.nod.into_iter() {
                self.config.tree.branch.paint_branch(
                    value,
                    has_next,
                    &mut self.config.canva.buffer,
                )?;
            }

            // Print entry's name
            fmeta.paint_entry(self)?;

            // Pop last node's element
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
