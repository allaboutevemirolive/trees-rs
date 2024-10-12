use crate::config;
use crate::config::root::WithBasePath;
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
    pub path_builder: config::root::TraversalPathBuilder<WithBasePath>,
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
        let path_builder = config::root::TraversalPathBuilder::default();

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

    #[inline(always)]
    pub fn walk_dir(&mut self, path: std::path::PathBuf) -> anyhow::Result<()> {
        tracing::info!("Displaying tree view for directory: {}", path.display());

        let enumerated_entries = self.get_sorted_entries(path)?;
        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            self.process_entry(idx, entry, entries_len)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn get_sorted_entries(
        &mut self,
        path: std::path::PathBuf,
    ) -> anyhow::Result<Vec<(usize, std::fs::DirEntry)>> {
        let mut entries = self.rg.inspt_dents(path, &mut self.dir_stats)?;
        self.rg.sort_dents(&mut entries);

        Ok(entries.into_iter().enumerate().collect())
    }

    #[inline(always)]
    fn process_entry(
        &mut self,
        idx: usize,
        entry: std::fs::DirEntry,
        entries_len: usize,
    ) -> anyhow::Result<()> {
        tracing::info!("Get entry's information for entry: {:?}", entry);

        let mut file_entry = walk::fent::FileEntry::new(entry)?;
        self.update_stats_and_print_info(&file_entry)?;
        self.update_node(idx, entries_len)?;
        self.handle_entry_type(&mut file_entry)?;
        Ok(())
    }

    #[inline(always)]
    fn handle_entry_type(&mut self, file_entry: &mut walk::fent::FileEntry) -> anyhow::Result<()> {
        if file_entry.is_symlink() {
            self.handle_symlink(file_entry)?;
            self.nod.pop();
            return Ok(());
        } else if file_entry.is_media_type() {
            self.handle_media(file_entry)?;
            self.nod.pop();
            return Ok(());
        } else if file_entry.is_file() {
            self.handle_file(file_entry)?;
            self.nod.pop();
            return Ok(());
        } else if file_entry.is_dir() {
            self.handle_directory(file_entry)?;
            self.nod.pop();
            return Ok(());
        } else {
            self.handle_special(file_entry)?;
            self.nod.pop();
            return Ok(());
        }
    }

    #[inline(always)]
    fn update_stats_and_print_info(
        &mut self,
        file_entry: &walk::fent::FileEntry,
    ) -> anyhow::Result<()> {
        // if let Some(size) = file_entry.size() {
        self.dir_stats.add_size(file_entry.size());
        // }
        self.handle_metadata(file_entry.metadata())?;
        Ok(())
    }

    #[inline(always)]
    fn update_node(&mut self, idx: usize, entries_len: usize) -> anyhow::Result<()> {
        self.nod.push_conditional(idx, entries_len);
        self.nod.paint_as_branch(&self.branch, self.buf)?;
        Ok(())
    }

    #[inline(always)]
    fn handle_symlink(&mut self, file_entry: &mut walk::fent::FileEntry) -> anyhow::Result<()> {
        self.dir_stats.symlink_add_one();
        self.rg.yellow(self.buf)?;
        self.buf
            .print_symlink(file_entry, &self.path_builder, self.rg.entries().symlink())?;
        self.rg.reset(self.buf)?;
        self.buf.write_message(" @ ")?;
        self.rg.underlined_blue(self.buf)?;

        self.buf.write_message(
            file_entry
                .resolve_symlink()
                .context("Failed to get target symlink")?
                .to_str()
                .context("Failed to convert target symlink to &str")?,
        )?;

        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    #[inline(always)]
    fn handle_media(&mut self, file_entry: &walk::fent::FileEntry) -> anyhow::Result<()> {
        self.dir_stats.media_add_one();
        self.rg.purple(self.buf)?;
        self.buf
            .print_file(file_entry, &self.path_builder, self.rg.entries().file())?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    #[inline(always)]
    fn handle_file(&mut self, file_entry: &walk::fent::FileEntry) -> anyhow::Result<()> {
        self.dir_stats.file_add_one();
        self.buf
            .print_file(file_entry, &self.path_builder, self.rg.entries().file())?;
        self.buf.newline()?;
        Ok(())
    }

    #[inline(always)]
    fn handle_directory(&mut self, file_entry: &walk::fent::FileEntry) -> anyhow::Result<()> {
        self.dir_stats.dir_add_one();
        self.rg.blue(self.buf)?;
        self.buf
            .print_dir(file_entry, &self.path_builder, self.rg.entries().dir())?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;

        if self.level.can_descend_further() {
            self.descend_into_directory(file_entry)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn descend_into_directory(&mut self, file_entry: &walk::fent::FileEntry) -> anyhow::Result<()> {
        self.level.increment();
        if self
            .walk_dir(file_entry.absolute_path().to_path_buf())
            .is_err()
        {
            self.dir_stats.err_dirs_add_one();
        }
        self.level.decrement();
        Ok(())
    }

    #[inline(always)]
    fn handle_special(&mut self, file_entry: &walk::fent::FileEntry) -> anyhow::Result<()> {
        self.dir_stats.special_add_one();
        self.rg.bold_red(self.buf)?;
        self.buf.write_os_string(file_entry.filename().clone())?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    #[cfg(unix)]
    #[inline(always)]
    pub fn handle_header(&mut self) -> anyhow::Result<()> {
        tracing::info!("Print directory header");

        use crate::config::root::PathManipulation;
        use std::os::unix::fs::MetadataExt;

        let file_name = self.path_builder.file_name();
        let base_path = self.path_builder.base_path();
        let fmeta = self.path_builder.metadata()?;

        self.dir_stats.add_size(fmeta.size());
        self.handle_metadata(&fmeta)?;
        self.rg.blue(self.buf)?;
        self.buf
            .print_header(&fmeta, &base_path, &file_name, self.rg.entries().head())?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;
        Ok(())
    }

    #[inline(always)]
    pub fn handle_metadata(&mut self, meta: &std::fs::Metadata) -> anyhow::Result<()> {
        tracing::info!("Print entry's information");
        self.buf
            .print_permission(meta, self.rg.metadata().permission())?;
        self.buf
            .print_btime(meta, self.rg.metadata().birth_time())?;
        self.buf.print_mtime(meta, self.rg.metadata().mod_time())?;
        self.buf
            .print_atime(meta, self.rg.metadata().access_time())?;
        self.rg.green(self.buf)?;
        self.buf.print_size(meta, self.rg.metadata().size())?;
        self.rg.reset(self.buf)?;
        Ok(())
    }

    #[inline(always)]
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
