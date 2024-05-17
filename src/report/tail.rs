use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Tail {
    pub directories: usize,
    pub files: usize,
    pub size: u64,
    pub hidden_files: usize,
    pub symlinks: usize,
}

impl Default for Tail {
    fn default() -> Self {
        Tail {
            directories: 1,
            files: 0,
            size: 0,
            hidden_files: 0,
            symlinks: 0,
        }
    }
}

impl Tail {
    #[allow(dead_code)]
    pub fn initialize(
        directories: usize,
        files: usize,
        size: u64,
        hidden_files: usize,
        symlinks: usize,
    ) -> Self {
        Self {
            directories,
            files,
            size,
            hidden_files,
            symlinks,
        }
    }

    pub fn dir_plus_one(&mut self) {
        self.directories += 1
    }

    pub fn file_plus_one(&mut self) {
        self.files += 1
    }

    pub fn hid_plus_one(&mut self) {
        self.hidden_files += 1
    }

    pub fn sym_plus_one(&mut self) {
        self.symlinks += 1
    }

    pub fn add_size(&mut self, size: u64) {
        self.size += size
    }
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gigabytes = self.size as f64 / 1_073_741_824.0;
        let gigabytes = format!("{:.3}", gigabytes);

        #[allow(unused_assignments)]
        let mut dirr = String::new();

        #[allow(unused_assignments)]
        let mut filee = String::new();

        #[allow(unused_assignments)]
        let mut sym_str = String::new();

        let hiddenn = String::from("hidden");

        #[allow(unused_assignments)]
        let mut gbyte = String::new();

        if self.directories <= 1 {
            dirr = "directory".to_string();
        } else {
            dirr = "directories".to_string();
        }

        if self.files <= 1 {
            filee = "file".to_string();
        } else {
            filee = "files".to_string();
        }

        if self.symlinks <= 1 {
            sym_str = "symlink".to_string();
        } else {
            sym_str = "symlinks".to_string();
        }

        if gigabytes.parse::<f64>().unwrap_or_default() <= 0.001 {
            gbyte = "gigabyte".to_string();
        } else {
            gbyte = "gigabytes".to_string();
        }

        let reportt = format!(
            "{} {}, {} {}, {} {}, {} {}, {} {}",
            self.directories,
            dirr,
            self.files,
            filee,
            self.hidden_files,
            hiddenn,
            self.symlinks,
            sym_str,
            gigabytes,
            gbyte
        );

        write!(f, "{}", reportt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let tail = Tail::initialize(10, 20, 100, 5, 0);
        assert_eq!(tail.directories, 10);
        assert_eq!(tail.files, 20);
        assert_eq!(tail.size, 100);
        assert_eq!(tail.hidden_files, 5);
    }

    // #[test]
    // fn test_dir_plus_one() {
    //     let mut tail = Tail::initialize(10, 20, 100, 5, 0);
    //     tail.dir_plus_one();
    //     assert_eq!(tail.directories, 11);
    // }

    // #[test]
    // fn test_file_plus_one() {
    //     let mut tail = Tail::initialize(10, 20, 100, 5);
    //     tail.file_plus_one();
    //     assert_eq!(tail.files, 21);
    // }

    // #[test]
    // fn test_hid_plus_one() {
    //     let mut tail = Tail::initialize(10, 20, 100, 5);
    //     tail.hid_plus_one();
    //     assert_eq!(tail.hidden_files, 6);
    // }

    // #[test]
    // fn test_add_size() {
    //     let mut tail = Tail::initialize(10, 20, 100, 5);
    //     tail.add_size(50);
    //     assert_eq!(tail.size, 150);
    // }
}
