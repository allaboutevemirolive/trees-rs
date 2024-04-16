mod canva;
use crate::canva::Canva;

mod cli;
use crate::cli::arg::TArgs;
use crate::cli::opt::Setting;

mod config;
use crate::config::path::Directory;

mod error;
use crate::error::simple::TResult;

mod report;
use crate::report::Report;

mod sort;

mod tree;
use crate::tree::level::Level;
use crate::tree::Tree;

mod util;

mod walk;
use crate::walk::Config;
use crate::walk::WalkDir;

use std::fs;

fn main() -> TResult<()> {
    let tree = Tree::new(Level::with_lvl_and_cap(1, 10_000), 5000)?;
    let canva = Canva::new()?;
    let report = Report::new()?;

    // Setup config
    let mut config = Config::new(tree, canva, report)?;
    let mut setting = Setting::new()?;

    // Collect arguments
    let mut args = TArgs::new();

    // Match arguments and get target path
    let (path, root_filename) = TArgs::match_app(&mut args, &mut setting, &mut config)?;

    // Collect header metadata
    let hmeta = fs::metadata(path.clone())?;
    let hpath = path.clone();
    let hfilename = root_filename.clone();

    // Initialize walk configuration
    let mut walk = WalkDir::new(&mut config, &path, &root_filename, setting)?;
    // Setup entry point of traversal
    let path = Directory::new(walk.root)?;

    // Print header permission
    canva::buffer::Buffer::paint_permission(
        &mut walk.config.canva.buffer,
        &hmeta,
        walk.setting.cr.pms,
    )?;

    // Print header creation-date
    canva::buffer::Buffer::paint_btime(
        &mut walk.config.canva.buffer,
        &hmeta,
        walk.setting.cr.btime,
    )?;

    // Print entry's modification-time
    canva::buffer::Buffer::paint_mtime(
        &mut walk.config.canva.buffer,
        &hmeta,
        walk.setting.cr.mtime,
    )?;

    // Print entry's access-time
    canva::buffer::Buffer::paint_atime(
        &mut walk.config.canva.buffer,
        &hmeta,
        walk.setting.cr.atime,
    )?;

    // Print header size
    canva::buffer::Buffer::paint_size(&mut walk.config.canva.buffer, &hmeta, walk.setting.cr.size)?;

    // Print header's name
    canva::buffer::Buffer::paint_header(
        &mut walk.config.canva.buffer,
        &hmeta,
        &hpath,
        &hfilename,
        walk.setting.cr.head,
    )?;
    canva::buffer::Buffer::write_newline(&mut walk.config.canva.buffer)?;

    // Traversing
    WalkDir::walk_dir(&mut walk, path)?;

    // Print summarize
    WalkDir::report(&mut walk)?;

    Ok(())
}
