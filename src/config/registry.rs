use std::io::StdoutLock;

use crate::canva::buffer::{Buffer, WhichPaint};
use crate::config::path::DirPath;
use crate::sort::dent::SortReg;

use super::path::Directory;

#[derive(Debug, Clone, Copy)]
pub struct CallbackRegistry<'a> {
    pub dp: DirPath,
    pub sd: SortReg,
    pub wp: WhichPaint<StdoutLock<'a>>,
}

impl<'a> CallbackRegistry<'a> {
    pub fn new() -> Self {
        let dp: DirPath = Directory::read_visible_entries;
        let sd = SortReg::new();
        let wp: WhichPaint<StdoutLock> = Buffer::write_dir_name_color;
        Self { dp, sd, wp }
    }

    // pub fn new(dir_path: DirPath<DirEntry>, sort_dent: SortReg) -> Self {
    //     let dp: DirPath<DirEntry> = Directory::read_visible_entries;
    //     let sd = SortReg::new();
    //     Self { dp, sd }
    // }
}
