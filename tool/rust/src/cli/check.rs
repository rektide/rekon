//! `rekon check` — run document checks over explicit Markdown paths.

use std::fs;
use std::path::{Path, PathBuf};

use rekon::check::{CheckOptions, DocumentChecks, DocumentFiles, Failure, check_document};

use super::Result;

struct DiskFiles;

impl DocumentFiles for DiskFiles {
    fn exists(&self, path: &Path) -> bool {
        fs::metadata(path).is_ok()
    }

    fn read(&self, path: &Path) -> std::io::Result<String> {
        fs::read_to_string(path)
    }
}

pub fn run(files: &[PathBuf], root: Option<&Path>, okf: bool) -> Result {
    let cwd = std::env::current_dir()?;
    let root = root.map(|declared| cwd.join(declared));
    let mut failures: Vec<Failure> = Vec::new();
    let mut destinations = 0usize;
    let mut sources = 0usize;
    for file in files {
        let display = file.display().to_string();
        let raw = match fs::read_to_string(file) {
            Ok(raw) => raw,
            Err(error) => {
                failures.push(Failure::at(&display, None, format!("unreadable: {error}")));
                continue;
            }
        };
        let document = fs::canonicalize(file).unwrap_or_else(|_| cwd.join(file));
        let document_root = root
            .clone()
            .unwrap_or_else(|| discover_root(&document, &cwd));
        let checks: DocumentChecks = check_document(
            &raw,
            &document,
            &display,
            &CheckOptions {
                root: &document_root,
                okf,
            },
            &DiskFiles,
        );
        destinations += checks.destinations;
        sources += checks.sources;
        failures.extend(checks.failures);
    }
    for failure in &failures {
        eprintln!("{}", failure.render());
    }
    println!(
        "{} documents, {} links, {} sources checked",
        files.len(),
        destinations,
        sources
    );
    if failures.is_empty() {
        println!(
            "document checks passed (structure, destinations, anchors{}).",
            if okf { ", provenance" } else { "" }
        );
        Ok(true)
    } else {
        eprintln!("{} problem(s)", failures.len());
        Ok(false)
    }
}

/// Bundle root for `/…` destinations: the nearest ancestor holding a jj or git
/// marker, falling back to the invocation directory.
fn discover_root(document: &Path, cwd: &Path) -> PathBuf {
    let mut directory = document
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| cwd.to_path_buf());
    loop {
        if directory.join(".jj").exists() || directory.join(".git").exists() {
            return directory;
        }
        if !directory.pop() {
            return cwd.to_path_buf();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::discover_root;
    use std::path::Path;

    #[test]
    fn root_discovery_falls_back_to_cwd_above_the_repository() {
        // This test runs inside the workspace, so a real marker is on the path.
        let cwd = std::env::current_dir().unwrap();
        let root = discover_root(&cwd, Path::new("/fallback"));
        assert!(root.join(".jj").exists() || root.join(".git").exists());
        assert_eq!(
            discover_root(Path::new("/definitely/not/a/repo/doc.md"), Path::new("/fallback")),
            Path::new("/fallback")
        );
    }
}
