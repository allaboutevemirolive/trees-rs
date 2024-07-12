use crate::config;
use crate::render;
use crate::report;
use crate::tree;
use crate::walk;

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

        let mut entries: Vec<std::fs::DirEntry> = self.rg.inspt_dents(path, &mut self.dir_stats)?;
        self.rg.sort_dents(&mut entries);

        tracing::info!("Enumerate sorted's DirEntry");
        let enumerated_entries: Vec<(usize, std::fs::DirEntry)> =
            entries.into_iter().enumerate().collect();

        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            tracing::info!("Get entry's information for entry: {:?}", entry);

            let mut visitor = walk::visit::Visitor::new(entry)?;

            self.dir_stats.add_size(visitor.size().unwrap());
            self.print_info(visitor.metadata())?;
            self.nod.push_if(idx, entries_len);
            self.nod.to_branch(&self.branch, self.buf)?;

            if visitor.is_symlink() {
                self.dir_stats.symlink_add_one();
                self.rg.yellow(self.buf)?;
                self.buf
                    .print_symlink(&mut visitor, &self.path_builder, self.rg.symlink)?;
                self.rg.reset(self.buf)?;

                self.buf.write_message(" @ ")?;

                self.rg.underlined_blue(self.buf)?;
                self.buf.write_message(
                    visitor
                        .get_target_symlink()
                        .expect("Cannot get target link.")
                        .to_str()
                        .expect("Cannot convert target symlink to &str"),
                )?;
                self.rg.reset(self.buf)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_media_type() {
                self.dir_stats.media_add_one();
                self.rg.purple(self.buf)?;
                self.buf
                    .print_file(&visitor, &self.path_builder, self.rg.file)?;
                self.rg.reset(self.buf)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_file() {
                self.dir_stats.file_add_one();
                self.buf
                    .print_file(&visitor, &self.path_builder, self.rg.file)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_dir() {
                self.dir_stats.dir_add_one();
                self.rg.blue(self.buf)?;
                self.buf
                    .print_dir(&visitor, &self.path_builder, self.rg.dir)?;
                self.rg.reset(self.buf)?;
                self.buf.newline()?;

                // TODO: Should this be in register?
                if self.level.can_descend_further() {
                    self.level.add_one();
                    // If folder needed permission, we skip it. Safe to use unwrap.
                    if self
                        .walk_dir(visitor.absolute_path().unwrap().clone())
                        .is_err()
                    {
                        self.dir_stats.err_dirs_add_one();
                        self.level.subtract_one();
                        self.nod.pop();
                        continue;
                    }
                    self.level.subtract_one();
                }
                self.nod.pop();
                continue;
            } else {
                // If entry is not dir, file or symlink like:
                // - Special File(Device File, Socket File, Named Pipe (FIFO))
                // - Unix-Specific(Block Device, Character Device)
                self.dir_stats.special_add_one();
                self.rg.bold_red(self.buf)?;
                self.buf.write_os_string(visitor.filename().clone())?;
                self.rg.reset(self.buf)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }
        }

        Ok(())
    }

    #[cfg(unix)]
    pub fn print_head(&mut self) -> anyhow::Result<()> {
        use std::os::unix::fs::MetadataExt;

        tracing::info!("Print directory header");

        let file_name = self.path_builder.filename();
        let base_path = self.path_builder.base_path();
        let fmeta = self.path_builder.metadata()?;

        self.dir_stats.add_size(fmeta.size());

        self.print_info(&fmeta).unwrap();

        self.rg.blue(self.buf)?;
        self.buf
            .print_header(&fmeta, &base_path.clone(), &file_name, self.rg.head)?;
        self.rg.reset(self.buf)?;
        self.buf.newline()?;

        Ok(())
    }

    pub fn print_info(&mut self, meta: &std::fs::Metadata) -> anyhow::Result<()> {
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

    pub fn print_report(&mut self, report_mode: report::stats::ReportMode) -> anyhow::Result<()> {
        tracing::info!("Print reports");
        // TODO: Improve report mode
        self.buf.newline()?;
        self.dir_stats.accumulate_items();
        // Store formatted DirectoryStats here
        let mut report_summary = report::stats::ReportSummary::with_capacity(50).unwrap();
        // Get report
        self.dir_stats
            .populate_report(&mut report_summary, report_mode);
        // Parse report
        let summary = report_summary.join(", ");

        self.buf.write_message(&summary)?;
        self.buf.newline()?;
        Ok(())
    }
}
