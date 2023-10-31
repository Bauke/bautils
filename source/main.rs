//! # Bautils
//!
//! > **Bauke's Ad-hoc Utilities.**

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

mod cli;

fn main() {
  cli::run();
}
