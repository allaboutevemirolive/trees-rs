mod canva;
mod cli;
mod config;
mod error;
mod report;
mod sort;
mod tree;
mod util;
mod walk;

use std::{fs, os::unix::fs::MetadataExt};

use canva::buffer::Buffer;

use crate::{
    canva::Canva,
    cli::{arg::TreeArgs, opt::Setting},
    config::path::Directory,
    error::simple::UResult,
    report::Report,
    tree::{level::Level, Tree},
    walk::{WalkDir, WalkDirConfig, WalkDirOption},
};

fn main() -> UResult<()> {
    let tree = Tree::new(Level::with_lvl_and_cap(1, 5000), 5000)?;
    let canva = Canva::new()?;
    let report = Report::new()?;

    // config
    let mut config = WalkDirConfig::new(tree, canva, report)?;
    let mut setting = Setting::new()?;

    let mut args = TreeArgs::new();

    let (path, path_filename) = args.match_app(&mut setting)?;

    let metadata_header = fs::metadata(path.clone())?;
    let path_header = path.clone();
    let filename_header = path_filename.clone();

    // walk_opts
    let walk_opts = WalkDirOption { flag: 1 };
    let mut walk = WalkDir::new(walk_opts, &mut config, &path, &path_filename, setting)?;
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
