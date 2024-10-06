mod cli;
mod config;
mod render;
mod report;
mod tree;
mod walk;

use report::stats::ReportMode;

macro_rules! trace {
    ($x:expr) => {
        if std::env::var("ENABLE_TRACING").is_ok() {
            dbg!($x);
        }
    };
}

fn main() -> anyhow::Result<()> {
    initialize_tracing();
    let mut args = initialize_args()?;
    let mut buf = render::buffer::Buffer::new(std::io::stdout().lock())?;
    let mut tr = initialize_tree_context(&mut buf)?;

    trace!(&tr);

    let mut base_dir = determine_base_directory()?;
    let report_mode = args.match_app(&mut tr, &mut base_dir)?;

    build_and_print_tree_head(&mut tr, &mut base_dir)?;

    iterate_directories_and_print_report(&mut tr, report_mode)?;

    Ok(())
}

/// Initializes tracing if the `ENABLE_TRACING` environment variable is set.
fn initialize_tracing() {
    if std::env::var("ENABLE_TRACING").is_ok() {
        tracing_subscriber::fmt::init();
        tracing::info!("{}", dbg!("Hello from Trees-rs"));
    }
}

/// Initializes the command-line arguments and returns a `TreeArgs` object.
fn initialize_args() -> anyhow::Result<cli::arg::TreeArgs> {
    Ok(cli::arg::TreeArgs::new())
}

/// Initializes the tree context for rendering and returns a `TreeCtxt`.
/// `'tr` is for the stdout lock lifetime, and `'a` is for the buffer reference.
fn initialize_tree_context<'tr, 'a>(
    buf: &'a mut render::buffer::Buffer<std::io::StdoutLock<'tr>>,
) -> anyhow::Result<walk::tr::TreeCtxt<'tr, 'a>> {
    use anyhow::Context;
    let tr = walk::tr::TreeCtxt::new(buf).context("Failed to create TreeCtxt")?;
    Ok(tr)
}

/// Determines the base directory from the current working directory.
fn determine_base_directory() -> anyhow::Result<config::root::BaseDirectory> {
    use anyhow::Context;
    let base_dir = config::root::BaseDirectory::from_current_dir()
        .context("Failed to determine base directory")?;
    Ok(base_dir)
}

/// Builds the tree head and prints it.
fn build_and_print_tree_head<'tr, 'a>(
    tr: &mut walk::tr::TreeCtxt<'tr, 'a>,
    base_dir: &mut config::root::BaseDirectory,
) -> anyhow::Result<()> {
    use anyhow::Context;

    tr.path_builder = base_dir
        .build()
        .context("Failed to build base directory path")?;
    tr.path_builder.append_root();
    tr.print_head()?;

    Ok(())
}

/// Iterates over directories and prints the final report.
fn iterate_directories_and_print_report<'tr, 'a>(
    tr: &mut walk::tr::TreeCtxt<'tr, 'a>,
    report_mode: ReportMode,
) -> anyhow::Result<()> {
    tracing::info!("Ready to iterate directories");
    tr.walk_dir(tr.path_builder.base_path())?;
    tr.print_report(report_mode)?;

    Ok(())
}
