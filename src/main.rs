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
use chrono::NaiveDateTime;
use log::{debug, info, trace};

mod arguments;
mod gatherer;

use arguments::{Action, Args, Collision, Gatherer};

/// figure out the name of the new file
fn new_file_path(
    existing_path: &Path,
    args: &Args,
    dt: &NaiveDateTime,
) -> Result<PathBuf> {
    // get file basename (file name)
    let file_basename = existing_path
        .file_name()
        .context("failed to extract filename")?
        .to_str()
        .context("failed to parse filename as valid utf8")?;

    let year = dt.format("%Y").to_string();
    let month = dt.format("%m").to_string();

    let mut copy = 0;
    loop {
        // construct new filename => ":target/:month/:year/(:copy) :name"
        // with (:copy) being optional if "rename" mode is specified
        let name = if copy > 0 {
            format!("({}) {}", copy, file_basename)
        } else {
            file_basename.to_string()
        };
        let new_path = args.target_dir.join(&year).join(&month).join(name);
        trace!("trying path {:?}", new_path);

        // just return the new path if it doesn't currently exist (no-collision)
        if !new_path.exists() {
            debug!("{:?} -> {:?}", existing_path, new_path);
            return Ok(new_path);
        }

        // handle collision
        debug!("new path {:?} exists", new_path);
        match args.collision {
            Collision::Skip => {
                // return here with an error if we are in skip mode
                bail!("{:?} already exists, refusing to overwrite", new_path);
            }
            Collision::Overwrite => {
                // break the loop here and continue if we are in overwrite
                // mode
                info!("overwriting {:?} with {:?}", new_path, existing_path);
                return Ok(new_path);
            }
            Collision::Rename => {
                // try to create a new filename if rename is set
                copy += 1;
            }
        };
    }
}

/// Process a single file given on the command line, returns the new file path
/// if successful
fn process_file(args: &Args, existing_path: &Path) -> Result<PathBuf> {
    // get the date time for the file
    let dt = match args.gatherer {
        Gatherer::Exif => gatherer::exif::get_date(existing_path),
        Gatherer::Exiftool => gatherer::exiftool::get_date(existing_path),
        Gatherer::Ffprobe => gatherer::ffprobe::get_date(existing_path),
    }?;

    // construct the new filename
    let new_path = new_file_path(existing_path, args, &dt)?;

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

    // perform the action on the file (move, copy, hardlink)
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
