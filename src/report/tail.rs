use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tail {
    #[serde(serialize_with = "serialize_directories")]
    directories: usize,
    #[serde(serialize_with = "serialize_files")]
    files: usize,
    #[serde(serialize_with = "serialize_hidden_files")]
    hidden_files: usize,
    #[serde(serialize_with = "serialize_symlinks")]
    symlinks: usize,
    #[serde(serialize_with = "serialize_special_files")]
    special_files: usize,
    #[serde(serialize_with = "serialize_total_items")]
    total_items: usize,
    #[serde(serialize_with = "serialize_size")]
    size: u64,
}

impl Default for Tail {
    fn default() -> Self {
        Tail {
            directories: 1,
            files: 0,
            size: 0,
            hidden_files: 0,
            symlinks: 0,
            total_items: 0,
            special_files: 0,
        }
    }
}

impl Tail {
    pub fn dir_add_one(&mut self) {
        self.directories += 1
    }

    pub fn file_add_one(&mut self) {
        self.files += 1
    }

    pub fn hidden_add_one(&mut self) {
        self.hidden_files += 1
    }

    pub fn symlink_add_one(&mut self) {
        self.symlinks += 1
    }

    pub fn add_size(&mut self, size: u64) {
        self.size += size
    }

    pub fn special_add_one(&mut self) {
        self.special_files += 1
    }

    /// Accumulate all items except hidden files.
    ///
    /// If user want to include hidden files, pass `--all` in the arguments
    pub fn accumulate_items(&mut self) {
        self.total_items = self.directories + self.files + self.symlinks + self.special_files;
    }
}

fn serialize_total_items<S>(total_items: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let human_readable = format!("{}", total_items);
    serializer.serialize_str(&human_readable)
}

fn serialize_symlinks<S>(files: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let human_readable = format!("{}", files);
    serializer.serialize_str(&human_readable)
}

fn serialize_special_files<S>(files: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let human_readable = format!("{}", files);
    serializer.serialize_str(&human_readable)
}

fn serialize_hidden_files<S>(files: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let human_readable = format!("{}", files);
    serializer.serialize_str(&human_readable)
}

fn serialize_files<S>(files: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let human_readable = format!("{}", files);
    serializer.serialize_str(&human_readable)
}

fn serialize_directories<S>(directories: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let human_readable = format!("{}", directories);
    serializer.serialize_str(&human_readable)
}

fn serialize_size<S>(size: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let gigabytes = *size as f64 / 1_073_741_824.0;

    let gigabytes = format!("{:.3}", gigabytes);

    let mut _gbyte = String::new();

    if gigabytes.parse::<f64>().unwrap_or_default() <= 0.001 {
        _gbyte = "gigabyte".to_string();
    } else {
        _gbyte = "gigabytes".to_string();
    }

    let mut _human_readable = String::new();

    if gigabytes.parse::<f64>().unwrap_or_default() <= 0.000 {
        _human_readable = format!("{} bytes", size);
    } else {
        _human_readable = format!("{} {}", gigabytes, _gbyte);
    }

    serializer.serialize_str(&_human_readable)
}
