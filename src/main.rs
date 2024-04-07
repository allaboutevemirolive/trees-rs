mod canva;
mod cli;
mod config;
mod error;
mod report;
mod sort;
mod tree;
mod util;
mod walk;

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
    println!(".");

    let tree = Tree::new(Level::with_lvl_and_cap(1, 5000), 5000)?;
    let canva = Canva::new()?;
    let report = Report::new()?;

    // config
    let mut config = WalkDirConfig::new(tree, canva, report)?;
    let mut setting = Setting::new()?;

    let mut args = TreeArgs::new();

    let (path, path_filename) = args.match_app(&mut setting)?;

    // walk_opts
    let walk_opts = WalkDirOption { flag: 1 };
    let mut walk = WalkDir::new(walk_opts, &mut config, &path, &path_filename, setting)?;
    let path = Directory::new(&walk.root)?;

    walk.walk_dir(path)?;
    walk.report()?;

    Ok(())
}
