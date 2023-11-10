//! The CLI logic.

use {
  crate::{
    cli::{
      ArgumentsSubcommand, Cli, DirectorySubcommand, FileSubcommand,
      MainSubcommand::*, MatchSubcommand, Parser,
    },
    logging::append_line_to_file,
  },
  chrono::{SecondsFormat, Utc},
  std::{ffi::OsStr, path::Path},
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

    Directory {
      command: directory_subcommand,
    } => match directory_subcommand {
      DirectorySubcommand::Exists { directory } => {
        if !(directory.exists() && directory.is_dir()) {
          std::process::exit(1);
        }
      }
    },

    File {
      command: file_subcommand,
    } => match file_subcommand {
      FileSubcommand::Exists { file } => {
        if !(file.exists() && file.is_file()) {
          std::process::exit(1);
        }
      }

      FileSubcommand::Parts {
        basename,
        directory,
        extension,
        file,
      } => {
        if basename {
          print!(
            "{}",
            file.file_stem().and_then(OsStr::to_str).unwrap_or_default()
          );
        }

        if directory {
          print!(
            "{}",
            file.parent().and_then(Path::to_str).unwrap_or_default()
          );
        }

        if extension {
          print!(
            "{}",
            file.extension().and_then(OsStr::to_str).unwrap_or_default()
          );
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

    Match {
      command: match_subcommand,
    } => match match_subcommand {
      MatchSubcommand::Regex { pattern, string } => {
        let regex = regex::RegexBuilder::new(&pattern).build().unwrap();
        if !regex.is_match(&string) {
          std::process::exit(1);
        }
      }
    },
  }
}
