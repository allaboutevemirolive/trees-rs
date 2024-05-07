mod canva;

mod cli;
use crate::cli::arg::TArgs;

mod config;

mod error;
use crate::error::simple::TResult;

mod report;

mod sort;

mod tree;

mod util;

mod walk;
use walk::{GlobalCtxt, Printer, Walker};

use std::ffi::OsString;
use std::fs::Metadata;
use std::path::PathBuf;

fn main() -> TResult<()> {
    let mut args = TArgs::new();
    let mut gcx = GlobalCtxt::new()?;

    // Yield starting path which we needs to traverse
    let (fpath, fname, fmeta) = args.match_app(&mut gcx)?;

    run_tree(&mut gcx, fname, fpath, fmeta)?;

    Ok(())
}

fn run_tree<'a, T>(gcx: &mut T, fname: OsString, fpath: PathBuf, fmeta: Metadata) -> TResult<()>
where
    T: Walker<'a> + Printer<'a>,
{
    gcx.print_head(fname, fpath.clone(), fmeta)?;

    gcx.walk_dir(fpath)?;

    gcx.print_report()?;

    Ok(())
}
