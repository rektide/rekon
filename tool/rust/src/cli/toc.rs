use std::fs;
use std::io::Write;
use std::path::PathBuf;

use super::Result;

pub fn run(
    files: &[PathBuf],
    check: bool,
    min_depth: u8,
    max_depth: u8,
    line_ranges: bool,
) -> Result {
    if min_depth > max_depth {
        return Err("--min-depth must not exceed --max-depth".into());
    }
    let mut updates = Vec::new();
    // Prepare every input before writing any file: a malformed later input
    // must not leave an earlier document partially updated.
    for file in files {
        let source =
            fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
        let document = update(&source, min_depth, max_depth, line_ranges)
            .map_err(|error| format!("{}: {error}", file.display()))?;
        updates.push((file, source, document));
    }
    let mut fresh = true;
    for (file, source, document) in updates {
        if source == document {
            println!("fresh: {}", file.display());
        } else if check {
            eprintln!("stale: {} (table of contents differs)", file.display());
            fresh = false;
        } else {
            // Resolve a symlink deliberately, preserving the link itself. Stage
            // beside its target so replacement remains on the same filesystem.
            let target = fs::canonicalize(file)?;
            let parent = target.parent().ok_or("document has no parent directory")?;
            let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
            temporary
                .as_file()
                .set_permissions(fs::metadata(&target)?.permissions())?;
            temporary.write_all(document.as_bytes())?;
            temporary.persist(&target)?;
            println!("updated: {}", file.display());
        }
    }
    Ok(fresh)
}

pub(super) fn update(
    source: &str,
    min_depth: u8,
    max_depth: u8,
    line_ranges: bool,
) -> std::result::Result<String, super::Error> {
    let options = rekon_toc::TocOptions {
        min_depth,
        max_depth,
        line_ranges,
        ..Default::default()
    };
    Ok(rekon_toc::update_toc(source, &options)?)
}
