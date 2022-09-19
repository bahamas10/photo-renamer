use std::path;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Args {
    /// Action to take for file organization
    #[clap(
        short = 'a',
        long,
        arg_enum,
        default_value_t = Action::Move,
        value_name = "action",
    )]
    pub action: Action,

    /// Directory to output files to
    #[clap(
        short = 'd',
        long,
        parse(from_os_str),
        default_value = ".",
        value_name = "dir",
    )]
    pub target_dir: path::PathBuf,

    /// Don't actually take any action
    #[clap(short = 'n', long)]
    pub dry_run: bool,

    /// Photos to process
    pub files: Vec<path::PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Action {
    /// Move files (rename)
    Move,

    /// Copy files
    Copy,

    /// Hard link files
    Hardlink,
}

pub fn parse() -> Args {
    Args::parse()
}
