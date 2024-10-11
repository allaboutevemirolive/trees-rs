#[derive(PartialEq, Clone, Copy)]
pub enum ReportMode {
    Default,
    Exhaustive,
}

#[derive(Debug, Clone, Copy)]
pub struct DirectoryStats {
    directories: usize,
    files: usize,
    media: usize,
    hidden_files: usize,
    symlinks: usize,
    special_files: usize,
    total_items: usize,
    size: u64,
    err_directories: u64,
}

impl Default for DirectoryStats {
    fn default() -> Self {
        DirectoryStats {
            directories: 1,
            files: 0,
            media: 0,
            size: 0,
            hidden_files: 0,
            symlinks: 0,
            total_items: 0,
            special_files: 0,
            err_directories: 0,
        }
    }
}

impl DirectoryStats {
    pub fn add_size(&mut self, size: u64) {
        tracing::info!("Accumulating entry's size: {}", size);
        self.size += size;
    }

    pub fn accumulate_items(&mut self) {
        self.total_items = self.directories
            + self.files
            + self.symlinks
            + self.special_files
            + self.media
            + (self.err_directories as usize);
    }

    // Add the missing increment methods
    pub fn symlink_add_one(&mut self) {
        self.symlinks += 1;
    }

    // Add the missing increment methods
    pub fn hidden_add_one(&mut self) {
        self.hidden_files += 1;
    }

    pub fn media_add_one(&mut self) {
        self.media += 1;
    }

    pub fn file_add_one(&mut self) {
        self.files += 1;
    }

    pub fn dir_add_one(&mut self) {
        self.directories += 1;
    }

    pub fn err_dirs_add_one(&mut self) {
        self.err_directories += 1;
    }

    pub fn special_add_one(&mut self) {
        self.special_files += 1;
    }

    pub fn populate_report(&self, report: &mut ReportSummary, report_mode: ReportMode) {
        let generated_report = self.generate_report(report_mode);
        for item in generated_report.report {
            report.push(item);
        }
    }

    pub fn generate_report(&self, report_mode: ReportMode) -> ReportSummary {
        let mut report = ReportSummary::with_capacity(8).expect("Failed to create report summary");

        // Boxed closures for consistent type
        let formatters: Vec<Box<dyn Fn(&ReportMode) -> anyhow::Result<String>>> = vec![
            Box::new(|mode| self.format_directories(mode)),
            Box::new(|mode| self.format_files(mode)),
            Box::new(|mode| self.format_hidden_files(mode)),
            Box::new(|mode| self.format_symlinks(mode)),
            Box::new(|mode| self.format_media_files(mode)),
            Box::new(|mode| self.format_total_items(mode)),
            Box::new(|mode| self.format_size(mode)),
        ];

        for formatter in formatters.iter() {
            if let Ok(formatted) = formatter(&report_mode) {
                report.push(formatted);
            }
        }

        report
    }

    fn format_stat(
        &self,
        value: usize,
        singular: &str,
        plural: &str,
        short: &str,
        mode: &ReportMode,
    ) -> String {
        let label = match mode {
            ReportMode::Default => short.to_string(),
            ReportMode::Exhaustive => {
                if value != 1 {
                    plural.to_string()
                } else {
                    singular.to_string()
                }
            }
        };
        format!("{}: {}", label, value)
    }

    fn format_directories(&self, mode: &ReportMode) -> anyhow::Result<String> {
        Ok(self.format_stat(self.directories, "Directory", "Directories", "D", mode))
    }

    fn format_files(&self, mode: &ReportMode) -> anyhow::Result<String> {
        Ok(self.format_stat(self.files, "File", "Files", "F", mode))
    }

    fn format_hidden_files(&self, mode: &ReportMode) -> anyhow::Result<String> {
        Ok(self.format_stat(self.hidden_files, "Hidden", "Hidden", "H", mode))
    }

    fn format_symlinks(&self, mode: &ReportMode) -> anyhow::Result<String> {
        Ok(self.format_stat(self.symlinks, "Symlink", "Symlinks", "SY", mode))
    }

    fn format_media_files(&self, _mode: &ReportMode) -> anyhow::Result<String> {
        Ok(format!("M: {}", self.media))
    }

    fn format_total_items(&self, mode: &ReportMode) -> anyhow::Result<String> {
        Ok(self.format_stat(self.total_items, "Total Item", "Total Items", "T", mode))
    }

    fn format_size(&self, mode: &ReportMode) -> anyhow::Result<String> {
        let (value, unit) = self.get_formatted_size();
        let size_label = if *mode == ReportMode::Default {
            "SZ"
        } else {
            "Size"
        };
        Ok(format!("{}: {} {}", size_label, value, unit))
    }

    fn get_formatted_size(&self) -> (String, String) {
        const GB: f64 = 1024.0 * 1024.0 * 1024.0;
        let size = self.size as f64;

        if size > GB {
            let gb_size = size / GB;
            (format!("{:.3}", gb_size), "GB".to_string())
        } else {
            (format!("{}", size), "bytes".to_string())
        }
    }

    pub fn hidden_files(&self) -> usize {
        self.hidden_files
    }
}

#[derive(Debug, Clone)]
pub struct ReportSummary {
    report: Vec<String>,
}

impl ReportSummary {
    pub fn with_capacity(cap: i32) -> anyhow::Result<Self> {
        Ok(ReportSummary {
            report: Vec::with_capacity(cap as usize),
        })
    }

    pub fn push(&mut self, str: String) {
        self.report.push(str);
    }

    pub fn join(&self, separator: &str) -> String {
        self.report.join(separator)
    }
}
