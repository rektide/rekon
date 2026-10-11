//! Declared operating coverage, independent of byte-assembly freshness.
//!
//! A reviewed baseline and required visible excerpts make omissions observable.
//! This is not semantic equivalence checking. The auditor never writes files.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::manifest::{ManifestError, parse_manifest, plan_assembly, validate_fragment_path};

pub const DEFAULT_COVERAGE: &str = "tool/assembly/coverage.json";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coverage {
    pub baseline: String,
    pub sha256: String,
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub heading: String,
    /// Human rationale for the extraction, refinement, or routing decision.
    pub note: String,
    pub checks: Vec<Requirement>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Requirement {
    pub fragment: String,
    pub contains: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Report {
    pub sections: usize,
    pub assertions: usize,
    pub findings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("{path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}: invalid coverage declaration: {source}")]
    Declaration {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error("{0}")]
    Invalid(String),
}

fn read(path: &Path) -> Result<String, AuditError> {
    fs::read_to_string(path).map_err(|source| AuditError::Read {
        path: path.to_path_buf(),
        source,
    })
}

/// Use a Markdown parser, not a line grep: shell comments in fences are not sections.
fn top_headings(markdown: &str) -> Vec<String> {
    let mut depth = 0usize;
    let mut heading = None::<String>;
    let mut headings = Vec::new();
    for event in Parser::new(markdown) {
        match event {
            Event::Start(tag) => {
                if depth == 0
                    && matches!(
                        tag,
                        Tag::Heading {
                            level: HeadingLevel::H1,
                            ..
                        }
                    )
                {
                    heading = Some(String::new());
                }
                depth += 1;
            }
            Event::End(_) => {
                depth -= 1;
                if depth == 0
                    && let Some(text) = heading.take()
                {
                    headings.push(normalize(&text));
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut heading {
                    heading.push_str(&text);
                }
            }
            _ => {}
        }
    }
    headings
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn fingerprint(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Match visible content, never metadata, comments, or Markdown punctuation.
fn visible_body(raw: &str) -> Result<String, &'static str> {
    let normalized = raw.replace("\r\n", "\n");
    let after_open = normalized
        .strip_prefix("---\n")
        .ok_or("missing fragment frontmatter")?;
    let (_, body) = after_open
        .split_once("\n---\n")
        .ok_or("unclosed fragment frontmatter")?;
    let mut visible = String::new();
    for event in Parser::new(body) {
        match event {
            Event::Text(text) | Event::Code(text) => visible.push_str(&text),
            Event::SoftBreak | Event::HardBreak | Event::Rule => visible.push('\n'),
            Event::End(
                TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::CodeBlock | TagEnd::Item,
            ) => {
                visible.push('\n');
            }
            _ => {}
        }
    }
    Ok(normalize(&visible))
}

/// Evaluate a declaration against already loaded inputs; deterministic, no filesystem effects.
pub fn evaluate(coverage: &Coverage, baseline: &str, selected: &HashMap<String, String>) -> Report {
    let headings = top_headings(baseline);
    let mut report = Report {
        sections: headings.len(),
        ..Report::default()
    };
    let actual_digest = fingerprint(baseline);
    if actual_digest != coverage.sha256 {
        report.findings.push(format!(
            "{}: baseline changed (SHA-256 {actual_digest}); review coverage before updating the pinned digest",
            coverage.baseline
        ));
    }
    if headings.is_empty() {
        report
            .findings
            .push("baseline has no top-level sections".into());
    }
    let mut heading_set = HashSet::new();
    for heading in &headings {
        if !heading_set.insert(heading.as_str()) {
            report
                .findings
                .push(format!("baseline has duplicate section {heading:?}"));
        }
    }
    let mut declared = HashSet::new();
    // Cache both successes and parse failures; a fragment can serve multiple sections.
    let visible: HashMap<_, _> = selected
        .iter()
        .map(|(path, raw)| (path.as_str(), visible_body(raw)))
        .collect();
    for section in &coverage.sections {
        let heading = &section.heading;
        if !declared.insert(heading.as_str()) {
            report
                .findings
                .push(format!("duplicate coverage entry for {heading:?}"));
        }
        if !heading_set.contains(heading.as_str()) {
            report
                .findings
                .push(format!("unknown baseline section {heading:?}"));
        }
        if section.note.trim().is_empty() || section.checks.is_empty() {
            report.findings.push(format!(
                "{heading}: needs a review note and at least one fragment check"
            ));
        }
        for check in &section.checks {
            if check.contains.is_empty() || check.contains.iter().any(|s| s.trim().is_empty()) {
                report.findings.push(format!(
                    "{heading}: {} needs nonempty instructional excerpts",
                    check.fragment
                ));
            }
            let body = match visible.get(check.fragment.as_str()) {
                None => {
                    report.findings.push(format!(
                        "{heading}: required fragment {} is not selected in the manifest",
                        check.fragment
                    ));
                    continue;
                }
                Some(Err(message)) => {
                    report
                        .findings
                        .push(format!("{heading}: {}: {message}", check.fragment));
                    continue;
                }
                Some(Ok(body)) => body,
            };
            for excerpt in &check.contains {
                report.assertions += 1;
                if !body.contains(&normalize(excerpt)) {
                    report.findings.push(format!(
                        "{heading}: {} is missing required instruction {excerpt:?}",
                        check.fragment
                    ));
                }
            }
        }
    }
    for heading in headings {
        if !declared.contains(heading.as_str()) {
            report
                .findings
                .push(format!("unaccounted baseline section {heading:?}"));
        }
    }
    report
}

/// Manifest/coverage arguments resolve from cwd; declared inputs from the manifest's directory.
pub fn audit(manifest_path: &Path, coverage_path: &Path, cwd: &Path) -> Result<Report, AuditError> {
    let manifest_file = cwd.join(manifest_path);
    let coverage_file = cwd.join(coverage_path);
    let manifest = parse_manifest(&read(&manifest_file)?, &manifest_file.to_string_lossy())?;
    let plan = plan_assembly(
        &manifest,
        &manifest_file.to_string_lossy(),
        &cwd.to_string_lossy(),
        None,
    )?;
    let coverage: Coverage =
        serde_json::from_str(&read(&coverage_file)?).map_err(|source| AuditError::Declaration {
            path: coverage_file,
            source,
        })?;
    // Reuse manifest path validation for the baseline; no second path grammar.
    validate_fragment_path(&coverage.baseline).map_err(|message| {
        AuditError::Invalid(format!(
            "invalid coverage baseline {:?}: {message}",
            coverage.baseline
        ))
    })?;
    if coverage.sha256.len() != 64
        || !coverage
            .sha256
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err(AuditError::Invalid(
            "coverage sha256 must be 64 lowercase hex digits".into(),
        ));
    }
    let root = Path::new(&plan.manifest_path)
        .parent()
        .expect("absolute manifest has parent");
    let baseline = read(&root.join(&coverage.baseline))?;
    let mut selected = HashMap::new();
    for fragment in plan.fragments {
        selected.insert(fragment.path, read(Path::new(&fragment.absolute))?);
    }
    Ok(evaluate(&coverage, &baseline, &selected))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASELINE: &str = "# beads\n\nUse readable IDs.\n\n```sh\n# not a section\nbd show <id>\n```\n\n> # not a section either\n\n- item\n  # nested\n";
    const FRAGMENT: &str =
        "---\nid: beads\n---\n# Beads\n\nAlways use **readable IDs** and\nexplicit `--id`.\n";

    fn declaration() -> Coverage {
        Coverage {
            baseline: "AGENTS.old.md".into(),
            sha256: fingerprint(BASELINE),
            sections: vec![Section {
                heading: "beads".into(),
                note: "Restore naming rules inline".into(),
                checks: vec![Requirement {
                    fragment: "beads/GLOBAL.md".into(),
                    contains: vec!["Always use readable IDs and explicit --id.".into()],
                }],
            }],
        }
    }

    #[test]
    fn recognizes_real_sections_and_wrapped_formatted_instructions() {
        let selected = HashMap::from([("beads/GLOBAL.md".into(), FRAGMENT.into())]);
        let result = evaluate(&declaration(), BASELINE, &selected);
        assert_eq!(result.sections, 1);
        assert_eq!(result.assertions, 1);
        assert!(result.findings.is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn catches_missing_selection_even_if_the_file_still_exists() {
        let result = evaluate(&declaration(), BASELINE, &HashMap::new());
        assert!(result.findings.iter().any(|f| f.contains("not selected")));
    }

    #[test]
    fn metadata_and_comments_cannot_hide_loss_of_the_actual_instruction() {
        let hidden = "---\nnote: Always use readable IDs and explicit --id.\n---\n# Beads\n\n<!-- Always use readable IDs and explicit --id. -->\nUse tickets.\n";
        let selected = HashMap::from([("beads/GLOBAL.md".into(), hidden.into())]);
        let result = evaluate(&declaration(), BASELINE, &selected);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.contains("missing required instruction"))
        );
    }

    #[test]
    fn changed_baseline_and_unaccounted_sections_both_report() {
        let result = evaluate(
            &declaration(),
            &format!("{BASELINE}\n# New obligation\n\nDo this.\n"),
            &HashMap::new(),
        );
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.contains("baseline changed"))
        );
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.contains("unaccounted baseline section \"New obligation\""))
        );
    }

    #[test]
    fn rejects_duplicate_unknown_and_vacuous_entries() {
        let mut coverage = declaration();
        coverage.sections.push(Section {
            heading: "beads".into(),
            note: "".into(),
            checks: vec![],
        });
        coverage.sections.push(Section {
            heading: "ghost".into(),
            note: "oops".into(),
            checks: vec![Requirement {
                fragment: "beads/GLOBAL.md".into(),
                contains: vec![],
            }],
        });
        let result = evaluate(&coverage, BASELINE, &HashMap::new());
        for expected in [
            "duplicate coverage",
            "unknown baseline",
            "review note",
            "nonempty instructional",
        ] {
            assert!(
                result.findings.iter().any(|f| f.contains(expected)),
                "{:?}",
                result.findings
            );
        }
    }

    #[test]
    fn resolves_bundle_inputs_from_manifest_and_does_not_write_output() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("bundle");
        fs::create_dir_all(root.join("beads")).unwrap();
        fs::write(root.join("AGENTS.old.md"), BASELINE).unwrap();
        fs::write(root.join("beads/GLOBAL.md"), FRAGMENT).unwrap();
        fs::write(
            root.join("manifest.json"),
            r#"{"fragments":["beads/GLOBAL.md"],"output":"AGENTS.md"}"#,
        )
        .unwrap();
        let raw = serde_json::json!({
            "baseline": "AGENTS.old.md", "sha256": declaration().sha256,
            "sections": [{"heading":"beads", "note":"inline", "checks":[{"fragment":"beads/GLOBAL.md","contains":["explicit --id."]}]}]
        });
        fs::write(directory.path().join("coverage.json"), raw.to_string()).unwrap();
        let result = audit(
            Path::new("bundle/manifest.json"),
            Path::new("coverage.json"),
            directory.path(),
        )
        .unwrap();
        assert!(result.findings.is_empty(), "{:?}", result.findings);
        assert!(!root.join("AGENTS.md").exists());
        fs::remove_file(root.join("beads/GLOBAL.md")).unwrap();
        assert!(matches!(
            audit(
                Path::new("bundle/manifest.json"),
                Path::new("coverage.json"),
                directory.path()
            ),
            Err(AuditError::Read { .. })
        ));
    }
}
