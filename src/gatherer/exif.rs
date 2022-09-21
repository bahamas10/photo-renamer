use std::path::Path;
use std::{fs, io};

use anyhow::{bail, ensure, Context, Result};
use exif::{DateTime, Value};
use exif::{In, Tag};
use log::trace;

/// Get the date from a file using internal exif metadata
pub fn get_date(existing_path: &Path) -> Result<DateTime> {
    // open the file
    let file = fs::File::open(&existing_path)?;
    let mut br = io::BufReader::new(&file);

    // read the exif data
    let exifread = exif::Reader::new();
    let exif = exifread
        .read_from_container(&mut br)
        .context("failed to read exif data")?;

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

    trace!("{:?} -> {:#?}", existing_path, dt);

    Ok(dt)
}
