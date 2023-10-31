use super::{Cli, Parser};

pub fn run() {
  let cli = Cli::parse();
  dbg!(cli);
}
