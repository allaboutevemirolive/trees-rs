use std::fs;
use std::io;
use tempfile::tempdir;
use tempfile::TempDir;

pub type SortDent = fn(&mut Vec<fs::DirEntry>);

#[derive(Debug, Clone, Copy)]
pub struct SortReg {
    pub func1: SortDent,
}

impl SortReg {
    // let sorter = SortReg::new();
    pub fn new() -> Self {
        Self {
            func1: SortReg::sort_vector_by_name,
        }
    }

    pub fn set(func1: SortDent) -> Self {
        Self { func1 }
    }

    pub fn sort_vector_by_name(vector: &mut Vec<fs::DirEntry>) {
        vector.sort_unstable_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    pub fn reverse_sort_vector_by_name(vector: &mut Vec<fs::DirEntry>) {
        vector.sort_unstable_by(|a, b| b.file_name().cmp(&a.file_name()));
    }
}

fn create_temp_dir_with_entries(entries: &[&str]) -> io::Result<TempDir> {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    for entry in entries {
        fs::File::create(temp_dir.path().join(entry))?;
    }
    Ok(temp_dir)
}

struct Functions {
    func1: fn(&mut Vec<(usize, fs::DirEntry)>),
    func1_original: fn(&mut Vec<(usize, fs::DirEntry)>),
    func2: fn(&mut Vec<(usize, fs::DirEntry)>),
}

#[derive(PartialEq)]
pub enum SortTy {
    ByFilename,
    BySize,
}

fn sort_vector_by_size(vector: &mut Vec<(usize, fs::DirEntry)>) {
    vector.sort_by_key(|&(size, _)| size);
}

fn reverse_sort_vector_by_size(vector: &mut Vec<(usize, fs::DirEntry)>) {
    vector.sort_by_key(|&(size, _)| std::cmp::Reverse(size));
}

pub fn sort_vector_by_name_test(vector: &mut Vec<(usize, fs::DirEntry)>) {
    vector.sort_unstable_by(|a, b| a.1.file_name().cmp(&b.1.file_name()));
}

// =====================================================

fn sort_entries(vector: &mut Vec<fs::DirEntry>, f: SortDent) {
    f(vector)
}

//
pub fn sort_vector_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by(|a, b| a.file_name().cmp(&b.file_name()));
}

//
pub fn reverse_sort_vector_by_name(vector: &mut Vec<fs::DirEntry>) {
    vector.sort_unstable_by(|a, b| b.file_name().cmp(&a.file_name()));
    vector.reverse();
}

// =====================================================

fn reverse_sort_vector_by_name_test(vector: &mut Vec<(usize, fs::DirEntry)>) {
    vector.sort_by(|a, b| b.1.file_name().cmp(&a.1.file_name()));
}

fn sort_vector_by_name_by_folder_first(vector: &mut Vec<(usize, fs::DirEntry)>) {
    vector.sort_by(|a, b| {
        let a_is_dir = a.1.file_type().map_or(false, |ft| ft.is_dir());
        let b_is_dir = b.1.file_type().map_or(false, |ft| ft.is_dir());

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.1.file_name().cmp(&b.1.file_name()),
        }
    });
}

// TODO:
fn reverse_sort_vector_by_name_by_folder_first(vector: &mut Vec<(usize, fs::DirEntry)>) {
    vector.sort_by(|a, b| {
        let a_is_dir = a.1.file_type().map_or(false, |ft| ft.is_dir());
        let b_is_dir = b.1.file_type().map_or(false, |ft| ft.is_dir());

        match (a_is_dir, b_is_dir) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => b.1.file_name().cmp(&a.1.file_name()), // Reversed ordering for file names
        }
    });
}

#[cfg(test)]
mod test_dent {
    use std::fs::DirEntry;

    use super::*;

    // cargo test test_function_pointer -- --nocapture
    #[test]
    fn test_function_pointer_sort_by_size() {
        let mut functions = Functions {
            func1: sort_vector_by_size,
            func1_original: sort_vector_by_size,
            func2: reverse_sort_vector_by_size,
        };

        // Example vector
        let mut example_vector: Vec<(usize, fs::DirEntry)> = vec![];

        // Populate the example vector with directory entries
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().unwrap();
                    let size = metadata.len();
                    example_vector.push((size as usize, entry));
                }
            }
        }

        // By default, execute func1
        (functions.func1)(&mut example_vector);
        println!("Sorted Vector: {:?}", example_vector);

        println!();
        // If user wants to use func2, set it to func1 before executing
        functions.func1 = functions.func2;
        (functions.func1)(&mut example_vector);
        println!("Reverse Sorted Vector: {:?}", example_vector);

        println!();
        // Set func1 back to its original value
        functions.func1 = functions.func1_original;
        (functions.func1)(&mut example_vector);
        println!("Original Sorted Vector: {:?}", example_vector);
    }

    // cargo test test_function_pointer_sort_by_name -- --nocapture
    #[test]
    fn test_function_pointer_sort_by_name() {
        let mut functions = Functions {
            func1: sort_vector_by_name_test,
            func1_original: sort_vector_by_name_test,
            func2: reverse_sort_vector_by_name_test,
        };

        // Example vector
        let mut example_vector: Vec<(usize, fs::DirEntry)> = vec![];

        // Populate the example vector with directory entries
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().unwrap();
                    let size = metadata.len();
                    example_vector.push((size as usize, entry));
                }
            }
        }

        // By default, execute func1
        (functions.func1)(&mut example_vector);
        println!("Sorted Vector by Name: {:?}", example_vector);

        println!();
        // If user wants to use func2, set it to func1 before executing
        functions.func1 = functions.func2;
        (functions.func1)(&mut example_vector);
        println!("Reverse Sorted Vector by Name: {:?}", example_vector);

        println!();
        // Set func1 back to its original value
        functions.func1 = functions.func1_original;
        (functions.func1)(&mut example_vector);
        println!("Original Sorted Vector by Name: {:?}", example_vector);
    }

    // cargo test test_function_pointer_sort_by_name_by_folder -- --nocapture
    #[test]
    fn test_function_pointer_sort_by_name_by_folder() {
        let mut functions = Functions {
            func1: sort_vector_by_name_by_folder_first,
            func1_original: sort_vector_by_name_by_folder_first,
            func2: reverse_sort_vector_by_name_by_folder_first,
        };

        // Example vector
        let mut example_vector: Vec<(usize, fs::DirEntry)> = vec![];

        // Populate the example vector with directory entries
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata().unwrap();
                    let size = metadata.len();
                    example_vector.push((size as usize, entry));
                }
            }
        }

        // By default, execute func1
        (functions.func1)(&mut example_vector);
        println!("Sorted Vector by Name by Folder: {:?}", example_vector);

        println!();
        // If user wants to use func2, set it to func1 before executing
        functions.func1 = functions.func2;
        (functions.func1)(&mut example_vector);
        println!(
            "Reverse Sorted Vector by Name by Folder: {:?}",
            example_vector
        );

        println!();
        // Set func1 back to its original value
        functions.func1 = functions.func1_original;
        (functions.func1)(&mut example_vector);
        println!(
            "Original Sorted Vector by Name by Folder: {:?}",
            example_vector
        );
    }

    // cargo test test_sort_vector_by_name -- --nocapture
    #[test]
    fn test_sort_vector_by_name() {
        let dir_entries = &["file3.txt", "file1.txt", "file2.txt"];

        let temp_dir = create_temp_dir_with_entries(dir_entries).unwrap();

        let mut entries: Vec<DirEntry> = fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect();

        // SortReg::sort_vector_by_name(&mut entries);

        let sorter = SortReg {
            func1: SortReg::sort_vector_by_name,
        };

        // // Access func1 as a method  // Compile-Error!
        // sorter.func1(&mut entries);

        // Access func1 as a field
        (sorter.func1)(&mut entries);

        let sorted_file_names: Vec<_> = entries.iter().map(|entry| entry.file_name()).collect();
        let expected_sorted = vec!["file1.txt", "file2.txt", "file3.txt"];

        assert_eq!(sorted_file_names, expected_sorted);

        temp_dir
            .close()
            .expect("Error removing temporary directory");
    }

    #[test]
    fn test_reverse_sort_vector_by_name() {
        let dir_entries = &["file3.txt", "file1.txt", "file2.txt"];

        let temp_dir = create_temp_dir_with_entries(dir_entries).unwrap();

        let mut entries: Vec<DirEntry> = fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect();

        // SortReg::sort_vector_by_name(&mut entries);

        let sorter = SortReg {
            func1: SortReg::reverse_sort_vector_by_name,
        };

        // // Access func1 as a method  // Compile-Error!
        // sorter.func1(&mut entries);

        // Access func1 as a field
        (sorter.func1)(&mut entries);

        let sorted_file_names: Vec<_> = entries.iter().map(|entry| entry.file_name()).collect();
        let expected_sorted = vec!["file3.txt", "file2.txt", "file1.txt"];

        assert_eq!(sorted_file_names, expected_sorted);

        temp_dir
            .close()
            .expect("Error removing temporary directory");
    }
}
