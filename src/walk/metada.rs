use crate::canva::buffer;
use crate::error::simple::TResult;
use crate::tree::level;

use std::ffi::OsString;
use std::fs::DirEntry;
use std::fs::FileType;
use std::fs::Metadata;
use std::path::PathBuf;

use super::WalkDir;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Visitor {
    pub abs: PathBuf,
    pub dent: DirEntry,
    pub meta: Metadata,
    pub name: OsString,
    pub filety: FileType,
    pub lvl: level::Level,
    pub size: u64,
}

impl<'mt, 'wd, 'cv: 'cr, 'cr: 'cv> Visitor {
    pub fn new(dent: DirEntry, level: &level::Level) -> TResult<Self> {
        let filety = dent.file_type()?;
        let abs = dent.path();
        let name = dent
            .path()
            .file_name()
            .map(|os_str| os_str.to_os_string())
            .expect("Failed to get file name");

        let meta = dent.metadata()?;

        let size = meta.len();

        Ok(Self {
            abs,
            dent,
            meta,
            filety,
            name,
            lvl: *level,
            size,
        })
    }

    pub fn get_relative_path(&self, current_dir: &PathBuf) -> Option<PathBuf> {
        let path = self.dent.path();
        if let Ok(relative_path) = path.strip_prefix(current_dir) {
            Some(relative_path.to_path_buf())
        } else {
            None
        }
    }

    #[rustfmt::skip]
    pub fn print_meta(meta: &Metadata, walk: &'mt mut WalkDir<'wd, 'cv, 'cr>) -> TResult<()> {
        // Print entry's permission
        buffer::Buffer::paint_permission(
            &mut walk.config.canva.buffer,
            &meta,
            walk.setting.cr.pms,
        )?;

        // Print entry's creation-date
        buffer::Buffer::paint_btime(
            &mut walk.config.canva.buffer, 
            &meta, 
            walk.setting.cr.btime
        )?;

        // Print entry's modification-time
        buffer::Buffer::paint_mtime(
            &mut walk.config.canva.buffer, 
            &meta, 
            walk.setting.cr.mtime
        )?;

        // Print entry's access-time
        buffer::Buffer::paint_atime(
            &mut walk.config.canva.buffer, 
            &meta, 
            walk.setting.cr.atime
        )?;

        // Print entry's size
        buffer::Buffer::paint_size(
            &mut walk.config.canva.buffer, 
            &meta, 
            walk.setting.cr.size
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod metada_test {

    #[allow(unused_imports)]
    use super::*;
    use std::fs::{self, File};
    use std::io;
    #[allow(unused_imports)]
    use std::io::Write;
    #[allow(unused_imports)]
    use std::os::unix::fs::PermissionsExt;
    #[allow(unused_imports)]
    use tempfile::tempdir;

    #[allow(dead_code)]
    fn create_temp_file_with_permissions(
        temp_dir: &tempfile::TempDir,
        permissions: u32,
    ) -> io::Result<DirEntry> {
        let file_path = temp_dir.path().join("test_file.txt");
        let file = File::create(&file_path)?;
        file.set_permissions(fs::Permissions::from_mode(permissions))?;
        let dir_entry = fs::read_dir(temp_dir.path())?.next().unwrap()?;
        Ok(dir_entry)
    }
}
