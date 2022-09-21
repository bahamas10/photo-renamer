use std::path::Path;
use std::process::Command;

use anyhow::{ensure, Context, Result};
use exif::DateTime;
use log::debug;

/// Get the date from a file using the `exiftool` external program
pub fn get_date(existing_path: &Path) -> Result<DateTime> {
    let output = Command::new("exiftool")
        .arg("-T")
        .arg("-DateTimeOriginal")
        .arg(existing_path)
        .output()
        .context("failed to execute exiftool")?;

    debug!("{:?} exiftool status = {:?}", existing_path, output.status);

    ensure!(
        output.status.success(),
        "exiftool failed, stderr\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let dt = DateTime::from_ascii(&output.stdout)
        .context("failed to parse exiftool date time format")?;

    Ok(dt)
}
