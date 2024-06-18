use crate::config::registry::Registry;
use crate::config::root::RootPath;
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

pub struct GlobalCtxt<'gcx> {
    pub branch: Branch,
    pub buf: Buffer<StdoutLock<'gcx>>,
    pub level: Level,
    pub nod: Node,
    pub rg: Registry<'gcx>,
    pub rpath: RootPath,
    pub tail: Tail,
}

impl<'gcx> GlobalCtxt<'gcx> {
    pub fn new() -> TResult<Self> {
        let buf = Buffer::new(io::stdout().lock())?;
        let branch = Branch::default();
        let nod = Node::default();
        let tail = Tail::default();
        let level = Level::default();
        let rg = Registry::new()?;
        let rpath = RootPath::from_current_dir()?;

        Ok(Self {
            branch,
            buf,
            level,
            nod,
            rg,
            rpath,
            tail,
        })
    }
}

pub trait Printer<'gcx> {
    // head
    // ├── dir
    // │   ├── entry
    fn print_head(&mut self, fname: OsString, fpath: PathBuf, fmeta: Metadata) -> TResult<()>;

    fn print_info(&mut self, meta: &Metadata) -> TResult<()>;

    // └── dir
    //     ├── entry1
    //     └── entry2
    //
    // 13 directories, 36 files, 0 hidden, 0.00 gigabyte
    fn print_report(&mut self) -> TResult<()>;
}

impl<'gcx> Printer<'gcx> for GlobalCtxt<'gcx> {
    fn print_head(&mut self, fname: OsString, fpath: PathBuf, fmeta: Metadata) -> TResult<()> {
        self.tail.add_size(fmeta.size());
        self.print_info(&fmeta)?;
        self.buf
            .print_header(&fmeta, &fpath.clone(), &fname, self.rg.head)?;
        self.buf.newline()?;

        Ok(())
    }

    fn print_info(&mut self, meta: &Metadata) -> TResult<()> {
        self.buf.print_permission(meta, self.rg.pms)?;
        self.buf.print_btime(meta, self.rg.btime)?;
        self.buf.print_mtime(meta, self.rg.mtime)?;
        self.buf.print_atime(meta, self.rg.atime)?;
        self.buf.print_size(meta, self.rg.size)?;
        Ok(())
    }

    fn print_report(&mut self) -> TResult<()> {
        self.buf.newline()?;
        self.buf.write_message(&self.tail.to_string())?;
        self.buf.newline()?;

        Ok(())
    }
}

pub trait Walker<'gcx> {
    fn walk_dir(&mut self, path: PathBuf) -> TResult<()>;
}

impl<'gcx> Walker<'gcx> for GlobalCtxt<'gcx> {
    fn walk_dir(&mut self, path: PathBuf) -> TResult<()> {
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
                    .print_symlink(&mut visitor, &self.rpath, self.rg.symlink)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_file() {
                self.tail.file_add_one();
                self.buf.print_file(&visitor, &self.rpath, self.rg.file)?;
                self.buf.newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_dir() {
                self.tail.dir_add_one();
                self.buf.print_dir(&visitor, &self.rpath, self.rg.dir)?;
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
}
