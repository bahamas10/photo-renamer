/*!
 * Rename picture and media files based on their exif data
 *
 * Initial code https://gist.github.com/papertigers/ff01ebcd496f433d67e703e39cbff07b
 *
 * Author: Dave Eddy <dave@daveeddy.com>
 * Date: September 19, 2022
 * License: MIT
 */

use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::{bail, ensure, Context, Result};
use exif::Exif;
use exif::{DateTime, Value};
use exif::{In, Tag};
use log::{debug, trace};

mod arguments;

use arguments::Action;

/// Extract date-time from a given exif object
fn get_datetime(exif: Exif) -> Result<DateTime> {
    // extract the date time exif field
    let date = exif
        .get_field(Tag::DateTime, In::PRIMARY)
        .context("failed to get date exif data")?;

    // ensure the data is in ascii format
    let data = match &date.value {
        Value::Ascii(vec) => vec,
        o => bail!("incorrect exif data format: {:?}", o),
    };

    // ensure data is actually present
    ensure!(!data.is_empty(), "empty exif date data found");
    let data = &data[0];

    // parse the date
    let dt = DateTime::from_ascii(data)
        .context("failed to parse exif date time format")?;

    Ok(dt)
}

/// Process a single file given on the command line, returns the new file path
/// if successful
fn process_file(
    args: &arguments::Args,
    existing_path: &Path,
) -> Result<PathBuf> {
    // get file basename (file name)
    let file_basename =
        existing_path.file_name().context("failed to extract filename")?;

    // open the file
    let file = fs::File::open(&existing_path)?;
    let mut br = io::BufReader::new(&file);

    // read the exif data
    let exifread = exif::Reader::new();
    let exif = exifread
        .read_from_container(&mut br)
        .context("failed to read exif data")?;

    // get date time
    let dt = get_datetime(exif)?;
    trace!("{:?} -> {:#?}", existing_path, dt);

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
