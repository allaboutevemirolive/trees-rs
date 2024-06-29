use crate::config::registry::Registry;
use crate::config::root::PathBuilder;
use crate::error::simple::TResult;
use crate::render::buffer::Buffer;
use crate::report::stats::DirectoryStats;
use crate::report::stats::ReportMode;
use crate::report::stats::ReportSummary;
use crate::tree::branch::Branch;
use crate::tree::level::Level;
use crate::tree::node::Node;
use crate::walk::visit::Visitor;

use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::io::StdoutLock;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub struct TreeCtxt<'tr> {
    pub branch: Branch,
    pub buf: Buffer<StdoutLock<'tr>>,
    pub level: Level,
    pub nod: Node,
    pub rg: Registry<'tr>,
    pub dir_stats: DirectoryStats,
    pub path_builder: PathBuilder,
}

impl<'tr> TreeCtxt<'tr> {
    pub fn new() -> TResult<Self> {
        let buf = Buffer::new(io::stdout().lock())?;
        let branch = Branch::default();
        let nod = Node::default();
        let dir_stats = DirectoryStats::default();
        let level = Level::default();
        let rg = Registry::new()?;
        let path_builder = PathBuilder::default();

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

    pub fn walk_dir(&mut self, path: PathBuf) -> TResult<()> {
        // Get entries in target path
        let mut entries: Vec<std::fs::DirEntry> = self.rg.inspt_dents(path, &mut self.dir_stats)?;

        self.rg.sort_dents(&mut entries);

        let enumerated_entries: Vec<(usize, std::fs::DirEntry)> =
            entries.into_iter().enumerate().collect();

        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            // Get entry's information
            let mut visitor = Visitor::new(entry)?;
            // Accumulate entry's size
            self.dir_stats
                .add_size(visitor.size().expect("Invalid size."));
            // Print entry's information
            self.print_info(&visitor.metadata())?;
            // If current entry is not the last entry in entries
            self.nod.push_if(idx, entries_len);
            // Convert node to branch's stick
            self.nod.to_branch(&self.branch, &mut self.buf)?;

            if visitor.is_symlink() {
                self.dir_stats.symlink_add_one();
                self.rg.yellow(&mut self.buf)?;
                self.buf
                    .print_symlink(&mut visitor, &self.path_builder, self.rg.symlink)?;
                self.rg.reset(&mut self.buf)?;

                self.buf.write_message(" @ ")?;

                self.rg.underlined_blue(&mut self.buf)?;
                self.buf.write_message(
                    visitor
                        .get_target_symlink()
                        .expect("Cannot get target link.")
                        .to_str()
                        .expect("Cannot convert target symlink to &str"),
                )?;
                self.rg.reset(&mut self.buf)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_media_type() {
                self.dir_stats.media_add_one();
                self.rg.blue(&mut self.buf)?; // TODO: Use specific color for media type
                self.buf
                    .print_file(&visitor, &self.path_builder, self.rg.file)?;
                self.rg.reset(&mut self.buf)?;
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
                self.rg.blue(&mut self.buf)?;
                self.buf
                    .print_dir(&visitor, &self.path_builder, self.rg.dir)?;
                self.rg.reset(&mut self.buf)?;
                self.buf.newline()?;

                if self.level.can_descend_further() {
                    self.level.add_one();
                    // If folder needed permission, we skip it. Safe to use unwrap.
                    if self.walk_dir(visitor.absolute_path().unwrap()).is_err() {
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
                self.rg.bold_red(&mut self.buf)?;
                self.buf.write_os_string(visitor.filename())?;
                self.rg.reset(&mut self.buf)?;
                self.nod.pop();
                continue;
            }
        }

        Ok(())
    }

    pub fn print_head(
        &mut self,
        file_name: OsString,
        base_path: PathBuf,
        fmeta: Metadata,
    ) -> TResult<()> {
        self.dir_stats.add_size(fmeta.size());

        self.print_info(&fmeta)?;

        self.rg.blue(&mut self.buf)?;
        self.buf
            .print_header(&fmeta, &base_path.clone(), &file_name, self.rg.head)?;
        self.rg.reset(&mut self.buf)?;
        self.buf.newline()?;

        Ok(())
    }

    pub fn print_info(&mut self, meta: &Metadata) -> TResult<()> {
        self.buf.print_permission(meta, self.rg.pms)?;
        self.buf.print_btime(meta, self.rg.btime)?;
        self.buf.print_mtime(meta, self.rg.mtime)?;
        self.buf.print_atime(meta, self.rg.atime)?;

        self.rg.green(&mut self.buf)?;
        self.buf.print_size(meta, self.rg.size)?;
        self.rg.reset(&mut self.buf)?;
        Ok(())
    }

    pub fn print_report(&mut self, report_mode: ReportMode) -> TResult<()> {
        self.buf.newline()?;
        self.dir_stats.accumulate_items();
        // Store formatted DirectoryStats here
        let mut report_summary = ReportSummary::with_capacity(50).unwrap();
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
