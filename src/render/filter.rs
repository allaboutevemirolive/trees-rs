use glob::Pattern;
use std::{
    fs::{DirEntry, Metadata},
    path::{Path, PathBuf},
    time::SystemTime,
};

// use super::globfig::GlobFilterConfig;

// Main structs
pub struct DirReader {
    config: DirReaderConfig,
}

#[derive(Clone, Default)]
pub struct DirReaderConfig {
    pub glob_filter: GlobFilterConfig,
    pub general_filter: GeneralFilterConfig,
    pub size_filter: SizeFilterConfig,
    pub time_filter: TimeFilterConfig,
    pub extension_filter: ExtensionFilterConfig,
    pub gitignore: GitignoreConfig,
    pub root_dir: Option<PathBuf>,
}

// Trait for filter configurations
pub trait FilterConfig {
    fn is_enabled(&self) -> bool;
}

#[derive(Clone, Default)]
pub struct GeneralFilterConfig {
    pub ignore_hidden: bool,
    pub only_folders: bool,
    pub follow_links: bool,
    pub max_depth: Option<u32>,
    pub exclude_empty_folders: bool,
}

impl FilterConfig for GeneralFilterConfig {
    fn is_enabled(&self) -> bool {
        self.ignore_hidden
            || self.only_folders
            || self.follow_links
            || self.max_depth.is_some()
            || self.exclude_empty_folders
    }
}

#[derive(Clone, Default)]
pub struct SizeFilterConfig {
    pub min_bytes: Option<u64>,
    pub max_bytes: Option<u64>,
}

impl FilterConfig for SizeFilterConfig {
    fn is_enabled(&self) -> bool {
        self.min_bytes.is_some() || self.max_bytes.is_some()
    }
}

#[derive(Clone, Default)]
pub struct TimeFilterConfig {
    pub min_modified: Option<SystemTime>,
    pub max_modified: Option<SystemTime>,
}

impl FilterConfig for TimeFilterConfig {
    fn is_enabled(&self) -> bool {
        self.min_modified.is_some() || self.max_modified.is_some()
    }
}

#[derive(Clone, Default)]
pub struct ExtensionFilterConfig {
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
}

impl FilterConfig for ExtensionFilterConfig {
    fn is_enabled(&self) -> bool {
        !self.whitelist.is_empty() || !self.blacklist.is_empty()
    }
}

#[derive(Clone, Default)]
pub struct GitignoreConfig {
    pub enabled: bool,
    pub custom_path: Option<PathBuf>,
    pub include_ignored: bool,
    pub exclude_tracked: bool,
    pub use_parent: bool,
}

impl FilterConfig for GitignoreConfig {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// Builder pattern for DirReaderConfig
#[derive(Default)]
pub struct DirReaderConfigBuilder {
    config: DirReaderConfig,
}

impl DirReaderConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn glob_filter(mut self, config: GlobFilterConfig) -> Self {
        self.config.glob_filter = config;
        self
    }

    pub fn general_filter(mut self, config: GeneralFilterConfig) -> Self {
        self.config.general_filter = config;
        self
    }

    pub fn size_filter(mut self, config: SizeFilterConfig) -> Self {
        self.config.size_filter = config;
        self
    }

    pub fn time_filter(mut self, config: TimeFilterConfig) -> Self {
        self.config.time_filter = config;
        self
    }

    pub fn extension_filter(mut self, config: ExtensionFilterConfig) -> Self {
        self.config.extension_filter = config;
        self
    }

    pub fn gitignore(mut self, config: GitignoreConfig) -> Self {
        self.config.gitignore = config;
        self
    }

    pub fn root_dir(mut self, path: PathBuf) -> Self {
        self.config.root_dir = Some(path);
        self
    }

    pub fn build(self) -> DirReaderConfig {
        self.config
    }
}

// Implementation for DirReader
impl DirReader {
    pub fn new(config: DirReaderConfig) -> Self {
        Self { config }
    }

    pub fn with_builder<F>(builder_fn: F) -> Self
    where
        F: FnOnce(DirReaderConfigBuilder) -> DirReaderConfigBuilder,
    {
        let builder = DirReaderConfigBuilder::new();
        let config = builder_fn(builder).build();
        Self::new(config)
    }
}

// Example usage
fn example_usage() {
    let reader = DirReader::with_builder(|b| {
        b.glob_filter(GlobFilterConfig {
            patterns: vec![Pattern::new("*.rs").unwrap()],
            case_sensitive: true,
        })
        .general_filter(GeneralFilterConfig {
            ignore_hidden: true,
            max_depth: Some(5),
            ..Default::default()
        })
    });
}
