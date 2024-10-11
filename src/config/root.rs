// use crate::walk::fent::FileEntry;

// use std::env;
// use std::ffi::OsString;
// use std::fs;
// use std::io;
// use std::marker::PhantomData;
// use std::path::{Path, PathBuf};

// /// A marker trait to indicate the origin of the path.
// pub trait PathOrigin {}

// /// Marker for paths originating from command-line arguments.
// struct FromArgs;
// impl PathOrigin for FromArgs {}

// /// A trait for types representing paths originating from the current directory.
// trait FromCurrentDirTrait {}

// /// Marker for paths originating from the current directory.
// struct FromCurrentDir;
// impl PathOrigin for FromCurrentDir {}
// impl FromCurrentDirTrait for FromCurrentDir {}

// /// Represents a traversal starting point with a base directory and a file name.
// #[derive(Debug, Clone)]
// pub struct TraversalBase<O: PathOrigin> {
//     base_path: PathBuf,
//     file_name: OsString,
//     _origin: PhantomData<O>,
// }

// impl<O: PathOrigin> TraversalBase<O> {
//     /// Extracts the file name from a path, defaulting to "." if none exists.
//     fn extract_file_name(path: &Path) -> OsString {
//         path.file_name()
//             .map(OsString::from)
//             .unwrap_or_else(|| OsString::from("."))
//     }

//     /// Returns the file name.
//     pub fn file_name(&self) -> OsString {
//         self.file_name.clone()
//     }

//     /// Returns the base path.
//     pub fn base_path(&self) -> PathBuf {
//         self.base_path.clone()
//     }

//     /// Retrieves metadata for the base directory.
//     pub fn metadata(&self) -> io::Result<fs::Metadata> {
//         fs::metadata(&self.base_path)
//     }

//     /// Creates a `TraversalPathBuilder` from this `TraversalBase`.
//     pub fn into_path_builder(self) -> anyhow::Result<TraversalPathBuilder<O>> {
//         tracing::info!("Creating TraversalPathBuilder from TraversalBase");
//         Ok(TraversalPathBuilder::new(self))
//     }

//     /// Creates a new `TraversalBase` from the current working directory.
//     pub fn new_from_current_dir() -> anyhow::Result<Self>
//     where
//         O: FromCurrentDirTrait,
//     {
//         tracing::info!("Initializing TraversalBase from current directory");
//         let base_path = env::current_dir()?;
//         let file_name = Self::extract_file_name(&base_path);

//         Ok(Self {
//             base_path,
//             file_name,
//             _origin: PhantomData,
//         })
//     }
// }

// impl TraversalBase<FromArgs> {
//     /// Creates a new `TraversalBase` from a given path.
//     pub fn new_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
//         let base_path = path.as_ref().to_path_buf();
//         let file_name = Self::extract_file_name(&base_path);

//         Ok(Self {
//             base_path,
//             file_name,
//             _origin: PhantomData,
//         })
//     }
// }

// /// Provides an associated type for the builder state.
// trait BuilderState {
//     type State;
// }

// /// Marker for the initial state of the builder.
// struct Initial;
// impl BuilderState for Initial {
//     type State = InitialState;
// }

// /// Marker for the state after the base name has been appended.
// struct BaseNameAppended;
// impl BuilderState for BaseNameAppended {
//     type State = BaseNameAppendedState;
// }

// /// Builds paths for traversal relative to a `TraversalBase`.
// #[derive(Debug, Clone)]
// pub struct TraversalPathBuilder<O: PathOrigin, S: BuilderState = Initial> {
//     path: PathBuf,
//     base: TraversalBase<O>,
//     _state: PhantomData<S>,
// }

// /// Represents the initial state of the builder.
// #[derive(Debug, Clone)]
// pub struct InitialState;

// /// Represents the state after the base name has been appended.
// #[derive(Debug, Clone)]
// pub struct BaseNameAppendedState;

// impl<O: PathOrigin> TraversalPathBuilder<O, Initial> {
//     /// Creates a new `TraversalPathBuilder` with the given `TraversalBase`.
//     pub fn new(base: TraversalBase<O>) -> Self {
//         Self {
//             path: PathBuf::with_capacity(5_000),
//             base,
//             _state: PhantomData,
//         }
//     }

//     /// Creates a new `TraversalPathBuilder` from the current directory.
//     pub fn new_from_current_dir() -> anyhow::Result<Self>
//     where
//         O: FromCurrentDirTrait,
//     {
//         Ok(Self::new(TraversalBase::<O>::new_from_current_dir()?))
//     }

//     /// Appends the base directory name to the path.
//     pub fn append_base_name(mut self) -> TraversalPathBuilder<O, BaseNameAppended> {
//         tracing::info!("Appending base directory name to path");
//         self.path.push(self.base.file_name());
//         TraversalPathBuilder {
//             path: self.path,
//             base: self.base,
//             _state: PhantomData,
//         }
//     }
// }

// impl<O: PathOrigin> TraversalPathBuilder<O, BaseNameAppended> {
//     /// Extends the path with a relative path derived from a `Visitor`.
//     pub fn extend_with_relative_from_visitor(
//         &mut self,
//         file_entry: &FileEntry,
//     ) -> anyhow::Result<&mut Self> {
//         let relative_path = file_entry.relative_path(&self.base.base_path()).unwrap();
//         self.path.push(relative_path);
//         Ok(self)
//     }
// }

// impl<O: PathOrigin, S: BuilderState> TraversalPathBuilder<O, S> {
//     /// Returns the file name of the base directory.
//     pub fn file_name(&self) -> OsString {
//         self.base.file_name()
//     }

//     /// Returns the base path.
//     pub fn base_path(&self) -> PathBuf {
//         self.base.base_path()
//     }

//     /// Retrieves metadata for the base directory.
//     pub fn metadata(&self) -> io::Result<fs::Metadata> {
//         self.base.metadata()
//     }

//     /// Converts the built path to an `OsString`.
//     pub fn into_os_string(self) -> OsString {
//         self.path.into_os_string()
//     }

//     /// Returns a reference to the underlying `PathBuf`.
//     pub fn as_path(&self) -> &Path {
//         self.path.as_path()
//     }
// }

// impl<O: PathOrigin + FromCurrentDirTrait> Default for TraversalPathBuilder<O, Initial> {
//     fn default() -> Self {
//         Self::new_from_current_dir().expect("Failed to create default TraversalPathBuilder")
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_traversal_base_from_current_dir() {
//         let base = TraversalBase::<FromCurrentDir>::new_from_current_dir().unwrap();
//         assert!(base.base_path().is_absolute());
//         assert_eq!(base.file_name(), OsString::from("."));
//     }

//     #[test]
//     fn test_traversal_base_from_path() {
//         let path = Path::new("/tmp/test");
//         let base = TraversalBase::<FromArgs>::new_from_path(path).unwrap();
//         assert_eq!(base.base_path(), path);
//         assert_eq!(base.file_name(), OsString::from("test"));
//     }

//     #[test]
//     fn test_traversal_path_builder_append_base_name() {
//         let path = Path::new("/tmp/test");
//         let base = TraversalBase::<FromArgs>::new_from_path(path).unwrap();
//         let builder = TraversalPathBuilder::new(base).append_base_name();
//         assert_eq!(builder.as_path(), Path::new("/tmp/test/test"));
//     }

//     // #[test]
//     // fn test_traversal_path_builder_extend_with_relative_from_visitor() {
//     //     let path = Path::new("/tmp/test");
//     //     let base = TraversalBase::<FromArgs>::new_from_path(path).unwrap();
//     //     let mut builder = TraversalPathBuilder::new(base).append_base_name();

//     //     // Assuming your FileEntry::relative_path returns a PathBuf relative to the base path
//     //     let file_entry = FileEntry::new(PathBuf::from("/tmp/test/subdir/file.txt"));
//     //     builder
//     //         .extend_with_relative_from_visitor(&file_entry)
//     //         .unwrap();
//     //     assert_eq!(
//     //         builder.as_path(),
//     //         Path::new("/tmp/test/test/subdir/file.txt")
//     //     );
//     // }

//     #[test]
//     fn test_traversal_path_builder_into_os_string() {
//         let path = Path::new("/tmp/test");
//         let base = TraversalBase::<FromArgs>::new_from_path(path).unwrap();
//         let builder = TraversalPathBuilder::new(base).append_base_name();
//         let os_string = builder.into_os_string();
//         assert_eq!(os_string, OsString::from("/tmp/test/test"));
//     }

//     #[test]
//     fn test_traversal_path_builder_new_from_current_dir() {
//         let builder = TraversalPathBuilder::<FromCurrentDir>::new_from_current_dir().unwrap();
//         assert!(builder.base_path().is_absolute());
//     }
// }
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

// Marker traits for builder state
pub struct Uninitialized;
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
#[derive(Clone)] // Added Clone derivation
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

    pub fn from_current_dir(self) -> io::Result<TraversalPathBuilder<WithBasePath>> {
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
