use std::io::StdoutLock;

use crate::canva::buffer::{Buffer, WhichPaint};
use crate::config::path::WhichReader;
use crate::error::simple::UResult;
use crate::sort::dent::{sort_vector_by_name, WhichSort};

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
        let ws = sort_vector_by_name;
        let wp: WhichPaint<StdoutLock> = Buffer::write_dir_name_color;
        Ok(Self { wr, ws, wp })
    }

    // pub fn new(dir_path: DirPath<DirEntry>, sort_dent: SortReg) -> Self {
    //     let dp: DirPath<DirEntry> = Directory::read_visible_entries;
    //     let sd = SortReg::new();
    //     Self { dp, sd }
    // }
}
