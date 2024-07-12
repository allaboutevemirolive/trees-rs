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

    tracing::info!("Initializing TreeArguments");
    let mut args = cli::arg::TreeArgs::new();

    tracing::info!("Initializing Buffer");
    let mut buf = render::buffer::Buffer::new(std::io::stdout().lock())?;

    let mut tr = walk::tr::TreeCtxt::new(&mut buf).context("Failed to create TreeCtxt")?;

    tracing::info!("Initializing BaseDirectory");
    let mut base_dir = config::root::BaseDirectory::from_current_dir()
        .context("Failed to determine base directory")?;

    tracing::info!("Filter arguments and get report mode");
    let report_mode = args.match_app(&mut tr, &mut base_dir)?;

    tracing::info!("Building PathBuilder");
    tr.path_builder = base_dir
        .build()
        .context("Failed to build base directory path")?;

    tracing::info!("Append root dir to PathBuilder");
    tr.path_builder.append_root();

    tracing::info!("Print directory header");
    tr.print_head()?;

    tracing::info!("Ready to iterate directories");
    tr.walk_dir(tr.path_builder.base_path())?;

    tracing::info!("Print reports");
    tr.print_report(report_mode)?;

    Ok(())
}
