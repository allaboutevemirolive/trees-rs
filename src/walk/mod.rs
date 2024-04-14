use crate::canva::buffer;
use crate::canva::Canva;
use crate::cli::opt::Setting;
use crate::config::path::Directory;
use crate::error::simple::UResult;
use crate::report::tail;
use crate::report::Report;
use crate::tree::branch;
use crate::tree::node;
use crate::tree::Tree;

pub mod metada;
use self::metada::FileMetadata;

use std::ffi::OsString;
use std::path::PathBuf;

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
        // Read, sort and enumerate entries
        let entries: Vec<(usize, std::fs::DirEntry)> = Directory::iterate_entries(&path, self)?;
        let entries_len = entries.len();

        for (idx, entry) in entries {
            // Collect metadata
            let fmeta = FileMetadata::new(entry, &self.config.tree.level)?;

            // Accumulate size for all entries
            tail::Tail::add_size(&mut self.config.report.tail, fmeta.size);

            // Print entry's permission
            buffer::Buffer::paint_permission(
                &mut self.config.canva.buffer,
                &fmeta.meta,
                self.setting.cr.pms,
            )?;

            // Print entry's creation-date
            buffer::Buffer::paint_btime(
                &mut self.config.canva.buffer,
                &fmeta.meta,
                self.setting.cr.btime,
            )?;

            buffer::Buffer::paint_mtime(
                &mut self.config.canva.buffer,
                &fmeta.meta,
                self.setting.cr.mtime,
            )?;

            buffer::Buffer::paint_atime(
                &mut self.config.canva.buffer,
                &fmeta.meta,
                self.setting.cr.atime,
            )?;

            // Print entry's size
            buffer::Buffer::paint_size(
                &mut self.config.canva.buffer,
                &fmeta.meta,
                self.setting.cr.size,
            )?;

            // Mark node based on idx of current entries and entries's len
            node::Node::mark_entry(&mut self.config.tree.nod, idx, entries_len);

            // Print branch based on node
            for (value, has_next) in self.config.tree.nod.into_iter() {
                // Print branch's structure for current entry
                branch::Branch::paint_branch(
                    &self.config.tree.branch,
                    value,
                    has_next,
                    &mut self.config.canva.buffer,
                )?;
            }

            // Print entry's name
            FileMetadata::paint_entry(&fmeta, self)?;

            // Pop last node's element
            node::Node::pop(&mut self.config.tree.nod);
        }
        Ok(())
    }

    pub fn report(&mut self) -> UResult<()> {
        buffer::Buffer::write_newline(&mut self.config.canva.buffer)?;
        // Get summarize
        let report = Report::get_tail(&self.config.report);
        // Print summarize
        buffer::Buffer::write_report(&mut self.config.canva.buffer, report)?;
        buffer::Buffer::write_newline(&mut self.config.canva.buffer)?;
        Ok(())
    }
}
