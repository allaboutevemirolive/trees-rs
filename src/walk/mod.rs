use crate::{
    canva::*, config::path::Directory, config::registry::CallbackRegistry, error::simple::UResult,
    report::*, tree::Tree,
};

use std::path::PathBuf;

pub mod metada;
use self::metada::*;

#[derive(Debug)]
pub struct WalkDir<'wd, 'cv, 'cr> {
    pub opts: WalkDirOption,
    pub config: &'wd mut WalkDirConfig<'cv>,
    pub root: &'wd PathBuf,
    pub cr: CallbackRegistry<'cr>,
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

#[derive(Debug)]
pub struct WalkDirOption {
    pub flag: i32,
}

impl<'wd, 'cv: 'cr, 'cr: 'cv> WalkDir<'wd, 'cv, 'cr> {
    pub fn new(
        opts: WalkDirOption,
        config: &'wd mut WalkDirConfig<'cv>,
        root: &'wd PathBuf,
        cr: CallbackRegistry<'cr>,
    ) -> UResult<Self> {
        Ok(Self {
            opts,
            config,
            root,
            cr,
        })
    }

    pub fn walk_dir(&mut self, path: Directory) -> UResult<()> {
        let entries = path.iterate_entries(self)?;
        let entries_len = entries.len();

        for (idx, entry) in entries {
            self.config.tree.nod.mark_entry(idx, entries_len);

            for (value, has_next) in self.config.tree.nod.into_iter() {
                self.config.tree.branch.paint_branch(
                    value,
                    has_next,
                    &mut self.config.canva.buffer,
                )?;
            }

            FileMetadata::new(entry, &self.root, &self.config.tree.level)?.which_file_type(self)?;

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
