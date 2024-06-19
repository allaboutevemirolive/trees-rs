// See: https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124
pub struct FileColors {
    pub directory: DirectoryColor,
    pub file: FileColor,
    pub symlink: SymlinkColor,
    pub target_symlink: TargetSymlinkColor,
    pub special_file: SpecialFile,

    // Attributes
    pub atime: AtimeColor, // Access time color
    pub btime: BtimeColor, // Birth time color (if available)
    pub mtime: MtimeColor, // Modification time color
    pub pms: PmsColor,     // POSIX file permissions color
    pub size: SizeColor,   // File size color
}

impl FileColors {
    pub fn new() -> Self {
        Self {
            directory: DirectoryColor::new(),
            file: FileColor::new(),
            symlink: SymlinkColor::new(),
            target_symlink: TargetSymlinkColor::new(),
            special_file: SpecialFile::new(),
            atime: AtimeColor::new(),
            btime: BtimeColor::new(),
            mtime: MtimeColor::new(),
            pms: PmsColor::new(),
            size: SizeColor::new(),
        }
    }

    pub fn disable_color(&mut self) {
        self.directory.disable_color();
        self.file.disable_color();
        self.symlink.disable_color();
        self.target_symlink.disable_color();
        self.special_file.disable_color();
        self.atime.disable_color();
        self.btime.disable_color();
        self.mtime.disable_color();
        self.pms.disable_color();
        self.size.disable_color();
    }
}

pub struct DirectoryColor {
    open_color: String,
    closed_color: String,
}

impl DirectoryColor {
    pub fn new() -> Self {
        Self {
            open_color: "\x1b[0;34m".to_string(), // blue
            closed_color: "\x1b[0m".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct SymlinkColor {
    open_color: String,
    closed_color: String,
}

impl SymlinkColor {
    pub fn new() -> Self {
        Self {
            open_color: "\x1b[0;33m".to_string(), // yellow
            closed_color: "\x1b[0m".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct TargetSymlinkColor {
    open_color: String,
    closed_color: String,
}

impl TargetSymlinkColor {
    pub fn new() -> Self {
        Self {
            open_color: "\x1b[4;34m".to_string(), // Underlined blue
            closed_color: "\x1b[0m".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

// TODO: We dont have official color for file yet
pub struct FileColor {
    open_color: String,
    closed_color: String,
}

impl FileColor {
    pub fn new() -> Self {
        Self {
            open_color: "".to_string(),
            closed_color: "".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct SpecialFile {
    open_color: String,
    closed_color: String,
}

impl SpecialFile {
    pub fn new() -> Self {
        Self {
            open_color: "\x1b[41m".to_string(), // Bold red
            closed_color: "\x1b[0m".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}
pub struct AtimeColor {
    open_color: String,
    closed_color: String,
}

impl AtimeColor {
    pub fn new() -> Self {
        Self {
            open_color: "".to_string(),
            closed_color: "".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct BtimeColor {
    open_color: String,
    closed_color: String,
}

impl BtimeColor {
    pub fn new() -> Self {
        Self {
            open_color: "".to_string(),
            closed_color: "".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct MtimeColor {
    open_color: String,
    closed_color: String,
}

impl MtimeColor {
    pub fn new() -> Self {
        Self {
            open_color: "".to_string(),
            closed_color: "".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct PmsColor {
    open_color: String,
    closed_color: String,
}

impl PmsColor {
    pub fn new() -> Self {
        Self {
            open_color: "".to_string(),
            closed_color: "".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}

pub struct SizeColor {
    open_color: String,
    closed_color: String,
}

impl SizeColor {
    pub fn new() -> Self {
        Self {
            open_color: "\x1B[1;32m".to_string(), // green
            closed_color: "\x1b[0m".to_string(),
        }
    }

    pub fn disable_color(&mut self) {
        self.open_color = "".to_string();
        self.closed_color = "".to_string();
    }

    pub fn open_color(&self) -> String {
        self.open_color.clone()
    }

    pub fn closed_color(&self) -> String {
        self.closed_color.clone()
    }
}
