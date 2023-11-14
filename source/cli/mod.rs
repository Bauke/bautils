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

  /// The directory subcommand.
  Directory {
    /// The directory subcommands.
    #[command(subcommand)]
    command: DirectorySubcommand,
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

  /// The match subcommand.
  Match {
    /// The match subcommands.
    #[command(subcommand)]
    command: MatchSubcommand,
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

/// The directory subcommands.
#[derive(Debug, Subcommand)]
pub enum DirectorySubcommand {
  /// Check whether a directory exists, if the directory does not exist the exit
  /// code will be 1.
  Exists {
    /// The path to a potential directory.
    #[arg()]
    directory: PathBuf,
  },
}

/// The file subcommands.
#[derive(Debug, Subcommand)]
pub enum FileSubcommand {
  /// Check whether a file exists, if the file does not exist the exit code will
  /// be 1.
  Exists {
    /// The path to a potential file.
    #[arg()]
    file: PathBuf,
  },

  /// Extract metadata of a file.
  Metadata {
    /// The chrono format string for dates, defaults to ISO 8601.
    #[arg(long, default_value = "%FT%T%z")]
    date_format: String,

    /// Get the modified date (uses --date-format as the format string).
    #[arg(long, default_value = "false")]
    modified: bool,

    /// The file to get the metadata of.
    #[arg()]
    file: PathBuf,
  },

  /// Extract parts of a file.
  Parts {
    /// Print the base name of the file (without the extension).
    #[arg(long, group = "part-to-print", default_value = "false")]
    basename: bool,

    /// Print the directory the file is in.
    #[arg(long, group = "part-to-print", default_value = "false")]
    directory: bool,

    /// Print the file extension (without the leading dot).
    #[arg(long, group = "part-to-print", default_value = "false")]
    extension: bool,

    /// The file to include parts from.
    #[arg()]
    file: PathBuf,
  },
}

/// The match subcommands.
#[derive(Debug, Subcommand)]
pub enum MatchSubcommand {
  /// Match a given string to a regular expression pattern, if the pattern
  /// matches the exit code will be 0, otherwise it will be 1. Regular
  /// expression parsing failures will intentionally cause a panic.
  Regex {
    /// The regular expression to match with.
    #[arg(short, long)]
    pattern: String,

    /// The string to test.
    #[arg()]
    string: String,
  },
}
