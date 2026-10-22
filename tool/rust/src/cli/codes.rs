use std::path::Path;

use rekon::codes::{CodesCheck, check_registry, generate, write_registry};

use super::Result;

pub fn run(dir: &Path, output: &Path, check: bool) -> Result {
    let generation = generate(dir, output)?;
    if check {
        match check_registry(&generation)? {
            CodesCheck::Fresh => {
                println!(
                    "fresh: {} ({} UTF-8 bytes)",
                    generation.output.display(),
                    generation.document.len()
                );
                Ok(true)
            }
            CodesCheck::Missing => {
                eprintln!("stale: {} (missing output)", generation.output.display());
                Ok(false)
            }
            CodesCheck::Stale {
                first_difference,
                expected_bytes,
                actual_bytes,
            } => {
                eprintln!(
                    "stale: {} (first differing UTF-8 byte {first_difference}; expected {expected_bytes} bytes, found {actual_bytes})",
                    generation.output.display()
                );
                Ok(false)
            }
        }
    } else {
        write_registry(&generation)?;
        println!(
            "generated: {} from {} ({} UTF-8 bytes)",
            generation.output.display(),
            dir.display(),
            generation.document.len()
        );
        Ok(true)
    }
}
