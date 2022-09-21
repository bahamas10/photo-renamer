use std::path::Path;
use std::process::Command;

use anyhow::{ensure, Context, Result};
use chrono::NaiveDateTime;
use log::debug;

const CMD: &str = "ffprobe";

/// Get the date from a file using the `ffprobe` external program
pub fn get_date(existing_path: &Path) -> Result<NaiveDateTime> {
    let output = Command::new(CMD)
        .arg("-v")
        .arg("quiet")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream_tags=creation_time")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
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

    let date_string = String::from_utf8_lossy(&output.stdout);

    let dt = chrono::DateTime::parse_from_rfc3339(date_string.trim())
        .with_context(|| {
            format!(
                "failed to parse {} output {:?} as a date",
                CMD, date_string
            )
        })?;
    let ndt = NaiveDateTime::new(dt.date_naive(), dt.time());

    Ok(ndt)
}
