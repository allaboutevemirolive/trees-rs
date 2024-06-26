use clap::Arg;
use clap::ArgAction;
use clap::Command;

pub mod options {
    pub mod miscellaneous {
        pub static VERSION: &str = "version";
        pub static LEVEL: &str = "level-bounds";
    }

    pub mod path {
        pub static ABSOLUTE: &str = "absolute-path";
        pub static RELATIVE: &str = "relative-path";
    }

    pub mod sort {
        pub static ASCENDING: &str = "general-sort";
        pub static REVERSE: &str = "reverse-sort";
        pub static NOSORT: &str = "no-sort";
        pub static FILEFIRST: &str = "sort-file-first";
    }

    pub mod color {
        pub static COLOR: &str = "color-entries"; // TODO: Remove
        pub static COLORLESS: &str = "no-color-entries";
    }

    pub mod read {
        pub static ALL: &str = "read-all-entries";
        pub static VISIBLE: &str = "read-visible-entries";
        pub static FOLDER: &str = "read-folders";
    }

    pub mod meta {
        pub static META: &str = "show-all-default-metadata";
        pub static PERMISSION: &str = "show-entry-permission";
        pub static BTIME: &str = "show-entries-creation-time";
        pub static MTIME: &str = "show-entries-modification-time";
        pub static ATIME: &str = "show-entries-access-time";
        pub static SIZE: &str = "show-entries-size";
    }

    pub mod report {
        pub static YIELD: &str = "show-exhaustive-report";
    }

    pub mod branch {
        pub static NOBRANCH: &str = "discard-branch-stick-from-output";
    }
}

pub fn tree_app() -> Command {
    Command::new("tree-rs")
        .infer_long_args(true)
        .args_override_self(true)
        .arg(
            Arg::new(options::miscellaneous::VERSION)
                .long("version")
                .short('V')
                .help("Print current version of Tree-rs.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::path::ABSOLUTE)
                .long("absolute")
                .short('A')
                .help("Print file/dir name along with it absolute path")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::path::RELATIVE)
                .long("relative")
                .short('f')
                .help("Print file/dir name along with it relative path")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::sort::REVERSE)
                .long("reverse")
                .short('r')
                .help("Sort entires in ascending order.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::sort::NOSORT)
                .long("no-sort")
                .short('S')
                .help("No entries sort.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::sort::ASCENDING)
                .long("sort")
                .short('s')
                .help("Sort entries.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::sort::FILEFIRST)
                .long("filesfirst")
                .short('F')
                .help("Sort files first.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::color::COLOR)
                .long("color")
                .short('c')
                .help("Print entries with color.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::color::COLORLESS)
                .long("color-less")
                .short('C')
                .help("Print entries without color.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::read::VISIBLE)
                .long("visible")
                .help("Print visible entries only.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::read::ALL)
                .long("all")
                .help("Print all entries.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::read::FOLDER)
                .long("folder")
                .help("Print directoris only.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::meta::META)
                .long("meta")
                .short('m')
                .help("Print all default entry's metadata.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::meta::PERMISSION)
                .long("permission")
                .short('p')
                .help("Print entires attribute.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::meta::BTIME)
                .long("btime")
                .help("Print the date that the entry was created.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::meta::MTIME)
                .long("mtime")
                .help("Print the date that the entry was modified.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::meta::ATIME)
                .long("atime")
                .help("Print the date that the entry was last time accessed.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::meta::SIZE)
                .long("size")
                .help("Print entires's size.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::miscellaneous::LEVEL)
                .long("level")
                .short('L')
                .num_args(1)
                .help("Print tree until certain depth. Default depth: 5000")
                .value_parser(clap::value_parser!(usize))
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(options::report::YIELD)
                .long("yield")
                .short('y')
                .help("Print exhaustive report")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::branch::NOBRANCH)
                .long("nobranch")
                .short('B')
                .help("Discard branch's stick from the output")
                .action(ArgAction::SetTrue),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test test_tree_args -- --nocapture
    #[test]
    fn test_tree_args() {
        let cloned_args: Vec<String> = vec![
            String::from("tree-rs"),
            String::from("--version"),
            String::from("--absolute"),
        ];

        let matches = tree_app()
            .try_get_matches_from(cloned_args.clone())
            .unwrap_or_else(|e| e.exit());

        println!("{:?}", matches);

        dbg!(matches);
    }

    #[test]
    fn test_tree_args_level() {
        let cloned_args: Vec<String> = vec![
            String::from("tree-rs"),
            String::from("--level"),
            String::from("8"),
        ];

        let matches = tree_app()
            .try_get_matches_from(cloned_args.clone())
            .unwrap_or_else(|e| e.exit());

        println!("{:?}", matches);

        dbg!(matches);
    }
}
