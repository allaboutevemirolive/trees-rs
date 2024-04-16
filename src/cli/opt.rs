use std::path::PathBuf;

use crate::{
    config::{path::get_absolute_current_shell, registry::Registry},
    error::simple::{TResult, TSimpleError},
};

#[derive(Debug, Clone)]
pub struct Setting<'a> {
    pub path: PathBuf,
    pub cr: Registry<'a>, // sort, read, paint
}

impl<'a> Setting<'a> {
    pub fn new() -> TResult<Self> {
        let path_dir = get_absolute_current_shell().map_err(|err| {
            TSimpleError::new(1, format!("Failed to get absolute current shell: {}", err))
        })?;

        let mut path = PathBuf::new();
        path.push(path_dir);

        let cr = Registry::new()?;

        Ok(Self { path, cr })
    }
}
