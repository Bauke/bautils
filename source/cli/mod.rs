//! The CLI definitions.

pub use {crate::cli::run::run, clap::Parser};

mod run;

/// The main CLI command.
#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Cli {}
