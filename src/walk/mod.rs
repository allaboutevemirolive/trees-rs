use crate::canva::buffer;
use crate::canva::Canva;
use crate::cli::opt::Setting;
use crate::config::path::Directory;
use crate::error::simple::TResult;
use crate::report::tail;
use crate::report::Report;
use crate::tree::branch;
use crate::tree::level;
use crate::tree::node;
use crate::tree::Tree;

pub mod metada;
use self::metada::Visitor;

use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct WalkDir<'wd, 'cv, 'st> {
    pub config: &'wd mut Config<'cv>,
    pub root: &'wd PathBuf,
    pub parent: &'wd OsString,
    pub setting: Setting<'st>,
}

#[derive(Debug)]
pub struct Config<'cv> {
    pub tree: Tree,
    pub canva: Canva<'cv>,
    pub report: Report,
}

impl<'cv> Config<'cv> {
    pub fn new(tree: Tree, canva: Canva<'cv>, report: Report) -> TResult<Self> {
        Ok(Self {
            tree,
            canva,
            report,
        })
    }
}

impl<'wd, 'cv: 'st, 'st: 'cv> WalkDir<'wd, 'cv, 'st> {
    pub fn new(
        config: &'wd mut Config<'cv>,
        root: &'wd PathBuf,
        parent: &'wd OsString,
        setting: Setting<'st>,
    ) -> TResult<Self> {
        Ok(Self {
            config,
            root,
            parent,
            setting,
        })
    }

    pub fn walk_dir(&mut self, path: Directory) -> TResult<()> {
        // Read, sort and enumerate entries
        let entries: Vec<(usize, std::fs::DirEntry)> = Directory::iterate_entries(&path, self)?;
        let entries_len = entries.len();

        for (idx, entry) in entries {
            // Collect metadata
            let visitor = Visitor::new(entry, &self.config.tree.level)?;

            // Accumulate size
            tail::Tail::add_size(&mut self.config.report.tail, visitor.size);

            // Print entry's metadata
            Visitor::print_meta(&visitor.meta, self)?;

            // Mark node based on idx of current entries and entries's len
            node::Node::mark_entry(&mut self.config.tree.nod, idx, entries_len);

            // Print branch based on marked node
            for (is_one, has_next) in self.config.tree.nod.into_iter() {
                // Print branch's structure for current entry
                branch::Branch::paint_branch(
                    &self.config.tree.branch,
                    is_one,
                    has_next,
                    &mut self.config.canva.buffer,
                )?;
            }

            if visitor.filety.is_dir() {
                tail::Tail::dir_plus_one(&mut self.config.report.tail);

                buffer::Buffer::paint_entry(
                    &mut self.config.canva.buffer,
                    &visitor,
                    self.root,
                    self.parent,
                    self.setting.cr.dir,
                )?;

                buffer::Buffer::write_newline(&mut self.config.canva.buffer)?;

                // Check depth-bound based on user preference. Default: 5000.
                self.is_traversable(&visitor)?;
            } else {
                // Accumulate entries
                tail::Tail::file_plus_one(&mut self.config.report.tail);

                // Paint entry's name
                buffer::Buffer::paint_entry(
                    &mut self.config.canva.buffer,
                    &visitor,
                    self.root,
                    self.parent,
                    self.setting.cr.file,
                )?;

                buffer::Buffer::write_newline(&mut self.config.canva.buffer)?;
            }

            // Pop the last node's element.
            node::Node::pop(&mut self.config.tree.nod);
        }
        Ok(())
    }

    fn is_traversable(&mut self, visitor: &Visitor) -> TResult<()> {
        if self.config.tree.level.lvl < self.config.tree.level.cap {
            // Preparing to traverse next directory's depth
            level::Level::plus_one(&mut self.config.tree.level);

            let mut walk = WalkDir::new(self.config, self.root, self.parent, self.setting.clone())?;

            // Get next directory's path for traversing
            let path: Directory = Directory::new(&visitor.abs)?;

            // Traversing
            WalkDir::walk_dir(&mut walk, path)?;

            // Indicates that we return to the current directory after traversing
            level::Level::minus_one(&mut self.config.tree.level);
        }
        Ok(())
    }

    pub fn report(&mut self) -> TResult<()> {
        buffer::Buffer::write_newline(&mut self.config.canva.buffer)?;
        // Get summarize
        let report = Report::get_tail(&self.config.report);
        // Print summarize
        buffer::Buffer::write_report(&mut self.config.canva.buffer, report)?;
        buffer::Buffer::write_newline(&mut self.config.canva.buffer)?;
        Ok(())
    }
}
