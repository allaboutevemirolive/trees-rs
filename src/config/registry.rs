use super::{
    inspect::{read_all_entries, read_visible_entries, read_visible_folders, FnReadDir},
    sorting::{reverse_sort_by_name, sort_by_file_first, sort_by_name, FnSortEntries},
};
use crate::{
    render::{
        attr::{
            atime::FnExtAccessTime, btime::FnExtBTime, mtime::FnExtModTime, pms::FnExtPermission,
            size::FnExtSize,
        },
        buffer::Buffer,
        color::FnColor,
        entree::{dirr::FnOutDir, filee::FnOutFile, headd::FnOutHead, symlinked::FnOutSymlink},
    },
    report::stats::DirectoryStats,
};
use std::{
    fs::DirEntry,
    io::{self, StdoutLock},
    path::PathBuf,
};

#[derive(Debug, Clone, Copy)]
pub struct Registry<'a> {
    read: FnReadDir,
    sort: FnSortEntries,
    colors: ColorFunctions<'a>,
    entries: EntryFunctions<'a>,
    metadata: MetadataFunctions<'a>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ColorFunctions<'a> {
    reset: FnColor<StdoutLock<'a>>,
    yellow: FnColor<StdoutLock<'a>>,
    bold_red: FnColor<StdoutLock<'a>>,
    underlined_blue: FnColor<StdoutLock<'a>>,
    blue: FnColor<StdoutLock<'a>>,
    green: FnColor<StdoutLock<'a>>,
    purple: FnColor<StdoutLock<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct EntryFunctions<'a> {
    dir: FnOutDir<StdoutLock<'a>>,
    file: FnOutFile<StdoutLock<'a>>,
    symlink: FnOutSymlink<StdoutLock<'a>>,
    head: FnOutHead<StdoutLock<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct MetadataFunctions<'a> {
    permission: FnExtPermission<StdoutLock<'a>>,
    birth_time: FnExtBTime<StdoutLock<'a>>,
    mod_time: FnExtModTime<StdoutLock<'a>>,
    access_time: FnExtAccessTime<StdoutLock<'a>>,
    size: FnExtSize<StdoutLock<'a>>,
}

pub trait Color<'a> {
    fn reset(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
    fn yellow(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
    fn bold_red(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
    fn underlined_blue(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
    fn blue(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
    fn green(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
    fn purple(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()>;
}

impl<'a> Color<'a> for Registry<'a> {
    fn reset(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.reset)(buf)
    }

    fn yellow(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.yellow)(buf)
    }

    fn bold_red(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.bold_red)(buf)
    }

    fn underlined_blue(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.underlined_blue)(buf)
    }

    fn blue(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.blue)(buf)
    }

    fn green(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.green)(buf)
    }

    fn purple(&self, buf: &mut Buffer<StdoutLock<'a>>) -> io::Result<()> {
        (self.colors.purple)(buf)
    }
}

impl<'a> Registry<'a> {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            read: read_visible_entries,
            sort: sort_by_name,
            colors: ColorFunctions::default(),
            entries: EntryFunctions::default(),
            metadata: MetadataFunctions::default(),
        })
    }

    pub fn inspt_dents(
        &self,
        path: PathBuf,
        dir_stats: &mut DirectoryStats,
    ) -> anyhow::Result<Vec<DirEntry>> {
        tracing::info!("Inspect directory's entries for path: {:?}", path.display());
        (self.read)(path, dir_stats)
    }

    pub fn sort_dents(&self, entries: &mut Vec<DirEntry>) {
        tracing::info!("Sort entries");
        (self.sort)(entries)
    }
}

// Read configuration methods
impl<'a> Registry<'a> {
    pub fn read_all_entries(&mut self) -> anyhow::Result<()> {
        self.read = read_all_entries;
        Ok(())
    }

    pub fn read_visible_entries(&mut self) -> anyhow::Result<()> {
        self.read = read_visible_entries;
        Ok(())
    }

    pub fn read_visible_folders(&mut self) -> anyhow::Result<()> {
        self.read = read_visible_folders;
        Ok(())
    }
}

// Sort configuration methods
impl<'a> Registry<'a> {
    pub fn with_sort_entries(&mut self) -> anyhow::Result<()> {
        self.sort = sort_by_name;
        Ok(())
    }

    pub fn with_reverse_sort_entries(&mut self) -> anyhow::Result<()> {
        self.sort = reverse_sort_by_name;
        Ok(())
    }

    pub fn with_sort_by_file_first(&mut self) -> anyhow::Result<()> {
        self.sort = sort_by_file_first;
        Ok(())
    }
}

// Default implementations for sub-structs
impl<'a> Default for ColorFunctions<'a> {
    fn default() -> Self {
        Self {
            reset: Buffer::reset_color,
            yellow: Buffer::yellow,
            bold_red: Buffer::bold_red,
            underlined_blue: Buffer::underlined_blue,
            blue: Buffer::blue,
            green: Buffer::green,
            purple: Buffer::purple,
        }
    }
}

impl<'a> Default for EntryFunctions<'a> {
    fn default() -> Self {
        Self {
            dir: Buffer::write_dir,
            file: Buffer::write_entry,
            symlink: Buffer::write_symlink,
            head: Buffer::write_header_name,
        }
    }
}

impl<'a> Default for MetadataFunctions<'a> {
    fn default() -> Self {
        Self {
            permission: Buffer::write_no_permission,
            birth_time: Buffer::write_no_btime,
            mod_time: Buffer::write_no_mtime,
            access_time: Buffer::write_no_atime,
            size: Buffer::write_no_size,
        }
    }
}

// Color Metadata configuration methods
impl<'a> Registry<'a> {
    pub fn with_color(&mut self) -> anyhow::Result<()> {
        self.colors = ColorFunctions::default();
        Ok(())
    }

    pub fn with_no_color(&mut self) -> anyhow::Result<()> {
        self.colors = ColorFunctions {
            reset: Buffer::no_color,
            yellow: Buffer::no_color,
            bold_red: Buffer::no_color,
            underlined_blue: Buffer::no_color,
            blue: Buffer::no_color,
            green: Buffer::no_color,
            purple: Buffer::no_color,
        };
        Ok(())
    }
}

// Metadata configuration methods
impl<'a> Registry<'a> {
    /// Configures the registry to display file permissions
    pub fn with_permission(&mut self) -> &mut Self {
        self.metadata.permission = Buffer::write_permission;
        self
    }

    /// Configures the registry to hide file permissions
    pub fn without_permission(&mut self) -> &mut Self {
        self.metadata.permission = Buffer::write_no_permission;
        self
    }

    /// Configures the registry to display birth times
    pub fn with_birth_time(&mut self) -> &mut Self {
        self.metadata.birth_time = Buffer::write_btime;
        self
    }

    /// Configures the registry to hide birth times
    pub fn without_birth_time(&mut self) -> &mut Self {
        self.metadata.birth_time = Buffer::write_no_btime;
        self
    }

    /// Configures the registry to display modification times
    pub fn with_modification_time(&mut self) -> &mut Self {
        self.metadata.mod_time = Buffer::write_mtime;
        self
    }

    /// Configures the registry to hide modification times
    pub fn without_modification_time(&mut self) -> &mut Self {
        self.metadata.mod_time = Buffer::write_no_mtime;
        self
    }

    /// Configures the registry to display access times
    pub fn with_access_time(&mut self) -> &mut Self {
        self.metadata.access_time = Buffer::write_atime;
        self
    }

    /// Configures the registry to hide access times
    pub fn without_access_time(&mut self) -> &mut Self {
        self.metadata.access_time = Buffer::write_no_atime;
        self
    }

    /// Configures the registry to display file sizes
    pub fn with_size(&mut self) -> &mut Self {
        self.metadata.size = Buffer::write_size;
        self
    }

    /// Configures the registry to hide file sizes
    pub fn without_size(&mut self) -> &mut Self {
        self.metadata.size = Buffer::write_no_size;
        self
    }
}

// Entry configuration methods
impl<'a> Registry<'a> {
    /// Configures the registry to use default entry display
    pub fn with_default_entry(&mut self) -> &mut Self {
        self.entries = EntryFunctions::default();
        self
    }

    /// Configures the registry to use relative paths for all entry types
    pub fn with_relative_paths(&mut self) -> &mut Self {
        self.entries = EntryFunctions {
            dir: Buffer::write_dir_relative_path,
            file: Buffer::write_entry_relative_path,
            head: Buffer::write_header_relative_path,
            symlink: Buffer::write_symlink_relative_path,
        };
        self
    }
}

// Example usage of the builder pattern
impl<'a> Registry<'a> {
    /// Configures all metadata to be displayed
    pub fn with_all_metadata(&mut self) -> &mut Self {
        self.with_permission()
            .with_birth_time()
            .with_modification_time()
            .with_access_time()
            .with_size()
    }

    /// Configures all metadata to be hidden
    pub fn without_metadata(&mut self) -> &mut Self {
        self.without_permission()
            .without_birth_time()
            .without_modification_time()
            .without_access_time()
            .without_size()
    }
}

// Helper struct method implementations
impl<'a> MetadataFunctions<'a> {
    pub fn show_all(&mut self) {
        self.permission = Buffer::write_permission;
        self.birth_time = Buffer::write_btime;
        self.mod_time = Buffer::write_mtime;
        self.access_time = Buffer::write_atime;
        self.size = Buffer::write_size;
    }

    pub fn hide_all(&mut self) {
        self.permission = Buffer::write_no_permission;
        self.birth_time = Buffer::write_no_btime;
        self.mod_time = Buffer::write_no_mtime;
        self.access_time = Buffer::write_no_atime;
        self.size = Buffer::write_no_size;
    }
}

// Getters for Registry
impl<'a> Registry<'a> {
    /// Returns a reference to the registry's color functions
    pub fn colors(&self) -> &ColorFunctions<'a> {
        &self.colors
    }

    /// Returns a reference to the registry's entry functions
    pub fn entries(&self) -> &EntryFunctions<'a> {
        &self.entries
    }

    /// Returns a reference to the registry's metadata functions
    pub fn metadata(&self) -> &MetadataFunctions<'a> {
        &self.metadata
    }

    /// Returns the current read function
    pub fn read_fn(&self) -> FnReadDir {
        self.read
    }

    /// Returns the current sort function
    pub fn sort_fn(&self) -> FnSortEntries {
        self.sort
    }
}

// Getters for ColorFunctions
impl<'a> ColorFunctions<'a> {
    /// Returns the reset color function
    pub fn reset(&self) -> FnColor<StdoutLock<'a>> {
        self.reset
    }

    /// Returns the yellow color function
    pub fn yellow(&self) -> FnColor<StdoutLock<'a>> {
        self.yellow
    }

    /// Returns the bold red color function
    pub fn bold_red(&self) -> FnColor<StdoutLock<'a>> {
        self.bold_red
    }

    /// Returns the underlined blue color function
    pub fn underlined_blue(&self) -> FnColor<StdoutLock<'a>> {
        self.underlined_blue
    }

    /// Returns the blue color function
    pub fn blue(&self) -> FnColor<StdoutLock<'a>> {
        self.blue
    }

    /// Returns the green color function
    pub fn green(&self) -> FnColor<StdoutLock<'a>> {
        self.green
    }

    /// Returns the purple color function
    pub fn purple(&self) -> FnColor<StdoutLock<'a>> {
        self.purple
    }
}

// Getters for EntryFunctions
impl<'a> EntryFunctions<'a> {
    /// Returns the directory writing function
    pub fn dir(&self) -> FnOutDir<StdoutLock<'a>> {
        self.dir
    }

    /// Returns the file writing function
    pub fn file(&self) -> FnOutFile<StdoutLock<'a>> {
        self.file
    }

    /// Returns the symlink writing function
    pub fn symlink(&self) -> FnOutSymlink<StdoutLock<'a>> {
        self.symlink
    }

    /// Returns the header writing function
    pub fn head(&self) -> FnOutHead<StdoutLock<'a>> {
        self.head
    }
}

// Getters for MetadataFunctions
impl<'a> MetadataFunctions<'a> {
    /// Returns the permission writing function
    pub fn permission(&self) -> FnExtPermission<StdoutLock<'a>> {
        self.permission
    }

    /// Returns the birth time writing function
    pub fn birth_time(&self) -> FnExtBTime<StdoutLock<'a>> {
        self.birth_time
    }

    /// Returns the modification time writing function
    pub fn mod_time(&self) -> FnExtModTime<StdoutLock<'a>> {
        self.mod_time
    }

    /// Returns the access time writing function
    pub fn access_time(&self) -> FnExtAccessTime<StdoutLock<'a>> {
        self.access_time
    }

    /// Returns the size writing function
    pub fn size(&self) -> FnExtSize<StdoutLock<'a>> {
        self.size
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_getters() -> anyhow::Result<()> {
        let registry = Registry::new()?;

        // Using Registry getters
        let _colors = registry.colors();
        let _entries = registry.entries();
        let _metadata = registry.metadata();
        let _read_fn = registry.read_fn();
        let _sort_fn = registry.sort_fn();

        // Using nested getters
        let _permission_fn = registry.metadata().permission();
        let _yellow_fn = registry.colors().yellow();
        let _dir_fn = registry.entries().dir();

        Ok(())
    }
}
