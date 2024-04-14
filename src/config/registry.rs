use crate::canva::buffer::Buffer;
use crate::canva::which::attr::date::WhichDate;
use crate::canva::which::attr::perm::WhichPermission;
use crate::canva::which::attr::size::WhichSize;
use crate::canva::which::entree::WhichEntry;
use crate::canva::which::headerr::WhichHeader;
use crate::config::path::WhichReader;
use crate::error::simple::UResult;
use crate::sort::dent::reverse_sort_by_name;
use crate::sort::dent::sort_by_name;
use crate::sort::dent::WhichSort;
use std::io::StdoutLock;

use super::path::Directory;

#[derive(Debug, Clone, Copy)]
pub struct CallbackRegistry<'a> {
    pub wr: WhichReader,
    pub ws: WhichSort,
    pub wa: WhichPermission<StdoutLock<'a>>,
    pub wd: WhichDate<StdoutLock<'a>>,
    /// Entries  
    pub we: WhichEntry<StdoutLock<'a>>,
    pub wf: WhichEntry<StdoutLock<'a>>,
    pub wh: WhichHeader<StdoutLock<'a>>,
    // Attribute
    pub wha: WhichPermission<StdoutLock<'a>>,
    pub whd: WhichDate<StdoutLock<'a>>,
    pub wsz: WhichSize<StdoutLock<'a>>,
}

impl<'a> CallbackRegistry<'a> {
    pub fn new() -> UResult<Self> {
        let wr: WhichReader = Directory::read_visible_entries;
        let ws: WhichSort = sort_by_name;
        let wa: WhichPermission<StdoutLock> = Buffer::write_no_permission;
        let wd: WhichDate<StdoutLock> = Buffer::write_no_date;
        let we: WhichEntry<StdoutLock> = Buffer::write_entry_color;
        let wf: WhichEntry<StdoutLock> = Buffer::write_entry;
        let wh: WhichHeader<StdoutLock> = Buffer::write_color_header_name;
        let wha: WhichPermission<StdoutLock> = Buffer::write_no_permission;
        let whd: WhichDate<StdoutLock> = Buffer::write_no_date;
        let wsz: WhichSize<StdoutLock> = Buffer::write_size_color;
        Ok(Self {
            wr,
            ws,
            wa,
            wd,
            we,
            wf,
            wh,
            wha,
            whd,
            wsz,
        })
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn read_all_entries(&mut self) -> UResult<()> {
        self.wr = Directory::read_all_entries;
        Ok(())
    }

    pub fn read_visible_entries(&mut self) -> UResult<()> {
        self.wr = Directory::read_visible_entries;
        Ok(())
    }

    pub fn read_visible_folders(&mut self) -> UResult<()> {
        self.wr = Directory::read_visible_folders;
        Ok(())
    }
}
#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_sort_entries(&mut self) -> UResult<()> {
        self.ws = sort_by_name;
        Ok(())
    }

    pub fn with_reverse_sort_entries(&mut self) -> UResult<()> {
        self.ws = reverse_sort_by_name;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_permission(&mut self) -> UResult<()> {
        self.wa = Buffer::write_permission;
        self.wha = Buffer::write_permission;
        Ok(())
    }

    pub fn with_no_permission(&mut self) -> UResult<()> {
        self.wa = Buffer::write_no_permission;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_date(&mut self) -> UResult<()> {
        self.wd = Buffer::write_date;
        self.whd = Buffer::write_date;
        Ok(())
    }

    pub fn with_no_date(&mut self) -> UResult<()> {
        self.wd = Buffer::write_no_date;
        Ok(())
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_color_entry(&mut self) -> UResult<()> {
        self.we = Buffer::write_entry_color;
        Ok(())
    }

    pub fn with_colorless_entry(&mut self) -> UResult<()> {
        self.wh = Buffer::write_header_name;
        self.we = Buffer::write_entry;
        Ok(())
    }

    pub fn with_relative_path(&mut self) -> UResult<()> {
        self.we = Buffer::write_entry_relative_path;
        self.wf = Buffer::write_entry_relative_path;
        self.wh = Buffer::write_header_relative_path;
        Ok(())
    }
}

#[allow(dead_code)]
impl<'a> CallbackRegistry<'a> {
    pub fn with_size(&mut self) -> UResult<()> {
        self.wsz = Buffer::write_size;
        Ok(())
    }

    pub fn with_size_color(&mut self) -> UResult<()> {
        self.wsz = Buffer::write_size_color;
        Ok(())
    }

    pub fn with_no_size(&mut self) -> UResult<()> {
        self.wsz = Buffer::write_no_size;
        Ok(())
    }
}
