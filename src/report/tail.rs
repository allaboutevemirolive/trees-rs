use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Tail {
    pub directories: usize,
    pub files: usize,
    pub size: u64,
    pub hidden_files: usize,
}

impl Tail {
    #[allow(dead_code)]
    pub fn initialize(directories: usize, files: usize, size: u64, hidden_files: usize) -> Self {
        Self {
            directories,
            files,
            size,
            hidden_files,
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

    pub fn add_size(&mut self, size: u64) {
        self.size += size
    }
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gigabytes = self.size as f64 / 1_073_741_824.0;
        let gigabytes = format!("{:.2}", gigabytes);

        write!(
            f,
            "{} directories, {} files, {} hidden, {} gigabytes",
            self.directories, self.files, self.hidden_files, gigabytes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let tail = Tail::initialize(10, 20, 100, 5);
        assert_eq!(tail.directories, 10);
        assert_eq!(tail.files, 20);
        assert_eq!(tail.size, 100);
        assert_eq!(tail.hidden_files, 5);
    }

    #[test]
    fn test_dir_plus_one() {
        let mut tail = Tail::initialize(10, 20, 100, 5);
        tail.dir_plus_one();
        assert_eq!(tail.directories, 11);
    }

    #[test]
    fn test_file_plus_one() {
        let mut tail = Tail::initialize(10, 20, 100, 5);
        tail.file_plus_one();
        assert_eq!(tail.files, 21);
    }

    #[test]
    fn test_hid_plus_one() {
        let mut tail = Tail::initialize(10, 20, 100, 5);
        tail.hid_plus_one();
        assert_eq!(tail.hidden_files, 6);
    }

    #[test]
    fn test_add_size() {
        let mut tail = Tail::initialize(10, 20, 100, 5);
        tail.add_size(50);
        assert_eq!(tail.size, 150);
    }
}
