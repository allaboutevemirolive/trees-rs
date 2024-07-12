mod cli;
mod config;
mod render;
mod report;
mod tree;
mod walk;

fn main() -> anyhow::Result<()> {
    use anyhow::Context;

    if std::env::var("ENABLE_TRACING").is_ok() {
        tracing_subscriber::fmt::init();
        tracing::info!("Hello from Trees-rs");
    }

    let mut args = cli::arg::TreeArgs::new();
    let mut buf = render::buffer::Buffer::new(std::io::stdout().lock())?;
    let mut tr = walk::tr::TreeCtxt::new(&mut buf).context("Failed to create TreeCtxt")?;

    let mut base_dir = config::root::BaseDirectory::from_current_dir()
        .context("Failed to determine base directory")?;

    let report_mode = args.match_app(&mut tr, &mut base_dir)?;

    tr.path_builder = base_dir
        .build()
        .context("Failed to build base directory path")?;

    tr.path_builder.append_root();
    tr.print_head()?;

    tracing::info!("Ready to iterate directories");
    tr.walk_dir(tr.path_builder.base_path())?;

    tr.print_report(report_mode)?;

    Ok(())
}
