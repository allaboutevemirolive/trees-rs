use std::fs;

pub type WhichSort = fn(&mut Vec<fs::DirEntry>);

// fn sort_entries(vector: &mut Vec<fs::DirEntry>, f: WhichSort) {
//     f(vector)
// }

pub fn sort_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by(|a, b| a.file_name().cmp(&b.file_name()));
}

pub fn reverse_sort_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by(|a, b| b.file_name().cmp(&a.file_name()));
}
