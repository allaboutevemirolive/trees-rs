mod canva;
mod cli;
mod config;
mod report;
mod sort;
mod tree;
mod walk;

use std::path::PathBuf;

use crate::{
    canva::Canva,
    config::{
        path::{get_absolute_current_shell, Directory},
        registry::CallbackRegistry,
    },
    report::Report,
    tree::{level::Level, Tree},
    walk::{WalkDirConfig, WalkDirOption},
};

fn main() {
    println!(".");

    let tree = Tree::new(Level::with_lvl_and_cap(1, 5000), 5000);
    let canva = Canva::new();
    let report = Report::new();

    let cr = CallbackRegistry::new();

    // config
    let mut config = WalkDirConfig::new(tree, canva, report);
    // root
    let curr = get_absolute_current_shell();
    let mut root = PathBuf::new();
    root.push(curr);
    // walk_opts
    let walk_opts = WalkDirOption { flag: 1 };

    let mut walk = crate::walk::WalkDir::new(walk_opts, &mut config, &root, cr);
    let path = Directory::new(&walk.root);
    walk.walk_dir(path);
    walk.report();
}
