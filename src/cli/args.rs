use clap::{arg, command, value_parser, Arg, ArgAction, Command};

mod miscellaneous {
    pub static VERSION: &str = "Version";
    pub static LEVEL: &str = "Level";
}

mod path {
    pub static ABSOLUTE: &str = "Absolute Path";
    pub static RELATIVE: &str = "Relative Path";
}

pub fn tree_app() -> Command {
    Command::new("tree-rs")
        .arg(
            Arg::new(miscellaneous::VERSION)
                .long("version")
                .short('V')
                .help("Print current version of Tree-rs.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(path::ABSOLUTE)
                .long("absolute")
                .short('A')
                .help("Print file/dir name along with it absolute path")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(path::RELATIVE)
                .long("relative")
                .short('R')
                .help("Print file/dir name along with it relative path")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(miscellaneous::LEVEL)
                .long("level")
                .short('L')
                .num_args(1)
                .help("Print tree until certain depth. Default depth: 5000")
                .value_parser(clap::value_parser!(usize))
                .action(clap::ArgAction::Set),
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
