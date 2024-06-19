use crate::error::simple::TResult;
use crate::walk::visit::Visitor;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Struct that store the path where we needs to start traverse
#[derive(Debug, Clone)]
pub struct BaseDirectory {
    /// This is not always absolute path
    base_path: PathBuf,
    /// If path is not provide in the argument, change this to dot ".", if exist, change this to the path
    file_name: OsString,
    /// Keep track which path we get from, either from cmd or cwd.
    is_path_from_args: bool,
}

impl BaseDirectory {
    pub fn from_current_dir() -> io::Result<Self> {
        let base_path = env::current_dir()?;

        Ok(Self {
            file_name: base_path
                .file_name()
                .map(OsString::from)
                .unwrap_or_else(|| OsString::from(".")),
            base_path,
            is_path_from_args: false,
        })
    }

    pub fn filename(&self) -> OsString {
        self.file_name.clone()
    }

    pub fn with_filename(&mut self, file_name: OsString) {
        self.file_name = file_name;
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_path.clone()
    }

    pub fn with_base_path(&mut self, base_path: PathBuf) {
        self.base_path = base_path
    }

    /// Marks this `BaseDirectory` instance as having a path explicitly
    /// provided through command-line arguments.
    pub fn set_path_from_args(&mut self) {
        self.is_path_from_args = true;
    }

    /// Marks this `BaseDirectory` instance as having a path derived from the
    /// current working directory (not explicitly provided as an argument).
    pub fn set_path_from_cwd(&mut self) {
        self.is_path_from_args = false;
    }

    /// Returns `true` if the path was provided as a command-line argument,
    /// and `false` if it was derived from the current working directory.
    pub fn _is_path_from_args(&self) -> bool {
        self.is_path_from_args
    }

    /// Sets the file name to "." (dot), indicating the current directory.
    pub fn set_file_name_to_current_dir(&mut self) {
        self.file_name = ".".into()
    }

    /// Retrieves metadata for the base directory.
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.base_path)
    }

    pub fn build(&self) -> TResult<PathBuilder> {
        Ok(PathBuilder {
            builder: PathBuf::with_capacity(5_000),
            base_dir: BaseDirectory {
                file_name: self.filename(),
                base_path: self.base_path(),
                is_path_from_args: self.is_path_from_args,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct PathBuilder {
    builder: PathBuf,
    base_dir: BaseDirectory,
}

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder {
            builder: PathBuf::with_capacity(5_000),
            base_dir: BaseDirectory::from_current_dir().expect("Cannot initialize BaseDirectory"),
        }
    }

    /// Appends the root directory from the `BaseDirectory` struct to the path builder.
    ///
    /// This function extracts the root directory name (the filename component of `self.base_dir`)
    /// and appends it to the current path being constructed in `self.builder`.
    ///
    /// # Why filename, not base_path?
    ///
    /// We specifically use the `filename()` method of `BaseDirectory` instead of its `base_path()`
    /// for a critical reason:
    ///
    /// * `filename()` gives us only the directory name itself, which is essential for correctly
    ///   constructing relative paths.
    /// * `base_path()` might contain an absolute path, which would interfere with our relative path
    ///   calculations.
    ///
    /// By using the filename, we ensure that we are only appending the necessary directory component
    /// to the path builder, maintaining the relative nature of the resulting path.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming the base_dir is set to a BaseDirectory with filename "project_root"
    /// path_builder.append_root();
    /// // path_builder now contains "project_root"
    /// ```
    pub fn append_root(&mut self) {
        self.builder.push(self.base_dir.filename());
    }

    /// Appends a relative path to the current path builder.
    ///
    /// This function constructs a new path by joining the existing path in `self.builder`
    /// with the relative path provided by the `Visitor`.
    ///
    /// # Arguments
    ///
    /// * `visit`: A reference to a `Visitor`, which provides the relative path to append.
    ///
    /// # Returns
    ///
    /// A new instance of `Self` with the updated path builder.
    /// The original `Self` instance is not modified.
    ///
    /// # Errors
    ///
    /// This function may return an error if there's an issue obtaining the relative path
    /// from the `Visitor`.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming the current builder path is 'src' and `visit.get_relative_path()` returns 'dir/some.rs'
    /// let new_builder = existing_builder.append_relative(&visit);  
    /// // new_builder.builder will now contain 'src/dir/some.rs'
    /// ```
    pub fn append_relative(&mut self, visit: &Visitor) -> Self {
        // Retrieve the relative path from the Visitor
        let relative_path = visit
            .get_relative_path(&self.base_dir.base_path())
            .expect("Failed to get relative path from Visitor");

        // Update the path builder with the relative path
        self.builder.push(relative_path);

        // Return a new instance with the updated path
        Self {
            builder: self.builder.clone(),
            base_dir: self.base_dir.clone(),
        }
    }

    pub fn into_os_string(&self) -> OsString {
        self.builder.clone().into_os_string()
    }

    pub fn filename(&self) -> OsString {
        self.base_dir.filename()
    }

    pub fn base_path(&self) -> PathBuf {
        self.base_dir.base_path()
    }

    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.base_dir.metadata()
    }
}
