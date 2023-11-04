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
  /// The arguments subcommand.
  Arguments {
    /// The arguments subcommands.
    #[command(subcommand)]
    command: ArgumentsSubcommand,
  },

  /// The log subcommand.
  Log {
    /// The data to log.
    #[arg(last = true)]
    data_to_log: Vec<String>,
  },
}

/// The arguments subcommands.
#[derive(Debug, Subcommand)]
pub enum ArgumentsSubcommand {
  /// Print the count of provided arguments.
  Count {
    /// The arguments to print the count of.
    #[arg(last = true)]
    arguments: Vec<String>,

    /// Include a newline at the end of the number.
    #[arg(short, long, default_value = "false")]
    newline: bool,
  },
}
