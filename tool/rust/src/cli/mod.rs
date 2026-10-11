//! Thin command adapters over the independently usable document engines.

mod combine;
mod okf;
mod toc;

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use rekon::DEFAULT_MANIFEST;
use rekon::coverage::{DEFAULT_COVERAGE, audit};

type Error = Box<dyn std::error::Error>;
type Result = std::result::Result<bool, Error>;

#[derive(Parser)]
#[command(
    name = "rekon",
    version,
    about = "Assemble enriched guidance, generate tables of contents, and inspect scoped OKF",
    arg_required_else_help = true,
    subcommand_required = true
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Assemble complete fragments with scoped metadata and a final table of contents
    Combine {
        #[arg(short, long, default_value = DEFAULT_MANIFEST)]
        manifest: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Compare final bytes without writing
        #[arg(short, long)]
        check: bool,
    },
    /// Update a semantic navigation region in Markdown or enriched MDX
    Toc {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Compare final bytes without writing
        #[arg(short, long)]
        check: bool,
        #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(1..=6))]
        min_depth: u8,
        #[arg(long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..=6))]
        max_depth: u8,
        /// Include physical line ranges in navigation attributes
        #[arg(long)]
        line_ranges: bool,
    },
    /// Read static document or section metadata without evaluating JavaScript
    Okf {
        #[command(subcommand)]
        command: OkfCommand,
    },
    /// Check declared operating coverage independently of generated-file freshness
    AuditContext {
        #[arg(short, long, default_value = DEFAULT_MANIFEST)]
        manifest: PathBuf,
        #[arg(long, default_value = DEFAULT_COVERAGE)]
        coverage: PathBuf,
    },
}

#[derive(Subcommand)]
enum OkfCommand {
    /// Extract YAML frontmatter and scoped Okf data records
    Extract {
        file: PathBuf,
        /// Select an enclosing section/article ID (optionally prefixed with #)
        #[arg(long)]
        scope: Option<String>,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Json,
    Yaml,
}

pub fn run(cli: Cli) -> Result {
    match cli.command {
        Command::Combine { manifest, output, check } => combine::run(&manifest, output.as_deref(), check),
        Command::Toc { files, check, min_depth, max_depth, line_ranges } => {
            toc::run(&files, check, min_depth, max_depth, line_ranges)
        }
        Command::Okf { command: OkfCommand::Extract { file, scope, format } } => {
            okf::run(&file, scope.as_deref(), format)
        }
        Command::AuditContext { manifest, coverage } => {
            let report = audit(&manifest, &coverage, &std::env::current_dir()?)?;
            if report.findings.is_empty() {
                println!(
                    "coverage: {} baseline sections, {} required excerpts checked (declared coverage, not semantic equivalence)",
                    report.sections, report.assertions
                );
                Ok(true)
            } else {
                eprintln!("coverage: {} problem(s)", report.findings.len());
                for finding in report.findings {
                    eprintln!("  {finding}");
                }
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn help_and_command_arguments_are_consistent() {
        Cli::command().debug_assert();
        for args in [
            vec!["rekon", "combine", "--check"],
            vec!["rekon", "toc", "AGENTS.md", "--line-ranges"],
            vec!["rekon", "okf", "extract", "AGENTS.md", "--scope", "fragment-code"],
        ] {
            Cli::command().try_get_matches_from(args).unwrap();
        }
        assert!(Cli::command().try_get_matches_from(["rekon", "toc"]).is_err());
    }
}
