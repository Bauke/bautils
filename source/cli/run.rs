//! The CLI logic.

use super::{Cli, Parser};

/// Parse the CLI arguments and execute them.
pub fn run() {
  let cli = Cli::parse();
  dbg!(cli);
}
