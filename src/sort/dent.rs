use std::fs;
use std::fs::DirEntry;

pub type FnSortEntries = fn(&mut Vec<fs::DirEntry>);

pub fn which_sort(sort: FnSortEntries, entries: &mut Vec<DirEntry>) {
    (sort)(entries);
}

#[allow(clippy::ptr_arg)]
pub fn sort_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by_key(|a| a.file_name())
}

#[allow(clippy::ptr_arg)]
pub fn reverse_sort_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by_key(|b| std::cmp::Reverse(b.file_name()));
}

#[allow(clippy::ptr_arg)]
#[allow(dead_code)]
pub fn sort_by_file_first(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by(|a, b| {
        let a_is_dir = a.file_type().map_or(false, |ft| ft.is_dir());
        let b_is_dir = b.file_type().map_or(false, |ft| ft.is_dir());

        if a_is_dir == b_is_dir {
            a.file_name().cmp(&b.file_name())
        } else if a_is_dir {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
}
