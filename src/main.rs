mod canva;
use crate::canva::Canva;

mod cli;
use crate::cli::arg::TreeArgs;
use crate::cli::opt::Setting;

mod config;
use crate::config::path::Directory;

mod error;
use crate::error::simple::UResult;

mod report;
use crate::report::Report;

mod sort;

mod tree;
use crate::tree::level::Level;
use crate::tree::Tree;

mod util;

mod walk;
use crate::walk::WalkDir;
use crate::walk::WalkDirConfig;

use std::fs;

fn main() -> UResult<()> {
    let tree = Tree::new(Level::with_lvl_and_cap(1, 5000), 5000)?;
    let canva = Canva::new()?;
    let report = Report::new()?;

    // Setup config
    let mut config = WalkDirConfig::new(tree, canva, report)?;
    let mut setting = Setting::new()?;

    // Collect arguments
    let mut args = TreeArgs::new();

    // Match arguments and get target path
    let (path, path_filename) = TreeArgs::match_app(&mut args, &mut setting)?;

    // Collect header metadata
    let metadata_header: fs::Metadata = fs::metadata(path.clone())?;
    let path_header = path.clone();
    let filename_header = path_filename.clone();

    // Initialize walk configuration
    let mut walk = WalkDir::new(&mut config, &path, &path_filename, setting)?;
    // Setup entry point of traversal
    let path = Directory::new(walk.root)?;

    // Print header permission
    canva::buffer::Buffer::paint_permission(
        &mut walk.config.canva.buffer,
        &metadata_header,
        walk.setting.cr.pms,
    )?;

    // Print header creation-date
    canva::buffer::Buffer::paint_btime(
        &mut walk.config.canva.buffer,
        &metadata_header,
        walk.setting.cr.btime,
    )?;

    canva::buffer::Buffer::paint_mtime(
        &mut walk.config.canva.buffer,
        &metadata_header,
        walk.setting.cr.mtime,
    )?;

    canva::buffer::Buffer::paint_atime(
        &mut walk.config.canva.buffer,
        &metadata_header,
        walk.setting.cr.atime,
    )?;

    // Print header size
    canva::buffer::Buffer::paint_size(
        &mut walk.config.canva.buffer,
        &metadata_header,
        walk.setting.cr.size,
    )?;

    // Print header's name
    canva::buffer::Buffer::paint_header(
        &mut walk.config.canva.buffer,
        &metadata_header,
        &path_header,
        &filename_header,
        walk.setting.cr.head,
    )?;
    canva::buffer::Buffer::write_newline(&mut walk.config.canva.buffer)?;

    // Traversing
    WalkDir::walk_dir(&mut walk, path)?;

    // Print summarize
    WalkDir::report(&mut walk)?;

    Ok(())
}
