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
use walk::GlobalCtxt;

use std::ffi::OsString;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

fn main() -> TResult<()> {
    let mut args = TArgs::new();
    let mut gcx = GlobalCtxt::new()?;

    // Yield out starting point path which we needs to traverse
    let (fpath, fname, fmeta) = args.match_app(&mut gcx)?;

    run_tree(&mut gcx, fname, fpath, fmeta)?;

    Ok(())
}

fn run_tree(gcx: &mut GlobalCtxt, fname: OsString, fpath: PathBuf, fmeta: Metadata) -> TResult<()> {
    print_head(gcx, fname, fpath.clone(), fmeta)?;

    gcx.walk_dir(fpath)?;

    print_report(gcx)?;

    Ok(())
}

// head
// ├── dir
// │   ├── entry
fn print_head(
    gcx: &mut GlobalCtxt,
    fname: OsString,
    fpath: PathBuf,
    fmeta: Metadata,
) -> TResult<()> {
    gcx.tail.add_size(fmeta.size());
    gcx.print_meta(&fmeta)?;
    gcx.buf
        .paint_header(&fmeta, &fpath.clone(), &fname, gcx.rg.head)?;
    gcx.buf.write_newline()?;

    Ok(())
}

// └── dir
//     ├── entry1
//     └── entry2
//
// 13 directories, 36 files, 0 hidden, 0.00 gigabytes
fn print_report(gcx: &mut GlobalCtxt) -> TResult<()> {
    gcx.buf.write_newline()?;
    gcx.buf.write_message(&gcx.tail.to_string())?;
    gcx.buf.write_newline()?;

    Ok(())
}
