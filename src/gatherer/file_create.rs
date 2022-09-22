use std::path::Path;
use std::fs;

use anyhow::Result;
use chrono::{DateTime, Utc, NaiveDateTime};

/// Get the date from a file using file create timestamp
pub fn get_date(existing_path: &Path) -> Result<NaiveDateTime> {
    // open the file
    let metadata = fs::metadata(&existing_path)?;
    let created = metadata.created()?;

    let dt: DateTime<Utc> = created.into();

    Ok(dt.naive_utc())
}
