mod render;

mod cli;
use crate::cli::arg::TreeArgs;

mod config;

mod error;
use crate::error::simple::TResult;

mod report;

mod tree;

mod walk;
use walk::TreeCtxt;

fn main() -> TResult<()> {
    let mut args = TreeArgs::new();
    let mut tr = TreeCtxt::new()?;

    args.match_app(&mut tr)?;

    // Update path_builder based on base_dir
    tr.path_builder = tr.base_dir.build().expect("Cannot build base directory.");
    tr.path_builder.append_root();

    run_tree(&mut tr)?;

    Ok(())
}

fn run_tree<'a>(tr: &mut TreeCtxt) -> TResult<()> {
    tr.print_head(
        tr.base_dir.filename(),
        tr.base_dir.base_path(),
        tr.base_dir.metadata()?,
    )?;

    tr.walk_dir(tr.base_dir.base_path())?;

    tr.print_report()?;

    Ok(())
}
