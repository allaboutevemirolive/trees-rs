mod render;

mod cli;
use crate::cli::arg::TreeArgs;

mod config;
use config::root::BaseDirectory;
use report::tail::ReportMode;

mod error;
use crate::error::simple::TResult;

mod report;

mod tree;

mod walk;
use crate::walk::tr::TreeCtxt;

fn main() -> TResult<()> {
    let mut args = TreeArgs::new();
    let mut tr = TreeCtxt::new()?;
    let mut base_dir = BaseDirectory::from_current_dir()?;

    let report_mode = args.match_app(&mut tr, &mut base_dir)?;

    // Update path_builder based on base_dir
    tr.path_builder = base_dir.build().expect("Cannot build base directory.");
    tr.path_builder.append_root();

    run_tree(&mut tr, report_mode)?;

    Ok(())
}

fn run_tree(tr: &mut TreeCtxt, report_mode: ReportMode) -> TResult<()> {
    tr.print_head(
        tr.path_builder.filename(),
        tr.path_builder.base_path(),
        tr.path_builder.metadata()?,
    )?;

    tr.walk_dir(tr.path_builder.base_path())?;

    tr.print_report(report_mode)?;

    Ok(())
}
