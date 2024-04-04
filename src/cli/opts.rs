use std::fs;

use crate::config::path::WhichPath;
use crate::sort::dent::{reverse_sort_vector_by_name, sort_vector_by_name, SortTy, WhichSort};

pub struct OptionArgs {
    pub current_dir: WhichPath,
    pub sort_ty: SortTy,
    // pub sort_dent: fn(&mut Vec<fs::DirEntry>),
}

impl OptionArgs {
    pub fn new() -> Self {
        let w = WhichPath::CurrentDir;
        let sort = SortTy::ByFilename;

        let mut sort_dent: WhichSort;

        sort_dent = reverse_sort_vector_by_name;

        if sort == SortTy::ByFilename {
            sort_dent = sort_vector_by_name
        }

        Self {
            current_dir: w,
            sort_ty: sort,
            // sort_dent,
        }
    }
}
