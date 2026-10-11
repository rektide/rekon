//! `rekon` CLI entry point.

mod cli;

use std::process::ExitCode;

use clap::{CommandFactory, FromArgMatches};

fn main() -> ExitCode {
    let args = match cli::Cli::command().try_get_matches() {
        Ok(matches) => matches,
        Err(error) => {
            if error.use_stderr() {
                eprintln!("{error}");
                return ExitCode::from(1);
            }
            print!("{error}");
            return ExitCode::SUCCESS;
        }
    };
    let cli = cli::Cli::from_arg_matches(&args).expect("clap validated these matches");
    match cli::run(cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("{error}");
            let mut cause = error.source();
            while let Some(source) = cause {
                eprintln!("  caused by: {source}");
                cause = source.source();
            }
            ExitCode::from(1)
        }
    }
}
