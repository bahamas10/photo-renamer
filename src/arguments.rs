use std::path;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Args {
    /// Action to take on filename collision
    #[clap(
        short = 'c',
        long,
        arg_enum,
        default_value_t = Collision::Skip,
        value_name = "collision",
    )]
    pub collision: Collision,

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
        value_name = "dir"
    )]
    pub target_dir: path::PathBuf,

    /// Gatherer to use for file metadata
    #[clap(
        short = 'g',
        long,
        arg_enum,
        default_value_t = Gatherer::Exif,
        value_name = "gatherer",
    )]
    pub gatherer: Gatherer,

    /// Don't actually take any action
    #[clap(short = 'n', long)]
    pub dry_run: bool,

    /// Photos to process
    pub files: Vec<path::PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Collision {
    /// Ignore files (refuse to overwrite)
    Skip,

    /// Rename files
    Rename,

    /// Overwrite files
    Overwrite,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Gatherer {
    /// Use exif data
    Exif,

    /// Use `exiftool`
    Exiftool,

    /// Use `ffprobe`
    Ffprobe,

    /// Use file create time from the filesystem
    FileCreate,

    /// Use file modify time from the filesystem
    FileModify,
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
