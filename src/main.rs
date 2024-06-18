mod render;

mod cli;
use crate::cli::arg::TArgs;

mod config;

mod error;
use crate::error::simple::TResult;

mod report;

mod tree;

mod walk;
use walk::GlobalCtxt;
use walk::Printer;
use walk::Walker;

fn main() -> TResult<()> {
    let mut args = TArgs::new();
    let mut gcx = GlobalCtxt::new()?;

    args.match_app(&mut gcx)?;

    run_tree(&mut gcx)?;

    Ok(())
}

fn run_tree<'a>(gcx: &mut GlobalCtxt) -> TResult<()> {
    gcx.print_head(
        gcx.base_dir.filename(),
        gcx.base_dir.base_path(),
        gcx.base_dir.metadata()?,
    )?;

    gcx.walk_dir(gcx.base_dir.base_path())?;

    gcx.print_report()?;

    Ok(())
}
