use super::inspect::read_all_entries;
use super::inspect::read_visible_entries;
use super::inspect::read_visible_folders;
use super::inspect::FnReadDir;
use super::sorting::reverse_sort_by_name;
use super::sorting::sort_by_file_first;
use super::sorting::sort_by_name;
use super::sorting::FnSortEntries;

use crate::render::attr::atime::FnExtAccessTime;
use crate::render::attr::btime::FnExtBTime;
use crate::render::attr::mtime::FnExtModTime;
use crate::render::attr::pms::FnExtPermission;
use crate::render::attr::size::FnExtSize;
use crate::render::buffer::Buffer;
use crate::render::color::FnColor;
use crate::render::entree::dirr::FnOutDir;
use crate::render::entree::filee::FnOutFile;
use crate::render::entree::headd::FnOutHead;
use crate::render::entree::symlinked::FnOutSymlink;
use crate::report::stats::DirectoryStats;

use std::fs::DirEntry;
use std::io;
use std::io::StdoutLock;
use std::path::PathBuf;

// TODO: Rename to Callback
#[derive(Debug, Clone, Copy)]
pub struct Registry<'a> {
    // Common util
    read: FnReadDir,
    sort: FnSortEntries,

    // Entry
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

    // Color
    reset: FnColor<StdoutLock<'a>>,
    yellow: FnColor<StdoutLock<'a>>,
    bold_red: FnColor<StdoutLock<'a>>,
    underlined_blue: FnColor<StdoutLock<'a>>,
    blue: FnColor<StdoutLock<'a>>,
    green: FnColor<StdoutLock<'a>>,
    purple: FnColor<StdoutLock<'a>>,
}

impl<'a> Registry<'a> {
    pub fn reset(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.reset)(buf)
    }

    pub fn yellow(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.yellow)(buf)
    }

    pub fn bold_red(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.bold_red)(buf)
    }

    pub fn underlined_blue(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.underlined_blue)(buf)
    }

    pub fn blue(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.blue)(buf)
    }

    pub fn green(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.green)(buf)
    }

    pub fn purple(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.purple)(buf)
    }
}

impl<'a> Registry<'a> {
    pub fn inspt_dents(
        &self,
        path: PathBuf,
        dir_stats: &mut DirectoryStats,
    ) -> anyhow::Result<Vec<DirEntry>> {
        tracing::info!("Inspect directory's entries");
        (self.read)(path, dir_stats)
    }

    pub fn sort_dents(&self, entries: &mut Vec<DirEntry>) {
        tracing::info!("Sort DirEntry");
        (self.sort)(entries)
    }
}

impl<'a> Registry<'a> {
    pub fn new() -> anyhow::Result<Self> {
        // Common util
        let read: FnReadDir = read_visible_entries;
        let sort: FnSortEntries = sort_by_name;

        // Entry
        let dir: FnOutDir<StdoutLock> = Buffer::write_dir;
        let file: FnOutFile<StdoutLock> = Buffer::write_entry;
        let head: FnOutHead<StdoutLock> = Buffer::write_header_name;
        let symlink: FnOutSymlink<StdoutLock> = Buffer::write_symlink;

        // Entry's metadata
        let pms: FnExtPermission<StdoutLock> = Buffer::write_no_permission;
        let btime: FnExtBTime<StdoutLock> = Buffer::write_no_btime;
        let mtime: FnExtModTime<StdoutLock> = Buffer::write_no_mtime;
        let atime: FnExtAccessTime<StdoutLock> = Buffer::write_no_atime;
        let size: FnExtSize<StdoutLock> = Buffer::write_no_size;

        // Color
        let reset: FnColor<StdoutLock> = Buffer::reset_color;
        let yellow: FnColor<StdoutLock> = Buffer::yellow;
        let bold_red: FnColor<StdoutLock> = Buffer::bold_red;
        let underlined_blue: FnColor<StdoutLock> = Buffer::underlined_blue;
        let blue: FnColor<StdoutLock> = Buffer::blue;
        let green: FnColor<StdoutLock> = Buffer::green;
        let purple: FnColor<StdoutLock> = Buffer::purple;

        Ok(Self {
            // common-util
            read,
            sort,
            // entry
            dir,
            file,
            head,
            symlink,
            // pms
            pms,
            btime,
            mtime,
            atime,
            size,
            // color
            reset,
            yellow,
            bold_red,
            underlined_blue,
            blue,
            green,
            purple,
        })
    }
}

// Read entries.
impl<'a> Registry<'a> {
    /// This method sets the internal `read` function to the implementation
    /// that reads all entries, including hidden ones.
    pub fn read_all_entries(&mut self) -> anyhow::Result<()> {
        self.read = read_all_entries;
        Ok(())
    }

    /// This sets the internal `read` function to the implementation
    /// that filters out hidden entries during the read process.
    pub fn read_visible_entries(&mut self) -> anyhow::Result<()> {
        self.read = read_visible_entries;
        Ok(())
    }

    /// This sets the internal `read` function to the implementation
    /// that focuses on retrieving visible folders within the registry.
    pub fn read_visible_folders(&mut self) -> anyhow::Result<()> {
        self.read = read_visible_folders;
        Ok(())
    }
}

// Sort's kind.
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_sort_entries(&mut self) -> anyhow::Result<()> {
        self.sort = sort_by_name;
        Ok(())
    }

    pub fn with_reverse_sort_entries(&mut self) -> anyhow::Result<()> {
        self.sort = reverse_sort_by_name;
        Ok(())
    }

    pub fn with_sort_by_file_first(&mut self) -> anyhow::Result<()> {
        self.sort = sort_by_file_first;
        Ok(())
    }
}

// Permission
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_permission(&mut self) -> anyhow::Result<()> {
        self.pms = Buffer::write_permission;
        Ok(())
    }

    pub fn with_no_permission(&mut self) -> anyhow::Result<()> {
        self.pms = Buffer::write_no_permission;
        Ok(())
    }
}

// Read entry's btime.
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_btime(&mut self) -> anyhow::Result<()> {
        self.btime = Buffer::write_btime;
        Ok(())
    }

    pub fn with_no_btime(&mut self) -> anyhow::Result<()> {
        self.btime = Buffer::write_no_btime;
        Ok(())
    }
}

// Read's mtime
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_mtime(&mut self) -> anyhow::Result<()> {
        self.mtime = Buffer::write_mtime;
        Ok(())
    }

    pub fn with_no_mtime(&mut self) -> anyhow::Result<()> {
        self.mtime = Buffer::write_no_mtime;
        Ok(())
    }
}

// Read atime
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_atime(&mut self) -> anyhow::Result<()> {
        self.atime = Buffer::write_atime;
        Ok(())
    }

    pub fn with_no_atime(&mut self) -> anyhow::Result<()> {
        self.atime = Buffer::write_no_atime;
        Ok(())
    }
}

// Kind's entry
impl<'a> Registry<'a> {
    pub fn with_entry(&mut self) -> anyhow::Result<()> {
        self.dir = Buffer::write_dir;
        Ok(())
    }

    pub fn with_relative_path(&mut self) -> anyhow::Result<()> {
        self.dir = Buffer::write_dir_relative_path;
        self.file = Buffer::write_entry_relative_path;
        self.head = Buffer::write_header_relative_path;
        self.symlink = Buffer::write_symlink_relative_path;
        Ok(())
    }
}

// Size
#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_size(&mut self) -> anyhow::Result<()> {
        self.size = Buffer::write_size;
        Ok(())
    }

    pub fn with_no_size(&mut self) -> anyhow::Result<()> {
        self.size = Buffer::write_no_size;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> Registry<'a> {
    pub fn with_color(&mut self) -> anyhow::Result<()> {
        self.reset = Buffer::reset_color;
        self.yellow = Buffer::yellow;
        self.bold_red = Buffer::bold_red;
        self.underlined_blue = Buffer::underlined_blue;
        self.blue = Buffer::blue;
        self.green = Buffer::green;
        Ok(())
    }

    pub fn with_no_color(&mut self) -> anyhow::Result<()> {
        self.reset = Buffer::no_color;
        self.yellow = Buffer::no_color;
        self.bold_red = Buffer::no_color;
        self.underlined_blue = Buffer::no_color;
        self.blue = Buffer::no_color;
        self.green = Buffer::no_color;
        self.purple = Buffer::no_color;
        Ok(())
    }
}
