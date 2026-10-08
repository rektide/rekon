//! `rekon` CLI entry point: the super-tool surface.

use std::process::ExitCode;

use clap::{CommandFactory, FromArgMatches, Parser};

/// The design note the `toc` stub points at.
const TOOLCHAIN_DESIGN: &str = "design/toolchain/draft0.glm53m.md";

#[derive(Parser)]
#[command(
    name = "rekon",
    version,
    about = "Rekon super-tool (rust); subcommands are the toolchain surface",
    arg_required_else_help = true,
    subcommand_required = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Assemble AGENTS.md from manifest-selected GLOBAL.md fragments (landing next)
    Combine,
    /// Placeholder: rust ToC subcommand (follows combine in the accepted roadmap)
    Toc,
}

fn main() -> ExitCode {
    // Exit codes follow the assembler contract: help succeeds; unknown flags
    // fail with 1 (not clap's default 2).
    let args = match Cli::command().try_get_matches() {
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
    let cli = Cli::from_arg_matches(&args).expect("clap reported these matches as valid");
    match cli.command {
        Command::Toc => {
            eprintln!("not yet implemented — see {TOOLCHAIN_DESIGN} (rust toc follows combine)");
            ExitCode::from(2)
        }
        Command::Combine => {
            eprintln!("not yet implemented in this stage; see {TOOLCHAIN_DESIGN}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn cli_accepts_the_two_declared_subcommands() {
        assert_eq!(
            Cli::command()
                .try_get_matches_from(["rekon", "toc"])
                .expect("valid toc invocation")
                .subcommand_name(),
            Some("toc")
        );
        assert_eq!(
            Cli::command()
                .try_get_matches_from(["rekon", "combine"])
                .expect("valid combine invocation")
                .subcommand_name(),
            Some("combine")
        );
    }
}
