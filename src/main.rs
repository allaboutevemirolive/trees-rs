mod canva;
mod cli;
mod config;
mod error;
mod report;
mod sort;
mod tree;
mod util;
mod walk;
// use crate::canva::which;
use crate::canva::Canva;
use crate::cli::arg::TreeArgs;
use crate::cli::opt::Setting;
use crate::config::path::Directory;
use crate::error::simple::UResult;
use crate::report::Report;
use crate::tree::level::Level;
use crate::tree::Tree;
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

    let (path, path_filename) = args.match_app(&mut setting)?;

    let metadata_header = fs::metadata(path.clone())?;
    let path_header = path.clone();
    let filename_header = path_filename.clone();

    let mut walk = WalkDir::new(&mut config, &path, &path_filename, setting)?;
    let path = Directory::new(&walk.root)?;

    walk.config
        .canva
        .buffer
        .paint_permission(&metadata_header, walk.setting.cr.wha)?;

    walk.config
        .canva
        .buffer
        .paint_date(&metadata_header, walk.setting.cr.whd)?;

    walk.config.canva.buffer.paint_header(
        &metadata_header,
        &path_header,     // abs_path
        &filename_header, // ../../tree/src
        walk.setting.cr.wh,
    )?;
    walk.config.canva.buffer.write_newline()?;

    walk.walk_dir(path)?;
    walk.report()?;

    Ok(())
}
