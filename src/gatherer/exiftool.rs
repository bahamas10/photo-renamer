use std::path::Path;
use std::process::Command;

use anyhow::{ensure, Context, Result};
use chrono::NaiveDateTime;
use log::debug;

const CMD: &str = "exiftool";
const DATE_FMT: &str = "%Y:%m:%d %H:%M:%S";

/// Get the date from a file using the `exiftool` external program
pub fn get_date(existing_path: &Path) -> Result<NaiveDateTime> {
    let output = Command::new(CMD)
        .arg("-T")
        .arg("-DateTimeOriginal")
        .arg(existing_path)
        .output()
        .with_context(|| format!("failed to execute {}", CMD))?;

    debug!("{:?} {} status = {:?}", existing_path, CMD, output.status);

    ensure!(
        output.status.success(),
        "{} failed, stderr\n{}",
        CMD,
        String::from_utf8_lossy(&output.stderr)
    );

    let date_str = String::from_utf8_lossy(&output.stdout);

    debug!("{} datetime raw {:?}", CMD, date_str);

    // parse the date
    let dt = NaiveDateTime::parse_from_str(date_str.trim(), DATE_FMT)
        .with_context(|| {
            format!(
                "failed to parse `{}` date time {:?} as fmt {:?}",
                CMD, date_str, DATE_FMT
            )
        })?;

    Ok(dt)
}
