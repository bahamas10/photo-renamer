use std::path::Path;
use std::{fs, io};

use anyhow::{bail, ensure, Context, Result};
use chrono::NaiveDateTime;
use exif::{In, Tag, Value};
use log::{debug, trace};

const DATE_FMT: &str = "%Y:%m:%d %H:%M:%S";

/// Get the date from a file using internal exif metadata
pub fn get_date(existing_path: &Path) -> Result<NaiveDateTime> {
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
    let date_str = String::from_utf8_lossy(&data[0]);

    debug!("exif datetime raw {:?}", date_str);

    // parse the date
    let dt = NaiveDateTime::parse_from_str(&date_str, DATE_FMT).with_context(
        || {
            format!(
                "failed to parse exif date time {:?} as fmt {:?}",
                date_str, DATE_FMT
            )
        },
    )?;

    trace!("{:?} -> {:#?}", existing_path, dt);

    Ok(dt)
}
