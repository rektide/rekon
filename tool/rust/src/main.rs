//! `rekon` CLI entry point: the super-tool surface.

use std::process::ExitCode;

use clap::{CommandFactory, FromArgMatches, Parser};
use rekon::DEFAULT_MANIFEST;
use rekon::coverage::{DEFAULT_COVERAGE, audit};
use std::path::PathBuf;

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
    /// Check declared operating coverage independently of generated-file freshness
    AuditContext {
        #[arg(short, long, default_value = DEFAULT_MANIFEST)]
        manifest: PathBuf,
        #[arg(long, default_value = DEFAULT_COVERAGE)]
        coverage: PathBuf,
    },
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
        Command::AuditContext { manifest, coverage } => {
            let result = std::env::current_dir()
                .map_err(|error| format!("current directory: {error}"))
                .and_then(|cwd| {
                    audit(&manifest, &coverage, &cwd).map_err(|error| error.to_string())
                });
            match result {
                Ok(report) if report.findings.is_empty() => {
                    println!(
                        "coverage: {} baseline sections, {} required excerpts checked (declared coverage, not semantic equivalence)",
                        report.sections, report.assertions
                    );
                    ExitCode::SUCCESS
                }
                Ok(report) => {
                    eprintln!("coverage: {} problem(s)", report.findings.len());
                    for finding in report.findings {
                        eprintln!("  {finding}");
                    }
                    ExitCode::from(1)
                }
                Err(message) => {
                    eprintln!("{message}");
                    ExitCode::from(1)
                }
            }
        }
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
