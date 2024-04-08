use std::io::StdoutLock;

use crate::{
    canva::buffer::{
        Buffer, WhichAttribute, WhichDate, WhichEntry, WhichFile, WhichHeader,
        WhichHeaderAttribute, WhichHeaderDate,
    },
    config::path::WhichReader,
    error::simple::UResult,
    sort::dent::{reverse_sort_by_name, sort_by_name, WhichSort},
};

use super::path::Directory;

#[derive(Debug, Clone, Copy)]
pub struct CallbackRegistry<'a> {
    pub wr: WhichReader,
    pub ws: WhichSort,
    pub wa: WhichAttribute<StdoutLock<'a>>,
    pub wd: WhichDate<StdoutLock<'a>>,
    /// Folder    
    pub we: WhichEntry<StdoutLock<'a>>,
    pub wf: WhichFile<StdoutLock<'a>>,
    pub wh: WhichHeader<StdoutLock<'a>>,
    pub wha: WhichHeaderAttribute<StdoutLock<'a>>,
    pub whd: WhichHeaderDate<StdoutLock<'a>>,
}

impl<'a> CallbackRegistry<'a> {
    pub fn new() -> UResult<Self> {
        let wr: WhichReader = Directory::read_visible_entries;
        let ws: WhichSort = sort_by_name;
        let wa: WhichAttribute<StdoutLock> = Buffer::write_no_attribute;
        let wd: WhichDate<StdoutLock> = Buffer::write_no_date;
        let we: WhichEntry<StdoutLock> = Buffer::write_dirname_color;
        let wf: WhichFile<StdoutLock> = Buffer::write_filename;
        let wh: WhichHeader<StdoutLock> = Buffer::write_header_name;
        let wha: WhichHeaderAttribute<StdoutLock> = Buffer::write_no_header_attribute;
        let whd: WhichHeaderDate<StdoutLock> = Buffer::write_no_header_date;
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
        })
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn read_all_entries(&mut self) -> UResult<()> {
        Ok(self.wr = Directory::read_all_entries)
    }

    pub fn read_visible_entries(&mut self) -> UResult<()> {
        Ok(self.wr = Directory::read_visible_entries)
    }

    pub fn read_visible_folders(&mut self) -> UResult<()> {
        Ok(self.wr = Directory::read_visible_folders)
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_sort_entries(&mut self) -> UResult<()> {
        Ok(self.ws = sort_by_name)
    }

    pub fn with_reverse_sort_entries(&mut self) -> UResult<()> {
        Ok(self.ws = reverse_sort_by_name)
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_attributes(&mut self) -> UResult<()> {
        self.wa = Buffer::write_attribute;
        self.wha = Buffer::write_header_attribute;
        Ok(())
    }

    pub fn with_no_attributes(&mut self) -> UResult<()> {
        Ok(self.wa = Buffer::write_no_attribute)
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_date(&mut self) -> UResult<()> {
        self.wd = Buffer::write_date;
        self.whd = Buffer::write_header_date;
        Ok(())
    }

    pub fn with_no_date(&mut self) -> UResult<()> {
        Ok(self.wd = Buffer::write_no_date)
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_color_entry(&mut self) -> UResult<()> {
        Ok(self.we = Buffer::write_dirname_color)
    }

    pub fn with_colorless_entry(&mut self) -> UResult<()> {
        Ok(self.we = Buffer::write_dirname)
    }

    pub fn with_relative_path(&mut self) -> UResult<()> {
        self.we = Buffer::write_relative_path;
        self.wf = Buffer::write_file_relative_path;
        self.wh = Buffer::write_header_relative_path;
        Ok(())
    }
}
