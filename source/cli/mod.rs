pub use {crate::cli::run::run, clap::Parser};

mod run;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Cli {}
