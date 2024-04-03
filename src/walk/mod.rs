use crate::config::path::Directory;
use crate::config::registry::CallbackRegistry;
use crate::report::*;
use crate::tree::{node, Tree};
use std::path::PathBuf;

pub mod metada;
use self::metada::*;

use crate::canva::*;

// pub type Result<T> = ::std::result::Result<T, Error>;

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
    pub fn new(tree: Tree, canva: Canva<'cv>, report: Report) -> Self {
        Self {
            tree,
            canva,
            report,
        }
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
    ) -> Self {
        Self {
            opts,
            config,
            root,
            cr,
        }
    }

    pub fn walk_dir(&mut self, path: Directory) {
        let entries = path.iterate_entries(self).unwrap();
        let entries_len = entries.len();

        for (idx, entry) in entries {
            self.config.tree.nod.add_entry_marker(idx, entries_len);

            for (value, has_next) in self.config.tree.nod.into_iter() {
                self.config.tree.branch.paint_branch(
                    value,
                    has_next,
                    &mut self.config.canva.buffer,
                );
            }

            FileMetadata::new(entry, &self.root, &self.config.tree.level).which_file_type(self);

            self.config.tree.nod.pop();
        }
    }

    pub fn report(&mut self) {
        self.config.canva.buffer.write_newline().unwrap();
        let report = self.config.report.get_tail();
        self.config.canva.buffer.write_report(report).unwrap();
        self.config.canva.buffer.write_newline().unwrap();
    }
}
