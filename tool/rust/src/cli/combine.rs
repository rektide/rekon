use std::path::Path;

use rekon::assembly::{CheckResult, assemble, check_assembly, write_assembly};

use super::Result;

pub fn run(manifest: &Path, output: Option<&Path>, check: bool) -> Result {
    let mut assembly = assemble(manifest, &std::env::current_dir()?, output)?;
    assembly.document = super::toc::update(&assembly.document, 2, 6, true)?;
    if check {
        match check_assembly(&assembly)? {
            CheckResult::Fresh => {
                println!("fresh: {} ({} UTF-8 bytes)", assembly.output.display(), assembly.document.len());
                Ok(true)
            }
            CheckResult::Missing => {
                eprintln!("stale: {} (missing output)", assembly.output.display());
                Ok(false)
            }
            CheckResult::Stale { first_difference, expected_bytes, actual_bytes } => {
                eprintln!(
                    "stale: {} (first differing UTF-8 byte {first_difference}; expected {expected_bytes} bytes, found {actual_bytes})",
                    assembly.output.display()
                );
                Ok(false)
            }
        }
    } else {
        write_assembly(&assembly)?;
        println!("assembled: {} ({} UTF-8 bytes)", assembly.output.display(), assembly.document.len());
        Ok(true)
    }
}
