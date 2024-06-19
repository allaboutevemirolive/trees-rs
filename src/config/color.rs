pub struct FileColors {
    pub directory: DirectoryColor,
    pub file: FileColor,
    pub symlink: SymlinkColor,
}

impl FileColors {
    pub fn new() -> Self {
        Self {
            directory: DirectoryColor::new(),
            file: FileColor::new(),
            symlink: SymlinkColor::new(),
        }
    }

    pub fn disable_color(&mut self) {
        self.directory.disable_color();
        self.file.disable_color();
        self.symlink.disable_color()
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
