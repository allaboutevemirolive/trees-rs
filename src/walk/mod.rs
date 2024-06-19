use crate::config::color::FileColors;
use crate::config::registry::Registry;
use crate::config::root::BaseDirectory;
use crate::config::root::PathBuilder;
use crate::error::simple::TResult;
use crate::render::buffer::Buffer;
use crate::report::tail::Tail;
use crate::tree::branch::Branch;
use crate::tree::level::Level;
use crate::tree::node::Node;

pub mod visit;
use self::visit::Visitor;

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
    pub base_dir: BaseDirectory,
    pub tail: Tail,
    pub file_colors: FileColors,
    pub path_builder: PathBuilder,
}

impl<'tr> TreeCtxt<'tr> {
    pub fn new() -> TResult<Self> {
        let buf = Buffer::new(io::stdout().lock())?;
        let branch = Branch::default();
        let nod = Node::default();
        let tail = Tail::default();
        let level = Level::default();
        let rg = Registry::new()?;
        let base_dir = BaseDirectory::from_current_dir()?;
        let file_colors = FileColors::new();
        let path_builder = PathBuilder::new();

        Ok(Self {
            branch,
            buf,
            level,
            nod,
            rg,
            base_dir,
            tail,
            file_colors,
            path_builder,
        })
    }

    pub fn walk_dir(&mut self, path: PathBuf) -> TResult<()> {
        // Get entries in target path
        let mut entries: Vec<std::fs::DirEntry> = self.rg.inspt_dents(path, &mut self.tail)?;

        self.rg.sort_dents(&mut entries);

        let enumerated_entries: Vec<(usize, std::fs::DirEntry)> =
            entries.into_iter().enumerate().collect();

        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            // Get entry's information
            let mut visitor = Visitor::new(entry)?;
            // Accumulate entry's size
            self.tail.add_size(visitor.size().expect("Invalid size."));
            // Print entry's information
            self.print_info(&visitor.meta)?;
            // If current entry is not the last entry in entries
            self.nod.push_if(idx, entries_len);
            // Convert node to branch's stick
            self.nod.into_branch(&self.branch, &mut self.buf)?;

            if visitor.is_symlink() {
                self.tail.symlink_add_one();
                self.buf
                    .write_message(&self.file_colors.symlink.open_color())?;
                self.buf
                    .print_symlink(&mut visitor, &self.path_builder, self.rg.symlink)?;
                self.buf
                    .write_message(&self.file_colors.symlink.closed_color())?;

                self.buf.write_message(" @ ")?;

                self.buf
                    .write_message(&self.file_colors.target_symlink.open_color())?;
                self.buf.write_message(
                    visitor
                        .get_target_symlink()
                        .expect("Cannot get target link.")
                        .to_str()
                        .expect("Cannot convert target symlink to &str"),
                )?;
                self.buf
                    .write_message(&self.file_colors.target_symlink.closed_color())?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_file() {
                self.tail.file_add_one();
                self.buf
                    .write_message(&self.file_colors.file.open_color())?;
                self.buf
                    .print_file(&visitor, &self.path_builder, self.rg.file)?;
                self.buf
                    .write_message(&self.file_colors.file.closed_color())?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_dir() {
                self.tail.dir_add_one();
                self.buf
                    .write_message(&self.file_colors.directory.open_color())?;
                self.buf
                    .print_dir(&visitor, &self.path_builder, self.rg.dir)?;
                self.buf
                    .write_message(&self.file_colors.directory.closed_color())?;

                self.buf.newline()?;

                if self.level.can_descend_further() {
                    self.level.add_one();
                    self.walk_dir(visitor.absolute_path().expect("Invalid absolute path."))?;
                    self.level.subtract_one();
                }
            }
            self.nod.pop(); // If entry is not dir, file or symlink
        }

        Ok(())
    }

    pub fn print_head(
        &mut self,
        file_name: OsString,
        base_path: PathBuf,
        fmeta: Metadata,
    ) -> TResult<()> {
        self.tail.add_size(fmeta.size());

        self.print_info(&fmeta)?;

        self.buf
            .write_message(&self.file_colors.directory.open_color())?;
        self.buf
            .print_header(&fmeta, &base_path.clone(), &file_name, self.rg.head)?;
        self.buf
            .write_message(&self.file_colors.directory.closed_color())?;
        self.buf.newline()?;

        Ok(())
    }

    pub fn print_info(&mut self, meta: &Metadata) -> TResult<()> {
        self.buf.print_permission(meta, self.rg.pms)?;
        self.buf.print_btime(meta, self.rg.btime)?;
        self.buf.print_mtime(meta, self.rg.mtime)?;
        self.buf.print_atime(meta, self.rg.atime)?;
        self.buf
            .write_message(&self.file_colors.size.open_color())?;
        self.buf.print_size(meta, self.rg.size)?;
        self.buf
            .write_message(&self.file_colors.size.closed_color())?;
        Ok(())
    }

    pub fn print_report(&mut self) -> TResult<()> {
        self.buf.newline()?;
        self.buf.write_message(&self.tail.to_string())?;
        self.buf.newline()?;

        Ok(())
    }
}
