use crate::canva::buffer;
use crate::config::path::get_absolute_current_shell;
use crate::config::path::Directory;
use crate::config::registry::Registry;
use crate::error::simple::TResult;
use crate::error::simple::TSimpleError;
use crate::report::tail::Tail;
use crate::sort::dent::ty_sort;
use crate::tree::branch::Branch;
use crate::tree::level::Level;
use crate::tree::node::Node;
use crate::walk::buffer::Buffer;

pub mod metada;
use self::metada::Visitor;

use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::io::StdoutLock;
use std::path::PathBuf;

/// Struct that store the path where we needs to start traverse
pub struct RootPath {
    pub fdot: OsString,
    pub fname: OsString,
    pub fpath: PathBuf,
}

impl RootPath {
    pub fn abs_curr_shell() -> TResult<Self> {
        let path_dir = get_absolute_current_shell().map_err(|err| {
            TSimpleError::new(1, format!("Failed to get absolute current shell: {}", err))
        })?;

        let mut fpath = PathBuf::new();
        fpath.push(path_dir);
        let fname = fpath.file_name().unwrap().to_os_string();
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
        let stdout = io::stdout();
        let buf = Buffer::new(stdout.lock())?;

        let branch = Branch::initialize("└── ", "├── ", "    ", "│   ")
            .map_err(|err| TSimpleError::new(1, format!("Failed to initialize branch: {}", err)))?;

        let node_cap = 5_000;

        let nod = Node::with_capacity(node_cap)
            .map_err(|err| TSimpleError::new(1, format!("Failed to initialize node: {}", err)))?;

        let tail = Tail::default();

        let level = Level::with_lvl_and_cap(1, 10_000);
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

    pub fn walk_dir<T>(&mut self, path: T) -> TResult<()>
    where
        T: Into<PathBuf>,
    {
        let mut entries: Vec<std::fs::DirEntry> =
            Directory::new(path)?.inspect_entries(&mut self.tail, self.rg.read)?;

        ty_sort(self.rg.sort, &mut entries);

        let enumerated_entries: Vec<(usize, std::fs::DirEntry)> =
            entries.into_iter().enumerate().collect();

        let entries_len = enumerated_entries.len();

        for (idx, entry) in enumerated_entries {
            let visitor = Visitor::new(entry, &self.level)?;

            self.tail.add_size(visitor.size);
            self.print_meta(&visitor.meta)?;

            self.nod.push_if(idx, entries_len);
            self.nod.to_branches(&self.branch, &mut self.buf)?;

            if visitor.filety.is_dir() {
                self.tail.dir_plus_one();
                self.buf.paint_entry(
                    &visitor,
                    &self.rpath.fpath,
                    &self.rpath.fname,
                    self.rg.dir,
                )?;
                self.buf.write_newline()?;
                if self.level.lvl < self.level.cap {
                    self.level.plus_one();
                    self.walk_dir(visitor.abs)?; // Traverse
                    self.level.minus_one();
                }
            } else {
                self.tail.file_plus_one();
                self.buf.paint_entry(
                    &visitor,
                    &self.rpath.fpath,
                    &self.rpath.fname,
                    self.rg.file,
                )?;
                self.buf.write_newline()?;
            }
            self.nod.pop();
        }

        Ok(())
    }

    pub fn print_meta(&mut self, meta: &Metadata) -> TResult<()> {
        self.buf.paint_permission(&meta, self.rg.pms)?;
        self.buf.paint_btime(&meta, self.rg.btime)?;
        self.buf.paint_mtime(&meta, self.rg.mtime)?;
        self.buf.paint_atime(&meta, self.rg.atime)?;
        self.buf.paint_size(&meta, self.rg.size)?;
        Ok(())
    }
}
