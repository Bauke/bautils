//! All logic for logging things.

use std::{fs::OpenOptions, io::prelude::*};

/// Append the `log_line` to a given file at `path` and creates the file if it
/// doesn't exist.
pub fn append_line_to_file(
  path: &str,
  log_line: &str,
) -> Result<(), std::io::Error> {
  let mut log_file = OpenOptions::new().create(true).append(true).open(path)?;
  writeln!(log_file, "{}", log_line)?;
  Ok(())
}
