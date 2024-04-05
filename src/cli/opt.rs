use std::path::PathBuf;

use crate::{
    config::{path::get_absolute_current_shell, registry::CallbackRegistry},
    error::simple::{UResult, USimpleError},
};

#[derive(Debug, Clone)]
pub struct Setting<'a> {
    pub path: PathBuf,
    pub cr: CallbackRegistry<'a>, // sort, read, paint
}

impl<'a> Setting<'a> {
    pub fn new() -> UResult<Self> {
        let path_dir = get_absolute_current_shell().map_err(|err| {
            USimpleError::new(1, format!("Failed to get absolute current shell: {}", err))
        })?;

        let mut path = PathBuf::new();
        path.push(path_dir);

        let cr = CallbackRegistry::new()?;

        Ok(Self { path, cr })
    }

    pub fn with_path(&self, path_dir: String) -> UResult<Self> {
        let mut path = PathBuf::new();
        path.push(path_dir);

        Ok(Self { path, cr: self.cr })
    }

    pub fn with_registry(&self, cr: CallbackRegistry<'a>) -> UResult<Self> {
        Ok(Self {
            path: self.path.clone(),
            cr,
        })
    }
}
