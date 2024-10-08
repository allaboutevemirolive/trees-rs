use crate::walk::visit::Visitor;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a base directory for traversal operations
#[derive(Debug, Clone)]
pub struct BaseDirectory {
    base_path: PathBuf,
    file_name: OsString,
    is_path_from_args: bool,
}

impl BaseDirectory {
    /// Creates a new BaseDirectory from the current working directory
    pub fn from_current_dir() -> io::Result<Self> {
        tracing::info!("Initializing BaseDirectory from current directory");
        let base_path = env::current_dir()?;
        let file_name = Self::extract_file_name(&base_path);

        Ok(Self {
            base_path,
            file_name,
            is_path_from_args: false,
        })
    }

    /// Creates a new BaseDirectory from a given path
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let base_path = path.as_ref().to_path_buf();
        let file_name = Self::extract_file_name(&base_path);

        Ok(Self {
            base_path,
            file_name,
            is_path_from_args: true,
        })
    }

    /// Extracts the file name from a path, defaulting to "." if none exists
    fn extract_file_name(path: &Path) -> OsString {
        path.file_name()
            .map(OsString::from)
            .unwrap_or_else(|| OsString::from("."))
    }

    // Getters
    pub fn filename(&self) -> OsString {
        self.file_name.clone()
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_path.clone()
    }

    pub fn is_path_from_args(&self) -> bool {
        self.is_path_from_args
    }

    // Setters
    pub fn set_filename(&mut self, file_name: OsString) {
        self.file_name = file_name;
    }

    pub fn set_base_path(&mut self, base_path: PathBuf) {
        self.base_path = base_path;
    }

    pub fn set_path_source(&mut self, from_args: bool) {
        self.is_path_from_args = from_args;
    }

    /// Sets the file name to the current directory (".")
    pub fn set_to_current_dir(&mut self) {
        self.file_name = OsString::from(".");
    }

    /// Retrieves metadata for the base directory
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.base_path)
    }

    /// Creates a PathBuilder from this BaseDirectory
    pub fn into_builder(self) -> anyhow::Result<PathBuilder> {
        tracing::info!("Creating PathBuilder from BaseDirectory");
        Ok(PathBuilder::new(self))
    }
}

#[derive(Debug, Clone)]
pub struct PathBuilder {
    path: PathBuf,
    base_dir: BaseDirectory,
}

impl PathBuilder {
    /// Creates a new PathBuilder with the given BaseDirectory
    pub fn new(base_dir: BaseDirectory) -> Self {
        Self {
            path: PathBuf::with_capacity(5_000),
            base_dir,
        }
    }

    /// Creates a new PathBuilder from the current directory
    pub fn from_current_dir() -> io::Result<Self> {
        Ok(Self::new(BaseDirectory::from_current_dir()?))
    }

    /// Appends the root directory to the path
    pub fn append_root(&mut self) -> &mut Self {
        tracing::info!("Appending root directory to path");
        self.path.push(self.base_dir.filename());
        self
    }

    /// Appends a relative path from a Visitor
    pub fn append_relative(&mut self, visitor: &Visitor) -> anyhow::Result<&mut Self> {
        let relative_path = visitor.relative_path(&self.base_dir.base_path()).unwrap();
        self.path.push(relative_path);
        Ok(self)
    }

    // Getters that delegate to base_dir
    pub fn filename(&self) -> OsString {
        self.base_dir.filename()
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_dir.base_path()
    }

    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.base_dir.metadata()
    }

    /// Converts the built path to an OsString
    pub fn into_os_string(self) -> OsString {
        self.path.into_os_string()
    }

    /// Returns a reference to the underlying PathBuf
    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::from_current_dir().expect("Failed to create default PathBuilder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_base_directory_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut base_dir = BaseDirectory::from_path(temp_dir.path()).unwrap();
        // NOTE: set base_dir to false because we didnt provide any path on the argument.
        base_dir.set_path_source(false);

        assert_eq!(base_dir.base_path(), temp_dir.path());
        assert!(!base_dir.is_path_from_args());
    }

    #[test]
    fn test_path_builder_operations() {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = BaseDirectory::from_path(temp_dir.path()).unwrap();
        let mut builder = PathBuilder::new(base_dir);

        builder.append_root();
        assert!(builder.as_path().to_string_lossy().contains(
            temp_dir
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .as_ref()
        ));
    }
}
