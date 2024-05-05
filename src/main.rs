mod canva;

use walk::GlobalCtxt;

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

use std::os::unix::fs::MetadataExt;

fn main() -> TResult<()> {
    let mut args = TArgs::new();
    let mut gcx = GlobalCtxt::new()?;

    let (fpath, fname, fmeta) = args.xmatch_app(&mut gcx)?;

    gcx.tail.add_size(fmeta.size());
    gcx.print_meta(&fmeta)?;
    gcx.buf
        .paint_header(&fmeta, &fpath.clone(), &fname, gcx.rg.head)?;
    gcx.buf.write_newline()?;

    gcx.walk_dir(fpath)?;

    gcx.buf.write_newline()?;
    gcx.buf.write_message(&gcx.tail.to_string())?;
    gcx.buf.write_newline()?;

    Ok(())
}
