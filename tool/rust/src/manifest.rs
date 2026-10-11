//! Manifest declaration parsing and lexical assembly planning.
//!
//! Port of `tool/assembly/manifest.ts` (the Effect implementation), which is
//! the production spelling of the [manifest contract]. Planning performs no
//! I/O: the manifest declares fragment paths resolved from its own directory
//! and an output that may deliberately target a parent, while a CLI override
//! resolves from the invocation cwd. Directory names *starting* with `..`
//! (for example `..notes/`) are fine; actual `.`/`..` segments are not.
//!
//! [manifest contract]: /constitution/agents-assembly.md

use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::nodepath;

/// Production selection since October 7; coverage is checked separately.
pub const DEFAULT_MANIFEST: &str = "agents.manifest.json";

/// Provenance-comment safety shared by fragment paths and sources.
pub fn is_provenance_text(text: &str) -> bool {
    !text.contains("--") && !text.chars().any(is_control_character)
}

/// The control-character class rejected in provenance values (`[\u0000-\u001f\u007f]`).
pub fn is_control_character(character: char) -> bool {
    matches!(character, '\u{0000}'..='\u{001f}' | '\u{007f}')
}

/// Validated relative path rules shared by fragment paths and the output.
fn validate_relative_path(path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("must be a non-empty path string".to_string());
    }
    if nodepath::is_absolute(path) {
        return Err("must be a relative path".to_string());
    }
    if nodepath::normalize(path) != path {
        return Err("must be a normalized path without redundant separators".to_string());
    }
    Ok(())
}

/// Validate one declared fragment path (relative, normalized, contained, provenance-safe).
pub(crate) fn validate_fragment_path(path: &str) -> Result<(), String> {
    validate_relative_path(path)?;
    if !is_provenance_text(path) {
        return Err(
            "must not contain \"--\" or control characters in a provenance comment".to_string(),
        );
    }
    if path
        .split('/')
        .any(|segment| segment == "." || segment == "..")
    {
        return Err(
            "fragment paths must not contain '.' or '..' segments or escape the manifest directory"
                .to_string(),
        );
    }
    Ok(())
}

/// The explicit declaration; unknown auxiliary keys do not change selection.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Manifest {
    pub fragments: Vec<String>,
    pub output: String,
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("{path}: invalid manifest: {message}")]
    Validation { path: String, message: String },
    #[error("{message}")]
    DuplicateFragmentPath {
        fragment: String,
        prior: String,
        message: String,
    },
    #[error("{message}")]
    OutputOverwritesInput {
        output: String,
        input: String,
        message: String,
    },
}

/// Parse the manifest text; `path` names the manifest for error context (as given, not absolute).
pub fn parse_manifest(raw: &str, path: &str) -> Result<Manifest, ManifestError> {
    let fail = |message: String| ManifestError::Validation {
        path: path.to_string(),
        message,
    };
    let parsed: Manifest =
        serde_json::from_str(raw).map_err(|cause| fail(format!("json: {cause}")))?;
    if parsed.fragments.is_empty() {
        return Err(fail(
            "fragments must be a non-empty array of path strings".to_string(),
        ));
    }
    for fragment in &parsed.fragments {
        validate_fragment_path(fragment)
            .map_err(|message| fail(format!("fragments[{fragment:?}]: {message}")))?;
    }
    validate_relative_path(&parsed.output).map_err(|message| fail(format!("output: {message}")))?;
    Ok(parsed)
}

/// Lexical planning result: manifest-relative inputs and a resolved output.
#[derive(Debug, PartialEq)]
pub struct AssemblyPlan {
    pub manifest_path: String,
    pub fragments: Vec<PlanFragment>,
    pub output: String,
}

#[derive(Debug, PartialEq)]
pub struct PlanFragment {
    pub path: String,
    pub absolute: String,
}

/// Lexical path planning: manifest-relative selection, invocation-relative override.
pub fn plan_assembly(
    manifest: &Manifest,
    manifest_path: &str,
    cwd: &str,
    output_override: Option<&str>,
) -> Result<AssemblyPlan, ManifestError> {
    let absolute_manifest = nodepath::resolve(cwd, manifest_path);
    let directory = nodepath::dirname(&absolute_manifest);
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut fragments = Vec::with_capacity(manifest.fragments.len());
    for path in &manifest.fragments {
        let absolute = nodepath::resolve(&directory, path);
        if let Some(prior) = seen.get(&absolute) {
            return Err(ManifestError::DuplicateFragmentPath {
                fragment: path.clone(),
                prior: prior.clone(),
                message: format!(
                    "{manifest_path}: manifest lists the same fragment twice: \"{prior}\" and \"{path}\" both resolve to {absolute}"
                ),
            });
        }
        seen.insert(absolute.clone(), path.clone());
        fragments.push(PlanFragment {
            path: path.clone(),
            absolute,
        });
    }
    let output = match output_override {
        Some(override_path) => nodepath::resolve(cwd, override_path),
        None => nodepath::resolve(&directory, &manifest.output),
    };
    let input = if output == absolute_manifest {
        Some(absolute_manifest.clone())
    } else {
        seen.get(&output).cloned()
    };
    if let Some(input) = input {
        let message = if output == absolute_manifest {
            format!("assembly output must not overwrite its manifest: {absolute_manifest}")
        } else {
            format!("assembly output must not overwrite source fragment: {input}")
        };
        return Err(ManifestError::OutputOverwritesInput {
            output,
            input,
            message,
        });
    }
    Ok(AssemblyPlan {
        manifest_path: absolute_manifest,
        fragments,
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECLARATION: &str = r#"{"fragments": ["rules/GLOBAL.md"], "output": "AGENTS.md"}"#;

    #[test]
    fn preserves_declaration_order_and_spellings_without_discovering_extra_files() {
        let raw = r#"{"fragments": ["second.md", "first.md"], "output": "AGENTS.md", "unrelated": "ignored"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        assert_eq!(
            manifest,
            Manifest {
                fragments: vec!["second.md".into(), "first.md".into()],
                output: "AGENTS.md".into()
            }
        );
    }

    #[test]
    fn rejects_malformed_manifest_data() {
        let cases = [
            "{ invalid json",
            "null",
            "[]",
            r#"{"fragments": [], "output": "AGENTS.md"}"#,
            r#"{"fragments": [42], "output": "AGENTS.md"}"#,
            r#"{"fragments": "rules/GLOBAL.md", "output": "AGENTS.md"}"#,
            r#"{"fragments": ["rules/GLOBAL.md"]}"#,
            r#"{"fragments": ["rules/GLOBAL.md"], "output": ""}"#,
            r#"{"fragments": ["rules/GLOBAL.md"], "output": "/absolute.md"}"#,
            r#"{"fragments": ["rules/GLOBAL.md"], "output": "./out.md"}"#,
            r#"{"fragments": ["rules/GLOBAL.md"], "output": "out//AGENTS.md"}"#,
        ];
        for raw in cases {
            let error = parse_manifest(raw, "broken.json").expect_err("manifest must be rejected");
            let ManifestError::Validation { path, .. } = &error else {
                panic!("expected a validation error for {raw}: got {error:?}");
            };
            assert_eq!(path, "broken.json");
        }
    }

    #[test]
    fn rejects_unsafe_declaration_paths() {
        let cases = [
            "",
            "/etc/hostname",
            "../outside.md",
            "frag/../inside.md",
            "./inside.md",
            "frag//inside.md",
            "evil-->name.md",
            "control\nname.md",
        ];
        for path in cases {
            let raw = format!(
                r#"{{"fragments": ["{}"], "output": "AGENTS.md"}}"#,
                path.escape_default()
            );
            assert!(
                parse_manifest(&raw, "bad.json").is_err(),
                "fragment path {path:?} must be rejected"
            );
        }
    }

    #[test]
    fn keeps_inputs_manifest_relative_deliberate_parent_output_and_override_cwd_relative() {
        let raw =
            r#"{"fragments": ["local.md", "nested/GLOBAL.md"], "output": "../output/AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "bundle/manifest.json").unwrap();
        let plain = plan_assembly(&manifest, "bundle/manifest.json", "/workspace", None).unwrap();
        assert_eq!(
            plain,
            AssemblyPlan {
                manifest_path: "/workspace/bundle/manifest.json".into(),
                fragments: vec![
                    PlanFragment {
                        path: "local.md".into(),
                        absolute: "/workspace/bundle/local.md".into()
                    },
                    PlanFragment {
                        path: "nested/GLOBAL.md".into(),
                        absolute: "/workspace/bundle/nested/GLOBAL.md".into()
                    },
                ],
                output: "/workspace/output/AGENTS.md".into(),
            }
        );
        let override_plan = plan_assembly(
            &manifest,
            "bundle/manifest.json",
            "/workspace",
            Some("preview.md"),
        )
        .unwrap();
        assert_eq!(override_plan.output, "/workspace/preview.md");
        assert_eq!(override_plan.fragments, plain.fragments);
        let absolute_override = plan_assembly(
            &manifest,
            "bundle/manifest.json",
            "/workspace",
            Some("/preview/AGENTS.md"),
        )
        .unwrap();
        assert_eq!(absolute_override.output, "/preview/AGENTS.md");
    }

    #[test]
    fn directory_names_beginning_with_two_dots_are_not_traversal_segments() {
        let raw = r#"{"fragments": ["..notes/GLOBAL.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        let plan = plan_assembly(&manifest, "manifest.json", "/workspace", None).unwrap();
        assert_eq!(plan.fragments[0].absolute, "/workspace/..notes/GLOBAL.md");
    }

    #[test]
    fn rejects_duplicate_resolved_fragment_paths() {
        let raw = r#"{"fragments": ["rules/GLOBAL.md", "rules/GLOBAL.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        let error = plan_assembly(&manifest, "manifest.json", "/workspace", None).unwrap_err();
        assert!(matches!(
            &error,
            ManifestError::DuplicateFragmentPath { fragment, prior, .. }
                if fragment == "rules/GLOBAL.md" && prior == "rules/GLOBAL.md"
        ));
    }

    #[test]
    fn rejects_declared_or_override_output_overwriting_manifest_or_fragment() {
        for output in ["manifest.json", "rules/GLOBAL.md"] {
            let raw = format!(r#"{{"fragments": ["rules/GLOBAL.md"], "output": "{output}"}}"#);
            let manifest = parse_manifest(&raw, "manifest.json").unwrap();
            assert!(matches!(
                plan_assembly(&manifest, "manifest.json", "/workspace", None).unwrap_err(),
                ManifestError::OutputOverwritesInput { .. }
            ));
            let ordinary = parse_manifest(DECLARATION, "manifest.json").unwrap();
            assert!(matches!(
                plan_assembly(&ordinary, "manifest.json", "/workspace", Some(output)).unwrap_err(),
                ManifestError::OutputOverwritesInput { .. }
            ));
        }
    }
}
