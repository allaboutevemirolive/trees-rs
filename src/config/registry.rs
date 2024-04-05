use std::io::StdoutLock;

use crate::{
    canva::buffer::{Buffer, WhichPaint},
    config::path::WhichReader,
    error::simple::UResult,
    sort::dent::{reverse_sort_by_name, sort_by_name, WhichSort},
};

use super::path::Directory;

#[derive(Debug, Clone, Copy)]
pub struct CallbackRegistry<'a> {
    pub wr: WhichReader,
    pub ws: WhichSort,
    pub wp: WhichPaint<StdoutLock<'a>>,
}

impl<'a> CallbackRegistry<'a> {
    pub fn new() -> UResult<Self> {
        let wr: WhichReader = Directory::read_visible_entries;
        let ws: WhichSort = sort_by_name;
        let wp: WhichPaint<StdoutLock> = Buffer::write_dir_name_color;
        Ok(Self { wr, ws, wp })
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn read_all_entries(&self) -> UResult<Self> {
        Ok(Self {
            wr: Directory::read_all_entries,
            ws: self.ws,
            wp: self.wp,
        })
    }

    pub fn read_visible_entries(&self) -> UResult<Self> {
        Ok(Self {
            wr: Directory::read_visible_entries,
            ws: self.ws,
            wp: self.wp,
        })
    }

    pub fn read_visible_folders(&self) -> UResult<Self> {
        Ok(Self {
            wr: Directory::read_visible_folders,
            ws: self.ws,
            wp: self.wp,
        })
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_sort_entries(&self) -> UResult<Self> {
        Ok(Self {
            wr: self.wr,
            ws: sort_by_name,
            wp: self.wp,
        })
    }

    pub fn with_reverse_sort_entries(&self) -> UResult<Self> {
        Ok(Self {
            wr: self.wr,
            ws: reverse_sort_by_name,
            wp: self.wp,
        })
    }
}

impl<'a> CallbackRegistry<'a> {
    pub fn with_color_entry(&self) -> UResult<Self> {
        Ok(Self {
            wr: self.wr,
            ws: self.ws,
            wp: Buffer::write_dir_name_color,
        })
    }

    pub fn with_colorless_entry(&self) -> UResult<Self> {
        Ok(Self {
            wr: self.wr,
            ws: self.ws,
            wp: Buffer::write_dir_name,
        })
    }
}
