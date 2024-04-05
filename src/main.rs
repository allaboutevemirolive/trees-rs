mod canva;
mod cli;
mod config;
mod error;
mod report;
mod sort;
mod tree;
mod util;
mod walk;

use std::path::PathBuf;

use crate::{
    canva::Canva,
    cli::{arg::TreeArgs, opt::Setting},
    config::{
        path::{get_absolute_current_shell, Directory},
        registry::CallbackRegistry,
    },
    error::simple::{UResult, USimpleError},
    report::Report,
    tree::{level::Level, Tree},
    walk::{WalkDir, WalkDirConfig, WalkDirOption},
};

fn main() -> UResult<()> {
    println!(".");

    let tree = Tree::new(Level::with_lvl_and_cap(1, 5000), 5000)?;
    let canva = Canva::new()?;
    let report = Report::new()?;

    // let cr = CallbackRegistry::new()?;

    // // config
    let mut config = WalkDirConfig::new(tree, canva, report)?;

    // // root
    // let curr = get_absolute_current_shell().map_err(|err| {
    //     USimpleError::new(1, format!("Failed to get absolute current shell: {}", err))
    // })?;
    // let mut root = PathBuf::new();
    // root.push(curr);

    // -----------------

    let mut setting = Setting::new()?;

    let mut args = TreeArgs::new();

    let path = args.match_app(&mut setting)?;

    // walk_opts
    let walk_opts = WalkDirOption { flag: 1 };
    let mut walk = WalkDir::new(walk_opts, &mut config, &path, setting)?;
    let path = Directory::new(&walk.root)?;

    walk.walk_dir(path)?;
    walk.report()?;

    Ok(())
}
