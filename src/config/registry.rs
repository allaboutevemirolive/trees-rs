use super::inspect::read_all_entries;
use super::inspect::read_visible_entries;
use super::inspect::read_visible_folders;
use super::inspect::FnReadDir;
use super::sortt::reverse_sort_by_name;
use super::sortt::sort_by_file_first;
use super::sortt::sort_by_name;
use super::sortt::FnSortEntries;

use crate::canva::attr::atime::FnExtAccessTime;
use crate::canva::attr::btime::FnExtBTime;
use crate::canva::attr::mtime::FnExtModTime;
use crate::canva::attr::pms::FnExtPermission;
use crate::canva::attr::size::FnExtSize;
use crate::canva::buffer::Buffer;
use crate::canva::entree::dirr::FnOutDir;
use crate::canva::entree::filee::FnOutFile;
use crate::canva::entree::headd::FnOutHead;
use crate::canva::entree::symlinked::FnOutSymlink;
use crate::error::simple::TResult;
use crate::report::tail::Tail;

use std::fs::DirEntry;
use std::io::StdoutLock;
use std::path::PathBuf;

// TODO: Rename to Callback
#[derive(Debug, Clone, Copy)]
pub struct Registry<'a> {
    // Common util
    pub read: FnReadDir,
    pub sort: FnSortEntries,
    /// Entry  
    pub dir: FnOutDir<StdoutLock<'a>>,
    pub file: FnOutFile<StdoutLock<'a>>,
    pub symlink: FnOutSymlink<StdoutLock<'a>>,
    pub head: FnOutHead<StdoutLock<'a>>,
    // Metadata
    pub pms: FnExtPermission<StdoutLock<'a>>,
    pub btime: FnExtBTime<StdoutLock<'a>>,
    pub mtime: FnExtModTime<StdoutLock<'a>>,
    pub atime: FnExtAccessTime<StdoutLock<'a>>,
    pub size: FnExtSize<StdoutLock<'a>>,
}

impl<'a> Registry<'a> {
    pub fn inspt_dents(&self, path: PathBuf, tail: &mut Tail) -> TResult<Vec<DirEntry>> {
        (self.read)(path, tail)
    }

    pub fn sort_dents(&self, entries: &mut Vec<DirEntry>) {
        (self.sort)(entries)
    }
}

impl<'a> Registry<'a> {
    pub fn new() -> TResult<Self> {
        let read: FnReadDir = read_visible_entries;
        let sort: FnSortEntries = sort_by_name;

        // Entry
        let dir: FnOutDir<StdoutLock> = Buffer::write_dir_color;
        let file: FnOutFile<StdoutLock> = Buffer::write_entry;
        let head: FnOutHead<StdoutLock> = Buffer::write_color_header_name;
        let symlink: FnOutSymlink<StdoutLock> = Buffer::write_symlink_color;

        // Entry's metadata
        let pms: FnExtPermission<StdoutLock> = Buffer::write_no_permission;
        let btime: FnExtBTime<StdoutLock> = Buffer::write_no_btime;
        let mtime: FnExtModTime<StdoutLock> = Buffer::write_no_mtime;
        let atime: FnExtAccessTime<StdoutLock> = Buffer::write_no_atime;
        let size: FnExtSize<StdoutLock> = Buffer::write_no_size;

        Ok(Self {
            read,
            sort,
            dir,
            file,
            head,
            symlink,
            pms,
            btime,
            mtime,
            atime,
            size,
        })
    }
}

// Read entries.
impl<'a> Registry<'a> {
    /// This method sets the internal `read` function to the implementation
    /// that reads all entries, including hidden ones.
    pub fn read_all_entries(&mut self) -> TResult<()> {
        self.read = read_all_entries;
        Ok(())
    }

    /// This sets the internal `read` function to the implementation
    /// that filters out hidden entries during the read process.
    pub fn read_visible_entries(&mut self) -> TResult<()> {
        self.read = read_visible_entries;
        Ok(())
    }

    /// This sets the internal `read` function to the implementation
    /// that focuses on retrieving visible folders within the registry.
    pub fn read_visible_folders(&mut self) -> TResult<()> {
        self.read = read_visible_folders;
        Ok(())
    }
}

// Sort's kind.
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

// Permission
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_permission(&mut self) -> TResult<()> {
        self.pms = Buffer::write_permission;
        Ok(())
    }

    pub fn with_no_permission(&mut self) -> TResult<()> {
        self.pms = Buffer::write_no_permission;
        Ok(())
    }
}

// Read entry's btime.
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_btime(&mut self) -> TResult<()> {
        self.btime = Buffer::write_btime;
        Ok(())
    }

    pub fn with_no_btime(&mut self) -> TResult<()> {
        self.btime = Buffer::write_no_btime;
        Ok(())
    }
}

// Read's mtime
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_mtime(&mut self) -> TResult<()> {
        self.mtime = Buffer::write_mtime;
        Ok(())
    }

    pub fn with_no_mtime(&mut self) -> TResult<()> {
        self.mtime = Buffer::write_no_mtime;
        Ok(())
    }
}

// Read atime
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_atime(&mut self) -> TResult<()> {
        self.atime = Buffer::write_atime;
        Ok(())
    }

    pub fn with_no_atime(&mut self) -> TResult<()> {
        self.atime = Buffer::write_no_atime;
        Ok(())
    }
}

// Kind's entry
impl<'a> Registry<'a> {
    pub fn with_color_entry(&mut self) -> TResult<()> {
        self.dir = Buffer::write_dir_color;
        Ok(())
    }

    pub fn with_colorless_entry(&mut self) -> TResult<()> {
        self.head = Buffer::write_header_name;
        self.dir = Buffer::write_dir;
        Ok(())
    }

    pub fn with_color_relative_path(&mut self) -> TResult<()> {
        self.dir = Buffer::write_color_dir_relative_path;
        self.file = Buffer::write_entry_relative_path;
        self.head = Buffer::write_color_header_relative_path;
        Ok(())
    }

    pub fn with_relative_path(&mut self) -> TResult<()> {
        self.dir = Buffer::write_dir_relative_path;
        self.file = Buffer::write_entry_relative_path;
        self.head = Buffer::write_header_relative_path;
        Ok(())
    }
}

// Size
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_size(&mut self) -> TResult<()> {
        self.size = Buffer::write_size;
        Ok(())
    }

    pub fn with_size_color(&mut self) -> TResult<()> {
        self.size = Buffer::write_size_color;
        Ok(())
    }

    pub fn with_no_size(&mut self) -> TResult<()> {
        self.size = Buffer::write_no_size;
        Ok(())
    }
}
