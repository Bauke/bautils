//! The CLI logic.

use {
  crate::{
    cli::{
      ArgumentsSubcommand, Cli, FileSubcommand, MainSubcommand::*, Parser,
    },
    logging::append_line_to_file,
  },
  chrono::{SecondsFormat, Utc},
};

/// Parse the CLI arguments and execute them.
pub fn run() {
  let cli = Cli::parse();

  match cli.command {
    Arguments {
      command: arguments_subcommand,
    } => match arguments_subcommand {
      ArgumentsSubcommand::Count { arguments, newline } => {
        let mut count = format!("{}", arguments.len());
        if newline {
          count.push('\n');
        }

        print!("{}", count);
      }
    },

    File {
      command: file_subcommand,
    } => match file_subcommand {
      FileSubcommand::Exists { file } => {
        if !file.exists() {
          std::process::exit(1);
        }
      }
    },

    Log { data_to_log, file } => {
      let log_line = format!(
        "{} {}",
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        data_to_log.join(" ")
      );
      println!("{}", log_line);
      append_line_to_file(&file, &log_line).unwrap();
    }
  }
}
