use std::fs;

pub type FnSortEntries = fn(&mut Vec<fs::DirEntry>);

#[allow(clippy::ptr_arg)]
pub fn sort_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by_key(|a| a.file_name())
}

#[allow(clippy::ptr_arg)]
pub fn reverse_sort_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by_key(|b| std::cmp::Reverse(b.file_name()));
}
