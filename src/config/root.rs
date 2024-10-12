use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

// Marker traits for builder state
#[derive(Debug)]
pub struct Uninitialized;
#[derive(Debug, Clone)]
pub struct WithBasePath;

// Main trait for traversal functionality
pub trait Traversal {
    fn metadata(&self) -> io::Result<fs::Metadata>;
    fn file_name(&self) -> OsString;
    fn base_path(&self) -> PathBuf;
}

// Builder trait with associated type
pub trait TraversalBuilder {
    type Traversal: Traversal;

    fn build(self) -> io::Result<Self::Traversal>;
}

// Extension trait for path manipulation
pub trait PathManipulation {
    fn append_base_name(self) -> Self;
    fn extend_with_relative_path<P: AsRef<Path>>(self, relative: P) -> Self;
    fn base_path(&mut self) -> Option<PathBuf>;
    fn set_from_args(&mut self, from_args: bool);
    fn from_args(&self) -> bool;
    fn set_file_name_to_current_dir(&mut self);
    fn set_base_path(&mut self, base_path: Option<PathBuf>);
    fn set_file_name(&mut self, file_name: Option<OsString>);
    fn current_path(&self) -> &PathBuf;
}

// Main struct that implements Traversal
pub struct TraversalPath {
    base_path: PathBuf,
    file_name: OsString,
    current_path: PathBuf,
    from_args: bool,
}

impl Traversal for TraversalPath {
    fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.base_path)
    }

    fn file_name(&self) -> OsString {
        self.file_name.clone()
    }

    fn base_path(&self) -> PathBuf {
        self.base_path.clone()
    }
}

// Builder struct with type state
#[derive(Debug, Clone)] // Added Clone derivation
pub struct TraversalPathBuilder<State = Uninitialized> {
    base_path: Option<PathBuf>,
    file_name: Option<OsString>,
    current_path: PathBuf,
    from_args: bool,
    _state: PhantomData<State>,
}

// Implementation for uninitialized state
impl TraversalPathBuilder<Uninitialized> {
    pub fn new() -> Self {
        TraversalPathBuilder {
            base_path: None,
            file_name: None,
            current_path: PathBuf::with_capacity(5_000),
            from_args: false,
            _state: PhantomData,
        }
    }

    pub fn from_current_dir(self) -> anyhow::Result<TraversalPathBuilder<WithBasePath>> {
        let base_path = env::current_dir()?;
        let file_name = Self::extract_file_name(&base_path);

        Ok(TraversalPathBuilder {
            base_path: Some(base_path),
            file_name: Some(file_name),
            current_path: self.current_path,
            from_args: false,
            _state: PhantomData,
        })
    }

    pub fn from_path<P: AsRef<Path>>(
        self,
        path: P,
    ) -> io::Result<TraversalPathBuilder<WithBasePath>> {
        let base_path = path.as_ref().to_path_buf();
        let file_name = Self::extract_file_name(&base_path);

        Ok(TraversalPathBuilder {
            base_path: Some(base_path),
            file_name: Some(file_name),
            current_path: self.current_path,
            from_args: true,
            _state: PhantomData,
        })
    }
}

// Common implementations for all states
impl<State> TraversalPathBuilder<State> {
    fn extract_file_name(path: &Path) -> OsString {
        path.file_name()
            .map(OsString::from)
            .unwrap_or_else(|| OsString::from("."))
    }
}

// Implement PathManipulation for WithBasePath state
impl PathManipulation for TraversalPathBuilder<WithBasePath> {
    fn append_base_name(mut self) -> Self {
        if let Some(ref file_name) = self.file_name {
            self.current_path.push(file_name);
        }
        self
    }

    fn extend_with_relative_path<P: AsRef<Path>>(mut self, relative: P) -> Self {
        self.current_path.push(relative);
        self
    }

    fn base_path(&mut self) -> Option<PathBuf> {
        self.base_path.clone()
    }

    fn set_from_args(&mut self, from_args: bool) {
        self.from_args = from_args;
    }

    fn from_args(&self) -> bool {
        self.from_args
    }

    /// Sets the file name to represent the current directory (".").
    fn set_file_name_to_current_dir(&mut self) {
        self.file_name = Some(OsString::from("."));
    }

    fn set_base_path(&mut self, base_path: Option<PathBuf>) {
        self.base_path = base_path;
    }

    fn set_file_name(&mut self, file_name: Option<OsString>) {
        self.file_name = file_name;
    }

    fn current_path(&self) -> &PathBuf {
        &self.current_path
    }
}

// Implement TraversalBuilder for WithBasePath state
impl TraversalBuilder for TraversalPathBuilder<WithBasePath> {
    type Traversal = TraversalPath;

    fn build(self) -> io::Result<Self::Traversal> {
        let base_path = self
            .base_path
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Base path not set"))?;
        let file_name = self
            .file_name
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File name not set"))?;

        Ok(TraversalPath {
            base_path,
            file_name,
            current_path: self.current_path,
            from_args: self.from_args,
        })
    }
}

// Example usage and helper functions
impl TraversalPath {
    pub fn builder() -> TraversalPathBuilder<Uninitialized> {
        TraversalPathBuilder::new()
    }

    pub fn as_path(&self) -> &Path {
        self.current_path.as_path()
    }

    pub fn into_os_string(self) -> OsString {
        self.current_path.into_os_string()
    }
}
