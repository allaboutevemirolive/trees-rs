use crate::walk::fent::FileEntry;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a traversal starting point with a base directory and a file name.
#[derive(Debug, Clone)]
pub struct TraversalBase {
    base_path: PathBuf,
    file_name: OsString,
    from_args: bool,
}

impl TraversalBase {
    /// Creates a new `TraversalBase` from the current working directory.
    pub fn new_from_current_dir() -> io::Result<Self> {
        tracing::info!("Initializing TraversalBase from current directory");
        let base_path = env::current_dir()?;
        let file_name = Self::extract_file_name(&base_path);

        Ok(Self {
            base_path,
            file_name,
            from_args: false,
        })
    }

    /// Creates a new `TraversalBase` from a given path.
    pub fn new_from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let base_path = path.as_ref().to_path_buf();
        let file_name = Self::extract_file_name(&base_path);

        Ok(Self {
            base_path,
            file_name,
            from_args: true,
        })
    }

    /// Extracts the file name from a path, defaulting to "." if none exists.
    fn extract_file_name(path: &Path) -> OsString {
        path.file_name()
            .map(OsString::from)
            .unwrap_or_else(|| OsString::from("."))
    }

    /// Returns the file name.
    pub fn file_name(&self) -> OsString {
        self.file_name.clone()
    }

    /// Returns the base path.
    pub fn base_path(&self) -> PathBuf {
        self.base_path.clone()
    }

    /// Returns `true` if the path originated from command-line arguments.
    pub fn is_from_args(&self) -> bool {
        self.from_args
    }

    /// Sets the file name.
    pub fn set_file_name(&mut self, file_name: OsString) {
        self.file_name = file_name;
    }

    /// Sets the base path.
    pub fn set_base_path(&mut self, base_path: PathBuf) {
        self.base_path = base_path;
    }

    /// Sets whether the path originated from command-line arguments.
    pub fn set_from_args(&mut self, from_args: bool) {
        self.from_args = from_args;
    }

    /// Sets the file name to represent the current directory (".").
    pub fn set_file_name_to_current_dir(&mut self) {
        self.file_name = OsString::from(".");
    }

    /// Retrieves metadata for the base directory.
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.base_path)
    }

    /// Creates a `TraversalPathBuilder` from this `TraversalBase`.
    pub fn into_path_builder(self) -> anyhow::Result<TraversalPathBuilder> {
        // Requires the `anyhow` crate
        tracing::info!("Creating TraversalPathBuilder from TraversalBase");
        Ok(TraversalPathBuilder::new(self))
    }
}

/// Builds paths for traversal relative to a `TraversalBase`.
#[derive(Debug, Clone)]
pub struct TraversalPathBuilder {
    path: PathBuf,
    base: TraversalBase,
}

impl TraversalPathBuilder {
    /// Creates a new `TraversalPathBuilder` with the given `TraversalBase`.
    pub fn new(base: TraversalBase) -> Self {
        Self {
            path: PathBuf::with_capacity(5_000),
            base,
        }
    }

    /// Creates a new `TraversalPathBuilder` from the current directory.
    pub fn new_from_current_dir() -> io::Result<Self> {
        Ok(Self::new(TraversalBase::new_from_current_dir()?))
    }

    /// Appends the base directory name to the path.
    pub fn append_base_name(&mut self) -> &mut Self {
        tracing::info!("Appending base directory name to path");
        self.path.push(self.base.file_name());
        self
    }

    /// Extends the path with a relative path derived from a `Visitor`.
    pub fn extend_with_relative_from_visitor(
        &mut self,
        file_entry: &FileEntry,
    ) -> anyhow::Result<&mut Self> {
        let relative_path = file_entry.relative_path(&self.base.base_path()).unwrap(); // Now using ? for error propagation
        self.path.push(relative_path);
        Ok(self)
    }

    /// Returns the file name of the base directory.
    pub fn file_name(&self) -> OsString {
        self.base.file_name()
    }

    /// Returns the base path.
    pub fn base_path(&self) -> PathBuf {
        self.base.base_path()
    }

    /// Retrieves metadata for the base directory.
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.base.metadata()
    }

    /// Converts the built path to an `OsString`.
    pub fn into_os_string(self) -> OsString {
        self.path.into_os_string()
    }

    /// Returns a reference to the underlying `PathBuf`.
    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }
}

impl Default for TraversalPathBuilder {
    fn default() -> Self {
        Self::new_from_current_dir().expect("Failed to create default TraversalPathBuilder")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::TempDir;
//
//     // Mock Visitor for testing (replace with your actual Visitor implementation)
//     struct MockVisitor;

//     impl Visitor for MockVisitor {
//         fn relative_path(&self, _base: &Path) -> anyhow::Result<PathBuf> {
//             Ok(PathBuf::from("mock_relative_path"))
//         }
//     }

//     #[test]
//     fn test_traversal_base_creation() {
//         let temp_dir = TempDir::new().unwrap();
//         let mut base = TraversalBase::new_from_path(temp_dir.path()).unwrap();
//         base.set_from_args(false);

//         assert_eq!(base.base_path(), temp_dir.path());
//         assert!(!base.is_from_args());
//     }

//     #[test]
//     fn test_traversal_path_builder_operations() {
//         let temp_dir = TempDir::new().unwrap();
//         let base = TraversalBase::new_from_path(temp_dir.path()).unwrap();
//         let mut builder = TraversalPathBuilder::new(base);

//         builder.append_base_name();

//         let expected_path = temp_dir.path().join(
//             temp_dir
//                 .path()
//                 .file_name()
//                 .unwrap()
//                 .to_string_lossy()
//                 .as_ref(),
//         );
//         assert_eq!(builder.as_path(), expected_path.as_path());

//         let mock_visitor = MockVisitor;
//         builder
//             .extend_with_relative_from_visitor(&mock_visitor)
//             .unwrap();

//         let expected_extended_path = expected_path.join("mock_relative_path");
//         assert_eq!(builder.as_path(), expected_extended_path.as_path());
//     }
// }
