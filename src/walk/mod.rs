use crate::canva::buffer::Buffer;
use crate::config::registry::Registry;
use crate::error::simple::TResult;
use crate::report::tail::Tail;
use crate::tree::branch::Branch;
use crate::tree::level::Level;
use crate::tree::node::Node;

pub mod visit;
use self::visit::Visitor;

use std::env;
use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::io::StdoutLock;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

/// Struct that store the path where we needs to start traverse
pub struct RootPath {
    pub fdot: OsString,
    pub fname: OsString,
    pub fpath: PathBuf,
}

impl RootPath {
    pub fn abs_curr_shell() -> TResult<Self> {
        let path_dir = env::current_dir()
            .expect("Failed to get current directory")
            .into_os_string();

        let mut fpath = PathBuf::new();
        fpath.push(path_dir);
        let fname = fpath
            .file_name()
            .expect("Cannot retrieve file name for the starting point path.")
            .to_os_string();
        let fdot = OsString::from(".");

        Ok(Self { fdot, fname, fpath })
    }
}

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
        let rpath = RootPath::abs_curr_shell()?;

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

    fn print_meta(&mut self, meta: &Metadata) -> TResult<()>;

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
        self.print_meta(&fmeta)?;
        self.buf
            .paint_header(&fmeta, &fpath.clone(), &fname, self.rg.head)?;
        self.buf.write_newline()?;

        Ok(())
    }

    fn print_meta(&mut self, meta: &Metadata) -> TResult<()> {
        self.buf.paint_permission(meta, self.rg.pms)?;
        self.buf.paint_btime(meta, self.rg.btime)?;
        self.buf.paint_mtime(meta, self.rg.mtime)?;
        self.buf.paint_atime(meta, self.rg.atime)?;
        self.buf.paint_size(meta, self.rg.size)?;
        Ok(())
    }

    fn print_report(&mut self) -> TResult<()> {
        self.buf.write_newline()?;
        self.buf.write_message(&self.tail.to_string())?;
        self.buf.write_newline()?;

        Ok(())
    }
}

pub trait Walker<'gcx> {
    fn walk_dir(&mut self, path: PathBuf) -> TResult<()>;
}

impl<'gcx> Walker<'gcx> for GlobalCtxt<'gcx> {
    fn walk_dir(&mut self, path: PathBuf) -> TResult<()> {
        let mut entries: Vec<std::fs::DirEntry> = self.rg.inspt_dents(path, &mut self.tail)?;

        self.rg.sort_dents(&mut entries);

        let enumerated_entries: Vec<(usize, std::fs::DirEntry)> =
            entries.into_iter().enumerate().collect();

        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            let mut visitor = Visitor::new(entry)?;

            self.tail.add_size(visitor.size);
            self.print_meta(&visitor.meta)?;

            self.nod.push_if(idx, entries_len);
            self.nod.to_branches(&self.branch, &mut self.buf)?;

            if visitor.is_symlink() {
                self.tail.sym_plus_one();
                self.buf
                    .print_symlink(&mut visitor, &self.rpath, self.rg.symlink)?;
                self.buf.write_newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_file() {
                self.tail.file_plus_one();
                self.buf.print_file(&visitor, &self.rpath, self.rg.file)?;
                self.buf.write_newline()?;
                self.nod.pop();
                continue;
            }

            if visitor.is_dir() {
                self.tail.dir_plus_one();
                self.buf.print_dir(&visitor, &self.rpath, self.rg.dir)?;
                self.buf.write_newline()?;

                if self.level.lvl < self.level.cap {
                    self.level.plus_one();
                    self.walk_dir(visitor.abs)?; // DFS
                    self.level.minus_one();
                }
            }
            // We handle the case where entry is not symlink or file or dir
            self.nod.pop();
        }

        Ok(())
    }
}
