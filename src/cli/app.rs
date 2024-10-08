use clap::{value_parser, Arg, ArgAction, Command};

pub mod cli_options {
    pub mod misc {
        pub const VERSION: &str = "version";
        pub const LEVEL: &str = "level";
    }

    pub mod path {
        pub const ABSOLUTE: &str = "absolute";
        pub const RELATIVE: &str = "relative";
    }

    pub mod sort {
        pub const REVERSE: &str = "reverse";
        pub const NO_SORT: &str = "no-sort";
        pub const SORT: &str = "sort";
        pub const FILES_FIRST: &str = "files-first";
    }

    pub mod color {
        pub const COLOR: &str = "color"; // TODO: Remove
        pub const NO_COLOR: &str = "no-color";
    }

    pub mod read {
        pub const VISIBLE: &str = "visible";
        pub const ALL: &str = "all";
        pub const FOLDER: &str = "folder";
    }

    pub mod meta {
        pub const META: &str = "meta";
        pub const PERMISSION: &str = "permission";
        pub const BTIME: &str = "btime";
        pub const MTIME: &str = "mtime";
        pub const ATIME: &str = "atime";
        pub const SIZE: &str = "size";
    }

    pub mod report {
        pub const YIELD: &str = "yield";
    }

    pub mod branch {
        pub const NO_BRANCH: &str = "no-branch";
    }
}

pub fn tree_app() -> Command {
    use cli_options::*;

    Command::new("tree-rs")
        .infer_long_args(true)
        .args_override_self(true)
        .arg(
            Arg::new(misc::VERSION)
                .long(misc::VERSION)
                .short('V')
                .action(ArgAction::SetTrue)
                .help("Print current version of Tree-rs."),
        )
        .arg(
            Arg::new(path::ABSOLUTE)
                .long(path::ABSOLUTE)
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Print file/dir name along with its absolute path"),
        )
        .arg(
            Arg::new(path::RELATIVE)
                .long(path::RELATIVE)
                .short('f')
                .action(ArgAction::SetTrue)
                .help("Print file/dir name along with its relative path"),
        )
        .arg(
            Arg::new(sort::REVERSE)
                .long(sort::REVERSE)
                .short('r')
                .action(ArgAction::SetTrue)
                .help("Sort entries in reverse order."),
        )
        .arg(
            Arg::new(sort::NO_SORT)
                .long(sort::NO_SORT)
                .short('S')
                .action(ArgAction::SetTrue)
                .help("Do not sort entries."),
        )
        .arg(
            Arg::new(sort::SORT)
                .long(sort::SORT)
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Sort entries alphabetically."),
        )
        .arg(
            Arg::new(sort::FILES_FIRST)
                .long(sort::FILES_FIRST)
                .short('F')
                .action(ArgAction::SetTrue)
                .help("Sort files before directories."),
        )
        .arg(
            Arg::new(color::COLOR)
                .long(color::COLOR)
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Print entries with color."),
        )
        .arg(
            Arg::new(color::NO_COLOR)
                .long(color::NO_COLOR)
                .short('C')
                .action(ArgAction::SetTrue)
                .help("Print entries without color."),
        )
        .arg(
            Arg::new(read::VISIBLE)
                .long(read::VISIBLE)
                .action(ArgAction::SetTrue)
                .help("Print only visible entries."),
        )
        .arg(
            Arg::new(read::ALL)
                .long(read::ALL)
                .action(ArgAction::SetTrue)
                .help("Print all entries."),
        )
        .arg(
            Arg::new(read::FOLDER)
                .long(read::FOLDER)
                .aliases(&["folders", "directories"])
                .action(ArgAction::SetTrue)
                .help("Print directories only."),
        )
        .arg(
            Arg::new(meta::META)
                .long(meta::META)
                .short('m')
                .action(ArgAction::SetTrue)
                .help("Print all default metadata for entries."),
        )
        .arg(
            Arg::new(meta::PERMISSION)
                .long(meta::PERMISSION)
                .short('p')
                .action(ArgAction::SetTrue)
                .help("Print entry permissions."),
        )
        .arg(
            Arg::new(meta::BTIME)
                .long(meta::BTIME)
                .action(ArgAction::SetTrue)
                .help("Print entry creation time."),
        )
        .arg(
            Arg::new(meta::MTIME)
                .long(meta::MTIME)
                .action(ArgAction::SetTrue)
                .help("Print entry modification time."),
        )
        .arg(
            Arg::new(meta::ATIME)
                .long(meta::ATIME)
                .action(ArgAction::SetTrue)
                .help("Print entry last access time."),
        )
        .arg(
            Arg::new(meta::SIZE)
                .long(meta::SIZE)
                .action(ArgAction::SetTrue)
                .help("Print entry size."),
        )
        .arg(
            Arg::new(misc::LEVEL)
                .long(misc::LEVEL)
                .short('L')
                .num_args(1)
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .help("Print tree until certain depth. Default depth: 5000"),
        )
        .arg(
            Arg::new(report::YIELD)
                .long(report::YIELD)
                .short('y')
                .action(ArgAction::SetTrue)
                .help("Print an exhaustive report."),
        )
        .arg(
            Arg::new(branch::NO_BRANCH)
                .long(branch::NO_BRANCH)
                .short('B')
                .action(ArgAction::SetTrue)
                .help("Omit branch lines from the output."),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_args() {
        let cloned_args = vec!["tree-rs", "--version", "--absolute"]
            .into_iter()
            .map(String::from);

        let matches = tree_app()
            .try_get_matches_from(cloned_args)
            .unwrap_or_else(|e| e.exit());

        println!("{:?}", matches);

        dbg!(matches);
    }

    #[test]
    fn test_tree_args_level() {
        let cloned_args = vec!["tree-rs", "--level", "8"]
            .into_iter()
            .map(String::from);

        let matches = tree_app()
            .try_get_matches_from(cloned_args)
            .unwrap_or_else(|e| e.exit());

        println!("{:?}", matches);

        dbg!(matches);
    }
}
