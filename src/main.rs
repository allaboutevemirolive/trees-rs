mod cli;
mod config;
mod render;
mod report;
mod tree;
mod walk;

fn main() -> anyhow::Result<()> {
    let mut args = cli::arg::TreeArgs::new();
    let mut buf = render::buffer::Buffer::new(std::io::stdout().lock())?;
    let mut tr = walk::tr::TreeCtxt::new(&mut buf)?;
    let mut base_dir = config::root::BaseDirectory::from_current_dir()?;

    let report_mode = args.match_app(&mut tr, &mut base_dir)?;

    tr.path_builder = base_dir.build().expect("Cannot build base directory.");
    tr.path_builder.append_root();

    tr.print_head()?;
    tr.walk_dir(tr.path_builder.base_path())?;
    tr.print_report(report_mode)?;

    Ok(())
}
