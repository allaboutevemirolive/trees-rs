use super::path::Directory;
use crate::canva::buffer::Buffer;
use crate::canva::which::attr::atime::WhichAccessTime;
use crate::canva::which::attr::btime::WhichBirthTime;
use crate::canva::which::attr::mtime::WhichModificationTime;
use crate::canva::which::attr::pms::WhichPermission;
use crate::canva::which::attr::size::WhichSize;
use crate::canva::which::entree::WhichEntry;
use crate::canva::which::headerr::WhichHeader;
use crate::config::path::WhichReader;
use crate::error::simple::UResult;
use crate::sort::dent::reverse_sort_by_name;
use crate::sort::dent::sort_by_name;
use crate::sort::dent::WhichSort;

use std::io::StdoutLock;

#[derive(Debug, Clone, Copy)]
pub struct CallbackRegistry<'a> {
    pub read: WhichReader,
    pub sort: WhichSort,
    /// Entry  
    pub dir: WhichEntry<StdoutLock<'a>>,
    pub file: WhichEntry<StdoutLock<'a>>,
    pub head: WhichHeader<StdoutLock<'a>>,
    // Metadata
    pub pms: WhichPermission<StdoutLock<'a>>,
    pub btime: WhichBirthTime<StdoutLock<'a>>,
    pub mtime: WhichModificationTime<StdoutLock<'a>>,
    pub atime: WhichAccessTime<StdoutLock<'a>>,
    pub size: WhichSize<StdoutLock<'a>>,
}

impl<'a> CallbackRegistry<'a> {
    pub fn new() -> UResult<Self> {
        let read: WhichReader = Directory::read_visible_entries;
        let sort: WhichSort = sort_by_name;

        // Entry
        let dir: WhichEntry<StdoutLock> = Buffer::write_entry_color;
        let file: WhichEntry<StdoutLock> = Buffer::write_entry;
        let head: WhichHeader<StdoutLock> = Buffer::write_color_header_name;

        // Meta
        let pms: WhichPermission<StdoutLock> = Buffer::write_no_permission;
        let btime: WhichBirthTime<StdoutLock> = Buffer::write_no_btime;
        let mtime: WhichModificationTime<StdoutLock> = Buffer::write_no_mtime;
        let atime: WhichAccessTime<StdoutLock> = Buffer::write_no_atime;
        let size: WhichSize<StdoutLock> = Buffer::write_no_size;

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

impl<'a> CallbackRegistry<'a> {
    pub fn read_all_entries(&mut self) -> UResult<()> {
        self.read = Directory::read_all_entries;
        Ok(())
    }

    pub fn read_visible_entries(&mut self) -> UResult<()> {
        self.read = Directory::read_visible_entries;
        Ok(())
    }

    pub fn read_visible_folders(&mut self) -> UResult<()> {
        self.read = Directory::read_visible_folders;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_sort_entries(&mut self) -> UResult<()> {
        self.sort = sort_by_name;
        Ok(())
    }

    pub fn with_reverse_sort_entries(&mut self) -> UResult<()> {
        self.sort = reverse_sort_by_name;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_permission(&mut self) -> UResult<()> {
        self.pms = Buffer::write_permission;
        Ok(())
    }

    pub fn with_no_permission(&mut self) -> UResult<()> {
        self.pms = Buffer::write_no_permission;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_btime(&mut self) -> UResult<()> {
        self.btime = Buffer::write_btime;
        Ok(())
    }

    pub fn with_no_btime(&mut self) -> UResult<()> {
        self.btime = Buffer::write_no_btime;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_mtime(&mut self) -> UResult<()> {
        self.mtime = Buffer::write_mtime;
        Ok(())
    }

    pub fn with_no_mtime(&mut self) -> UResult<()> {
        self.mtime = Buffer::write_no_mtime;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_atime(&mut self) -> UResult<()> {
        self.atime = Buffer::write_atime;
        Ok(())
    }

    pub fn with_no_atime(&mut self) -> UResult<()> {
        self.atime = Buffer::write_no_atime;
        Ok(())
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_color_entry(&mut self) -> UResult<()> {
        self.dir = Buffer::write_entry_color;
        Ok(())
    }

    pub fn with_colorless_entry(&mut self) -> UResult<()> {
        self.head = Buffer::write_header_name;
        self.dir = Buffer::write_entry;
        Ok(())
    }

    pub fn with_relative_path(&mut self) -> UResult<()> {
        self.dir = Buffer::write_entry_relative_path;
        self.file = Buffer::write_entry_relative_path;
        self.head = Buffer::write_header_relative_path;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_size(&mut self) -> UResult<()> {
        self.size = Buffer::write_size;
        Ok(())
    }

    pub fn with_size_color(&mut self) -> UResult<()> {
        self.size = Buffer::write_size_color;
        Ok(())
    }

    pub fn with_no_size(&mut self) -> UResult<()> {
        self.size = Buffer::write_no_size;
        Ok(())
    }
}
