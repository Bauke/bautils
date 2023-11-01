//! The CLI definitions.

pub use {
  crate::cli::run::run,
  clap::{Parser, Subcommand},
};

mod run;

/// The main CLI command.
#[derive(Debug, Parser)]
#[command(about, author, version, propagate_version = true)]
pub struct Cli {
  /// The main CLI subcommand.
  #[command(subcommand)]
  pub command: MainSubcommand,
}

/// The main subcommands.
#[derive(Debug, Subcommand)]
pub enum MainSubcommand {
  /// The log subcommand.
  Log {
    /// The data to log.
    #[arg(last = true)]
    data_to_log: Vec<String>,
  },
}
