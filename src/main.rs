/*!
 * Rename picture and media files based on their exif data
 *
 * Initial code https://gist.github.com/papertigers/ff01ebcd496f433d67e703e39cbff07b
 *
 * Author: Dave Eddy <dave@daveeddy.com>
 * Date: September 19, 2022
 * License: MIT
 */

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, ensure, Context, Result};
use log::debug;

mod arguments;
mod gatherer;

use arguments::{Action, Gatherer};

/// Process a single file given on the command line, returns the new file path
/// if successful
fn process_file(
    args: &arguments::Args,
    existing_path: &Path,
) -> Result<PathBuf> {
    // get file basename (file name)
    let file_basename =
        existing_path.file_name().context("failed to extract filename")?;

    let dt = match args.gatherer {
        Gatherer::Exif => gatherer::exif::get_date(existing_path),
        Gatherer::Exiftool => gatherer::exiftool::get_date(existing_path),
    }?;

    // construct new filename => ":target/:month/:year/:name"
    // XXX should this be customizable?
    let new_path = args
        .target_dir
        .join(format!("{}", dt.year))
        .join(format!("{:02}", dt.month))
        .join(file_basename);
    debug!("{:?} -> {:?}", existing_path, new_path);

    // check if the new path exists already
    ensure!(
        !new_path.exists(),
        "{:?} already exists, refusing to overwrite",
        new_path
    );

    // stop here if dry-run
    if args.dry_run {
        print!("[dry-run] ");
        return Ok(new_path);
    }

    // create intermediate directories (mkdir -p, effectively)
    let parent_dir = new_path
        .parent()
        .with_context(|| format!("parent dir not found for {:?}", new_path))?;
    fs::create_dir_all(&parent_dir)
        .with_context(|| format!("failed to create dirs {:?}", parent_dir))?;

    // performa the action on the file
    let error_msg = format!(
        "failed to {:?} {} -> {}",
        args.action,
        existing_path.display(),
        new_path.display()
    );
    match args.action {
        Action::Copy => fs::copy(&existing_path, &new_path)
            .context(error_msg)
            .map(|_| ())?,
        Action::Move => {
            fs::rename(&existing_path, &new_path).context(error_msg)?
        }
        Action::Hardlink => {
            fs::hard_link(&existing_path, &new_path).context(error_msg)?
        }
    };

    Ok(new_path)
}

fn main() -> Result<()> {
    env_logger::init();

    // parse args
    let args = arguments::parse();
    debug!("{:#?}", args);

    ensure!(!args.files.is_empty(), "at least 1 file must be specified");

    let mut had_error = false;

    // loop files found
    for path in args.files.iter() {
        match process_file(&args, path) {
            Ok(new_path) => {
                // successfully processed file
                println!(
                    "{:?} {} -> {}",
                    args.action,
                    path.display(),
                    new_path.display()
                )
            }
            Err(e) => {
                // failed to process file
                // XXX this is super verbose, should this be behind a `-v` option?
                eprintln!("-----");
                eprintln!("[error] {}", path.display());
                eprintln!("{:?}", e);
                eprintln!("-----");
                had_error = true;
            }
        }
    }

    if had_error {
        bail!("errors seen");
    }

    Ok(())
}
