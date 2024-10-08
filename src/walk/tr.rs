use crate::config;
use crate::render;
use crate::report;
use crate::tree;
use crate::walk;

// Color trait which allows output to be colorize.
// We use function pointer to generate colorize output
// instead of enum for performance reason.
use crate::config::registry::Color;

use anyhow::Context;

#[derive(Debug)]
pub struct TreeCtxt<'tr, 'a> {
    pub branch: tree::branch::Branch,
    pub buf: &'a mut render::buffer::Buffer<std::io::StdoutLock<'tr>>,
    pub level: tree::level::Level,
    pub nod: tree::node::Node,
    pub rg: config::registry::Registry<'tr>,
    pub dir_stats: report::stats::DirectoryStats,
    pub path_builder: config::root::PathBuilder,
}

impl<'tr, 'a> TreeCtxt<'tr, 'a> {
    pub fn new(
        buf: &'a mut render::buffer::Buffer<std::io::StdoutLock<'tr>>,
    ) -> anyhow::Result<Self> {
        tracing::info!("Initializing TreeCtxt");

        let branch = tree::branch::Branch::default();
        let nod = tree::node::Node::default();
        let dir_stats = report::stats::DirectoryStats::default();
        let level = tree::level::Level::default();
        let rg = config::registry::Registry::new()?;
        let path_builder = config::root::PathBuilder::default();

        Ok(Self {
            branch,
            buf,
            level,
            nod,
            rg,
            dir_stats,
            path_builder,
        })
    }

    pub fn walk_dir(&mut self, path: std::path::PathBuf) -> anyhow::Result<()> {
        tracing::info!("Displaying tree view for directory: {}", path.display());

        let enumerated_entries = self.get_sorted_entries(path)?;
        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            self.process_entry(idx, entry, entries_len)?;
        }

        Ok(())
    }

    fn get_sorted_entries(
        &mut self,
        path: std::path::PathBuf,
    ) -> anyhow::Result<Vec<(usize, std::fs::DirEntry)>> {
        let mut entries = self.rg.inspt_dents(path, &mut self.dir_stats)?;
        self.rg.sort_dents(&mut entries);

        Ok(entries.into_iter().enumerate().collect())
    }

    fn process_entry(
        &mut self,
        idx: usize,
        entry: std::fs::DirEntry,
        entries_len: usize,
    ) -> anyhow::Result<()> {
        tracing::info!("Get entry's information for entry: {:?}", entry);

        let mut visitor = walk::visit::Visitor::new(entry)?;
        self.update_stats_and_print_info(&visitor)?;
        self.update_node(idx, entries_len)?;
        self.handle_entry_type(&mut visitor)?;
        Ok(())
    }

    fn handle_entry_type(&mut self, visitor: &mut walk::visit::Visitor) -> anyhow::Result<()> {
        if visitor.is_symlink() {
            self.handle_symlink(visitor)?;
            self.nod.pop();
            return Ok(());
        } else if visitor.is_media_type() {
            self.handle_media(visitor)?;
            self.nod.pop();
            return Ok(());
        } else if visitor.is_file() {
            self.handle_file(visitor)?;
            self.nod.pop();
            return Ok(());
        } else if visitor.is_dir() {
            self.handle_directory(visitor)?;
            self.nod.pop();
            return Ok(());
        } else {
            self.handle_special(visitor)?;
            self.nod.pop();
            return Ok(());
        }
    }

    fn update_stats_and_print_info(
        &mut self,
        visitor: &walk::visit::Visitor,
    ) -> anyhow::Result<()> {
        // if let Some(size) = visitor.size() {
        self.dir_stats.add_size(visitor.size());
        // }
        self.handle_info(visitor.metadata())?;
        Ok(())
    }

    fn update_node(&mut self, idx: usize, entries_len: usize) -> anyhow::Result<()> {
        self.nod.push_conditional(idx, entries_len);
        self.nod.paint_as_branch(&self.branch, self.buf)?;
        Ok(())
    }

    fn handle_symlink(&mut self, visitor: &mut walk::visit::Visitor) -> anyhow::Result<()> {
        self.dir_stats.symlink_add_one();
        self.rg.yellow(self.buf)?;
        self.buf
            .print_symlink(visitor, &self.path_builder, self.rg.symlink)?;
        self.rg.reset(self.buf)?;
        self.buf.write_message(" @ ")?;
        self.rg.underlined_blue(self.buf)?;

        self.buf.write_message(
            visitor
                .resolve_symlink()
                .context("Failed to get target symlink")?
                .to_str()
                .context("Failed to convert target symlink to &str")?,
        )?;

        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    fn handle_media(&mut self, visitor: &walk::visit::Visitor) -> anyhow::Result<()> {
        self.dir_stats.media_add_one();
        self.rg.purple(self.buf)?;
        self.buf
            .print_file(visitor, &self.path_builder, self.rg.file)?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    fn handle_file(&mut self, visitor: &walk::visit::Visitor) -> anyhow::Result<()> {
        self.dir_stats.file_add_one();
        self.buf
            .print_file(visitor, &self.path_builder, self.rg.file)?;
        self.buf.newline()?;
        Ok(())
    }

    fn handle_directory(&mut self, visitor: &walk::visit::Visitor) -> anyhow::Result<()> {
        self.dir_stats.dir_add_one();
        self.rg.blue(self.buf)?;
        self.buf
            .print_dir(visitor, &self.path_builder, self.rg.dir)?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;

        if self.level.can_descend_further() {
            self.descend_into_directory(visitor)?;
        }
        Ok(())
    }

    fn descend_into_directory(&mut self, visitor: &walk::visit::Visitor) -> anyhow::Result<()> {
        self.level.increment();
        // if let Some(path) = visitor.absolute_path() {
        if self
            .walk_dir(visitor.absolute_path().to_path_buf())
            .is_err()
        {
            self.dir_stats.err_dirs_add_one();
        }
        // }
        self.level.decrement();
        Ok(())
    }

    fn handle_special(&mut self, visitor: &walk::visit::Visitor) -> anyhow::Result<()> {
        self.dir_stats.special_add_one();
        self.rg.bold_red(self.buf)?;
        self.buf.write_os_string(visitor.filename().clone())?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    #[cfg(unix)]
    pub fn handle_header(&mut self) -> anyhow::Result<()> {
        use std::os::unix::fs::MetadataExt;
        tracing::info!("Print directory header");

        let file_name = self.path_builder.filename();
        let base_path = self.path_builder.base_path();
        let fmeta = self.path_builder.metadata()?;

        self.dir_stats.add_size(fmeta.size());
        self.handle_info(&fmeta)?;
        self.rg.blue(self.buf)?;
        self.buf
            .print_header(&fmeta, &base_path, &file_name, self.rg.head)?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    pub fn handle_info(&mut self, meta: &std::fs::Metadata) -> anyhow::Result<()> {
        tracing::info!("Print entry's information");
        self.buf.print_permission(meta, self.rg.pms)?;
        self.buf.print_btime(meta, self.rg.btime)?;
        self.buf.print_mtime(meta, self.rg.mtime)?;
        self.buf.print_atime(meta, self.rg.atime)?;
        self.rg.green(self.buf)?;
        self.buf.print_size(meta, self.rg.size)?;
        self.rg.reset(self.buf)?;
        Ok(())
    }

    pub fn handle_report(&mut self, report_mode: report::stats::ReportMode) -> anyhow::Result<()> {
        tracing::info!("Print reports");
        self.buf.newline()?;
        self.dir_stats.accumulate_items();
        let mut report_summary = report::stats::ReportSummary::with_capacity(50)?;
        self.dir_stats
            .populate_report(&mut report_summary, report_mode);
        let summary = report_summary.join(", ");
        self.buf.write_message(&summary)?;
        self.buf.newline()?;
        Ok(())
    }
}
