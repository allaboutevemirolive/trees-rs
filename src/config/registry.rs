use super::path::read_all_entries;
use super::path::read_visible_entries;
use super::path::read_visible_folders;
use super::path::Directory;
use crate::canva::buffer::Buffer;
use crate::canva::which::attr::atime::write_atime;
use crate::canva::which::attr::atime::write_no_atime;
use crate::canva::which::attr::atime::FnExtAccessTime;
use crate::canva::which::attr::btime::write_btime;
use crate::canva::which::attr::btime::write_no_btime;
use crate::canva::which::attr::btime::FnExtBTime;
use crate::canva::which::attr::mtime::write_mtime;
use crate::canva::which::attr::mtime::write_no_mtime;
use crate::canva::which::attr::mtime::FnExtModTime;
use crate::canva::which::attr::pms::write_no_permission;
use crate::canva::which::attr::pms::write_permission;
use crate::canva::which::attr::pms::FnExtPermission;
use crate::canva::which::attr::size::write_no_size;
use crate::canva::which::attr::size::write_size;
use crate::canva::which::attr::size::write_size_color;
use crate::canva::which::attr::size::FnExtSize;
use crate::canva::which::entree::filee::FnOutFile;
use crate::canva::which::entree::headd::FnOutHead;
use crate::config::path::FnReadDir;
use crate::error::simple::TResult;
use crate::report::tail;
use crate::report::tail::Tail;
use crate::sort::dent::reverse_sort_by_name;
use crate::sort::dent::sort_by_file_first;
use crate::sort::dent::sort_by_name;
use crate::sort::dent::FnSortEntries;

use std::fs::DirEntry;
use std::fs::Metadata;
use std::io;
use std::io::StdoutLock;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct Registry<'a> {
    pub read: FnReadDir,
    pub sort: FnSortEntries,
    /// Entry  
    pub dir: FnOutFile<StdoutLock<'a>>,
    pub file: FnOutFile<StdoutLock<'a>>,
    pub head: FnOutHead<StdoutLock<'a>>,
    // Metadata
    pub pms: FnExtPermission<StdoutLock<'a>>,
    pub btime: FnExtBTime<StdoutLock<'a>>,
    pub mtime: FnExtModTime<StdoutLock<'a>>,
    pub atime: FnExtAccessTime<StdoutLock<'a>>,
    pub size: FnExtSize<StdoutLock<'a>>,
}

impl<'a> Registry<'a> {
    pub fn sort_dents(&self, entries: &mut Vec<DirEntry>) {
        (self.sort)(entries)
    }

    pub fn inspt_dents(&self, path: PathBuf, mut tail: Tail) -> TResult<Vec<DirEntry>> {
        (self.read)(path, &mut tail)
    }

    pub fn print_permission(
        &self,
        buf: &'a mut Buffer<StdoutLock<'a>>,
        meta: &Metadata,
    ) -> io::Result<()> {
        (self.pms)(buf, meta)
    }

    //     pub fn paint_permission(&mut self, meta: &Metadata, f: FnExtPermission<W>) -> io::Result<()> {
    //         f(self, meta)
    //     }
}

impl<'a> Registry<'a> {
    pub fn new() -> TResult<Self> {
        let read: FnReadDir = read_visible_entries;
        let sort: FnSortEntries = sort_by_name;

        // Entry
        let dir: FnOutFile<StdoutLock> = Buffer::write_entry_color;
        let file: FnOutFile<StdoutLock> = Buffer::write_entry;
        let head: FnOutHead<StdoutLock> = Buffer::write_color_header_name;

        // Entry's metadata
        let pms: FnExtPermission<StdoutLock> = write_no_permission;
        let btime: FnExtBTime<StdoutLock> = write_no_btime;
        let mtime: FnExtModTime<StdoutLock> = write_no_mtime;
        let atime: FnExtAccessTime<StdoutLock> = write_no_atime;
        let size: FnExtSize<StdoutLock> = write_no_size;

        Ok(Self {
            read,
            sort,
            dir,
            file,
            head,
            pms,
            btime,
            mtime,
            atime,
            size,
        })
    }
}

impl<'a> Registry<'a> {
    pub fn read_all_entries(&mut self) -> TResult<()> {
        self.read = read_all_entries;
        Ok(())
    }

    pub fn read_visible_entries(&mut self) -> TResult<()> {
        self.read = read_visible_entries;
        Ok(())
    }

    pub fn read_visible_folders(&mut self) -> TResult<()> {
        self.read = read_visible_folders;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_sort_entries(&mut self) -> TResult<()> {
        self.sort = sort_by_name;
        Ok(())
    }

    pub fn with_reverse_sort_entries(&mut self) -> TResult<()> {
        self.sort = reverse_sort_by_name;
        Ok(())
    }

    pub fn with_sort_by_file_first(&mut self) -> TResult<()> {
        self.sort = sort_by_file_first;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_permission(&mut self) -> TResult<()> {
        self.pms = write_permission;
        Ok(())
    }

    pub fn with_no_permission(&mut self) -> TResult<()> {
        self.pms = write_no_permission;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_btime(&mut self) -> TResult<()> {
        self.btime = write_btime;
        Ok(())
    }

    pub fn with_no_btime(&mut self) -> TResult<()> {
        self.btime = write_no_btime;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_mtime(&mut self) -> TResult<()> {
        self.mtime = write_mtime;
        Ok(())
    }

    pub fn with_no_mtime(&mut self) -> TResult<()> {
        self.mtime = write_no_mtime;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_atime(&mut self) -> TResult<()> {
        self.atime = write_atime;
        Ok(())
    }

    pub fn with_no_atime(&mut self) -> TResult<()> {
        self.atime = write_no_atime;
        Ok(())
    }
}

impl<'a> Registry<'a> {
    pub fn with_color_entry(&mut self) -> TResult<()> {
        self.dir = Buffer::write_entry_color;
        Ok(())
    }

    pub fn with_colorless_entry(&mut self) -> TResult<()> {
        self.head = Buffer::write_header_name;
        self.dir = Buffer::write_entry;
        Ok(())
    }

    pub fn with_color_relative_path(&mut self) -> TResult<()> {
        self.dir = Buffer::write_color_entry_relative_path;
        self.file = Buffer::write_entry_relative_path;
        self.head = Buffer::write_color_header_relative_path;
        Ok(())
    }

    pub fn with_relative_path(&mut self) -> TResult<()> {
        self.dir = Buffer::write_entry_relative_path;
        self.file = Buffer::write_entry_relative_path;
        self.head = Buffer::write_header_relative_path;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_size(&mut self) -> TResult<()> {
        self.size = write_size;
        Ok(())
    }

    pub fn with_size_color(&mut self) -> TResult<()> {
        self.size = write_size_color;
        Ok(())
    }

    pub fn with_no_size(&mut self) -> TResult<()> {
        self.size = write_no_size;
        Ok(())
    }
}
