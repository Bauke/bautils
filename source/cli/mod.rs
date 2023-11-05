//! The CLI definitions.

pub use {
  crate::cli::run::run,
  clap::{Parser, Subcommand},
  std::path::PathBuf,
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

  /// The file subcommand.
  File {
    /// The file subcommands.
    #[command(subcommand)]
    command: FileSubcommand,
  },

  /// The log subcommand.
  Log {
    /// The data to log.
    #[arg(last = true)]
    data_to_log: Vec<String>,

    /// The file to log to, it will be created if it doesn't exist.
    #[arg(short, long, default_value = "bautils.log")]
    file: String,
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

/// The file subcommands
#[derive(Debug, Subcommand)]
pub enum FileSubcommand {
  /// Check whether a file exists, if the file does not exist the exit code will
  /// be 1.
  Exists {
    /// The path to a potential file.
    #[arg()]
    file: PathBuf,
  },
}
