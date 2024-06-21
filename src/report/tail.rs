use crate::error::simple::TResult;

#[derive(PartialEq)]
pub enum ReportMode {
    Default,
    Exhaustive,
}

#[derive(Debug, Clone, Copy)]
pub struct Tail {
    directories: usize,
    files: usize,
    hidden_files: usize,
    symlinks: usize,
    special_files: usize,
    total_items: usize,
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

#[derive(Debug, Clone)]
pub struct ReportSummary {
    report: Vec<String>,
}

impl ReportSummary {
    pub fn with_capacity(cap: i32) -> TResult<Self> {
        Ok(ReportSummary {
            report: Vec::with_capacity(cap as usize),
        })
    }

    pub fn push(&mut self, str: String) {
        self.report.push(str);
    }

    pub fn join(&self, str: &str) -> String {
        self.report.join(str)
    }
}

impl Tail {
    pub fn populate_report(&self, report_summary: &mut ReportSummary, report_mode: ReportMode) {
        let directories = self.directories_to_string().unwrap();
        let directories = format!("{}: {}", directories.1, directories.0);

        let files = self.files_to_string().unwrap();
        let files = format!("{}: {}", files.1, files.0);

        let hidden_files = self.hidden_files_to_string().unwrap();
        let hidden_files = format!("{}: {}", hidden_files.1, hidden_files.0);

        let symlinks = self.symlinks_to_string().unwrap();
        let symlinks = format!("{}: {}", symlinks.1, symlinks.0);

        let special_files = self.special_files_to_string().unwrap();
        let special_files = format!("{}: {}", special_files.1, special_files.0);

        let total_items = self.total_items_to_string().unwrap();
        let total_items = format!("{}: {}", total_items.1, total_items.0);

        let size = self.size_to_string().unwrap();
        let size = format!("Sizes: {} {}", size.0, size.1);

        if report_mode == ReportMode::Default {
            report_summary.push(directories);
            report_summary.push(files);
            report_summary.push(symlinks);
            report_summary.push(size);
        } else {
            report_summary.push(directories);
            report_summary.push(files);
            report_summary.push(hidden_files);
            report_summary.push(symlinks);
            report_summary.push(special_files);
            report_summary.push(total_items);
            report_summary.push(size);
        }
    }
}

#[allow(unused_assignments, dead_code)]
impl Tail {
    fn directories_to_string(&self) -> TResult<(String, String)> {
        let mut dir_str = String::new();

        let directories = self.directories;

        if directories > 1 {
            dir_str = "Directories".to_string();
        } else {
            dir_str = "Directory".to_string();
        }

        let dir_count = format!("{}", directories);

        Ok((dir_count, dir_str))
    }

    fn files_to_string(&self) -> TResult<(String, String)> {
        let mut file_str = String::new();

        let files = self.files;

        if files > 1 {
            file_str = "Files".to_string();
        } else {
            file_str = "File".to_string();
        }

        let file_count = format!("{}", files);

        Ok((file_count, file_str))
    }

    fn hidden_files_to_string(&self) -> TResult<(String, String)> {
        let hidden_files_str = "Hidden".to_string();

        let hidden_files = self.hidden_files;

        let hidden_files_count = format!("{}", hidden_files);

        Ok((hidden_files_count, hidden_files_str))
    }

    fn symlinks_to_string(&self) -> TResult<(String, String)> {
        let mut symlinks_str = String::new();

        let symlinks = self.symlinks;

        if symlinks > 1 {
            symlinks_str = "Symlinks".to_string();
        } else {
            symlinks_str = "Symlink".to_string();
        }

        let symlinks_count = format!("{}", symlinks);

        Ok((symlinks_count, symlinks_str))
    }

    fn special_files_to_string(&self) -> TResult<(String, String)> {
        let mut special_files_str = String::new();

        let special_files = self.special_files;

        if special_files > 1 {
            special_files_str = "Special Files".to_string();
        } else {
            special_files_str = "Special File".to_string();
        }

        let special_files_count = format!("{}", special_files);

        Ok((special_files_count, special_files_str))
    }

    fn total_items_to_string(&self) -> TResult<(String, String)> {
        let mut total_items_str = String::new();

        let total_items = self.total_items;

        if total_items > 1 {
            total_items_str = "Total Items".to_string();
        } else {
            total_items_str = "Total Item".to_string();
        }

        let total_items_count = format!("{}", total_items);

        Ok((total_items_count, total_items_str))
    }

    fn size_to_string(&self) -> TResult<(String, String)> {
        let size = self.size as f64;
        let size_count: f64;

        let mut unit_count = String::new();
        let mut unit_str = String::new();

        let two_gb_in_bytes = 2.0 * 1024.0 * 1024.0 * 1024.0; // 2147483648.0 @ 2 gigabytes

        let one_gb_in_bytes = 1.0 * 1024.0 * 1024.0 * 1024.0; // 1073741824.0 @ 1 gigabyte

        if size > two_gb_in_bytes {
            unit_str = "Gigabytes".to_string();
            size_count = size / 1_073_741_824.0;
            unit_count = format!("{:.3}", size_count);
        } else if size < two_gb_in_bytes && size > one_gb_in_bytes {
            unit_str = "Gigabyte".to_string();
            size_count = size / 1_073_741_824.0;
            unit_count = format!("{:.3}", size_count);
        } else if size < one_gb_in_bytes {
            unit_str = "bytes".to_string();
            size_count = size;
            unit_count = format!("{}", size_count);
        }

        Ok((unit_count, unit_str))
    }
}
