//! Manifest declaration parsing and lexical assembly planning.
//!
//! Port of `tool/assembly/manifest.ts` (the Effect implementation), which is
//! the production spelling of the [manifest contract]. Planning performs no
//! writes: the manifest declares fragment paths resolved from its own directory
//! and an output that may deliberately target a parent, while a CLI override
//! resolves from the invocation cwd. Directory names *starting* with `..`
//! (for example `..notes/`) are fine; actual `.`/`..` segments are not.
//!
//! **Globbing:** a declared entry containing a pattern metacharacter (`*`,
//! `?`, `[`) is a directive the walk expands: matching files under the
//! manifest's directory are found and selected in deterministic sorted order.
//! The pattern is itself the declaration — expansion is not unconstrained
//! discovery — and every match must independently satisfy the fragment path
//! rules. [`expand_globs`] performs this read-only walk and returns plain
//! paths, so [`plan_assembly`] stays a pure planner over explicit entries.
//!
//! [manifest contract]: /constitution/agents-assembly.md

use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::nodepath;

/// Production selection since October 7; coverage is checked separately.
pub const DEFAULT_MANIFEST: &str = "agents.manifest.json";
pub const DEFAULT_FRAGMENT_GLOB: &str = "**/GLOBAL.md";

fn default_output() -> String {
    "AGENTS.md".to_string()
}

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
        return Err("must not contain \"--\" or control characters in a provenance comment".to_string());
    }
    if path.split('/').any(|segment| segment == "." || segment == "..") {
        return Err("fragment paths must not contain '.' or '..' segments or escape the manifest directory".to_string());
    }
    Ok(())
}

/// A declared entry containing a pattern metacharacter is a glob directive.
pub fn is_pattern(entry: &str) -> bool {
    entry.contains('*') || entry.contains('?') || entry.contains('[')
}

/// Expand glob directives into plain fragment paths, deterministically sorted.
///
/// The walk starts at `directory` (the manifest's own directory) and never
/// leaves it: patterns already forbid `.`/`..` segments, and every match is
/// revalidated against the fragment path rules after relativization. Dotfiles
/// and dot-directories are skipped so `*` stays literal about hidden state.
/// A pattern matching nothing is an error: a stale directive must fail loudly
/// rather than silently assemble less context.
pub fn expand_globs(manifest: &Manifest, directory: &str) -> Result<Manifest, ManifestError> {
    if !manifest.fragments.iter().any(|f| is_pattern(f)) && manifest.exclude.is_empty() {
        return Ok(manifest.clone());
    }
    let options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: true,
        require_literal_leading_dot: true,
    };
    let mut fragments = Vec::with_capacity(manifest.fragments.len());
    for entry in &manifest.fragments {
        if !is_pattern(entry) {
            fragments.push(entry.clone());
            continue;
        }
        let pattern_root = nodepath::resolve(directory, entry);
        let paths = glob::glob_with(&pattern_root, options).map_err(|error| ManifestError::GlobExpansion {
            pattern: entry.clone(),
            message: format!("invalid glob: {error}"),
        })?;
        let mut matches: Vec<String> = Vec::new();
        for candidate in paths {
            let candidate = candidate.map_err(|error| ManifestError::GlobExpansion {
                pattern: entry.clone(),
                message: format!("walk failed: {error}"),
            })?;
            let Ok(metadata) = std::fs::metadata(&candidate) else {
                continue;
            };
            if !metadata.is_file() {
                continue;
            }
            let Ok(relative) = candidate.strip_prefix(std::path::Path::new(directory)) else {
                return Err(ManifestError::GlobExpansion {
                    pattern: entry.clone(),
                    message: format!("match {} escapes the manifest directory {}", candidate.display(), directory),
                });
            };
            // Recursive glob walks can traverse directory aliases. Do not silently
            // select the same owner through an alias (or leave the bundle via one).
            let mut component_path = std::path::PathBuf::from(directory);
            let mut symlinked = false;
            for component in relative.components() {
                component_path.push(component);
                if std::fs::symlink_metadata(&component_path)
                    .map_err(|error| ManifestError::GlobExpansion {
                        pattern: entry.clone(),
                        message: format!("cannot inspect {}: {error}", component_path.display()),
                    })?
                    .file_type()
                    .is_symlink()
                {
                    symlinked = true;
                    break;
                }
            }
            if symlinked {
                continue;
            }
            let relative = relative
                .to_str()
                .ok_or_else(|| ManifestError::GlobExpansion {
                    pattern: entry.clone(),
                    message: format!("match {} is not valid UTF-8", candidate.display()),
                })?
                .replace(std::path::MAIN_SEPARATOR, "/");
            if let Err(reason) = validate_fragment_path(&relative) {
                return Err(ManifestError::GlobExpansion {
                    pattern: entry.clone(),
                    message: format!("match {relative:?} is not a selectable fragment path: {reason}"),
                });
            }
            matches.push(relative);
        }
        if matches.is_empty() {
            return Err(ManifestError::GlobExpansion {
                pattern: entry.clone(),
                message: "glob directive matched no files".to_string(),
            });
        }
        matches.sort();
        fragments.extend(matches);
    }
    for excluded in &manifest.exclude {
        let before = fragments.len();
        fragments.retain(|fragment| fragment != excluded);
        if fragments.len() == before {
            return Err(ManifestError::GlobExpansion {
                pattern: excluded.clone(),
                message: "excluded path was not selected".to_string(),
            });
        }
    }
    Ok(Manifest {
        fragments,
        output: manifest.output.clone(),
        exclude: Vec::new(),
        omit_metadata_fields: manifest.omit_metadata_fields.clone(),
    })
}

/// The explicit declaration; unknown auxiliary keys do not change selection.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Manifest {
    #[serde(default)]
    pub fragments: Vec<String>,
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Top-level frontmatter keys omitted only from emitted Okf records.
    #[serde(default)]
    pub omit_metadata_fields: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("{path}: invalid manifest: {message}")]
    Validation { path: String, message: String },
    #[error("{message}")]
    DuplicateFragmentPath { fragment: String, prior: String, message: String },
    #[error("{message}")]
    OutputOverwritesInput { output: String, input: String, message: String },
    #[error("glob directive {pattern:?}: {message}")]
    GlobExpansion { pattern: String, message: String },
}

/// Parse the manifest text; `path` names the manifest for error context (as given, not absolute).
pub fn parse_manifest(raw: &str, path: &str) -> Result<Manifest, ManifestError> {
    let fail = |message: String| ManifestError::Validation {
        path: path.to_string(),
        message,
    };
    let value: serde_json::Value = serde_json::from_str(raw).map_err(|cause| fail(format!("json: {cause}")))?;
    if !value.is_object() {
        return Err(fail("manifest must be an object".to_string()));
    }
    let mut parsed: Manifest = serde_json::from_value(value).map_err(|cause| fail(format!("json: {cause}")))?;
    if parsed.fragments.is_empty() {
        parsed.fragments.push(DEFAULT_FRAGMENT_GLOB.to_string());
    }
    for fragment in &parsed.fragments {
        validate_fragment_path(fragment).map_err(|message| fail(format!("fragments[{fragment:?}]: {message}")))?;
    }
    let mut exclusions = std::collections::HashSet::new();
    for excluded in &parsed.exclude {
        validate_fragment_path(excluded).map_err(|message| fail(format!("exclude[{excluded:?}]: {message}")))?;
        if is_pattern(excluded) || !exclusions.insert(excluded) {
            return Err(fail(format!("exclude[{excluded:?}]: must be a unique literal path")));
        }
    }
    let mut omitted = std::collections::HashSet::new();
    for field in &parsed.omit_metadata_fields {
        if field.trim().is_empty() || !omitted.insert(field) || ["id", "order", "source", "status"].contains(&field.as_str()) {
            return Err(fail(format!("omit_metadata_fields[{field:?}]: must be a unique non-empty field other than id, order, source, or status")));
        }
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
pub fn plan_assembly(manifest: &Manifest, manifest_path: &str, cwd: &str, output_override: Option<&str>) -> Result<AssemblyPlan, ManifestError> {
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
                message: format!("{manifest_path}: manifest lists the same fragment twice: \"{prior}\" and \"{path}\" both resolve to {absolute}"),
            });
        }
        seen.insert(absolute.clone(), path.clone());
        fragments.push(PlanFragment { path: path.clone(), absolute });
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
        return Err(ManifestError::OutputOverwritesInput { output, input, message });
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
                output: "AGENTS.md".into(),
                exclude: vec![],
                omit_metadata_fields: vec![]
            }
        );
    }

    #[test]
    fn rejects_malformed_manifest_data() {
        let cases = [
            "{ invalid json",
            "null",
            "[]",
            r#"{"fragments": [42], "output": "AGENTS.md"}"#,
            r#"{"fragments": "rules/GLOBAL.md", "output": "AGENTS.md"}"#,
            r#"{"omit_metadata_fields": ["sources", "sources"]}"#,
            r#"{"omit_metadata_fields": ["id"]}"#,
            r#"{"omit_metadata_fields": [""]}"#,
            r#"{"omit_metadata_fields": "sources"}"#,
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
            let raw = format!(r#"{{"fragments": ["{}"], "output": "AGENTS.md"}}"#, path.escape_default());
            assert!(parse_manifest(&raw, "bad.json").is_err(), "fragment path {path:?} must be rejected");
        }
    }

    #[test]
    fn keeps_inputs_manifest_relative_deliberate_parent_output_and_override_cwd_relative() {
        let raw = r#"{"fragments": ["local.md", "nested/GLOBAL.md"], "output": "../output/AGENTS.md"}"#;
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
        let override_plan = plan_assembly(&manifest, "bundle/manifest.json", "/workspace", Some("preview.md")).unwrap();
        assert_eq!(override_plan.output, "/workspace/preview.md");
        assert_eq!(override_plan.fragments, plain.fragments);
        let absolute_override = plan_assembly(&manifest, "bundle/manifest.json", "/workspace", Some("/preview/AGENTS.md")).unwrap();
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

    fn write_bundle(root: &std::path::Path, files: &[(&str, &str)]) {
        for (path, body) in files {
            let full = root.join(path);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, body).unwrap();
        }
    }

    #[test]
    fn glob_directives_expand_deterministically_and_skip_directories_and_dotfiles() {
        let root = tempfile::tempdir().unwrap();
        write_bundle(
            root.path(),
            &[
                ("praxis/zeta/GLOBAL.md", "z"),
                ("praxis/alpha/GLOBAL.md", "a"),
                ("praxis/alpha/notes.md", "not a fragment name, still matched by walk only if pattern says so"),
                ("praxis/alpha/.hidden.md", "h"),
                ("praxis/beta", ""), // a directory named like a file
            ],
        );
        let raw = r#"{"fragments": ["praxis/*/GLOBAL.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "bundle/manifest.json").unwrap();
        let expanded = expand_globs(&manifest, root.path().to_str().unwrap()).unwrap();
        assert_eq!(
            expanded.fragments,
            vec!["praxis/alpha/GLOBAL.md".to_string(), "praxis/zeta/GLOBAL.md".to_string()]
        );
        assert_eq!(expanded.output, "AGENTS.md");
    }

    #[test]
    fn plain_entries_pass_through_and_patterns_expand_in_place() {
        let root = tempfile::tempdir().unwrap();
        write_bundle(root.path(), &[("CONTEXT.md", "c"), ("code/GLOBAL.md", "g"), ("code/rust/GLOBAL.md", "r")]);
        let raw = r#"{"fragments": ["CONTEXT.md", "code/**/GLOBAL.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        let expanded = expand_globs(&manifest, root.path().to_str().unwrap()).unwrap();
        assert_eq!(
            expanded.fragments,
            vec!["CONTEXT.md".to_string(), "code/GLOBAL.md".to_string(), "code/rust/GLOBAL.md".to_string()]
        );
    }

    #[test]
    fn glob_matching_nothing_fails_loudly() {
        let root = tempfile::tempdir().unwrap();
        let raw = r#"{"fragments": ["nope/*.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        let error = expand_globs(&manifest, root.path().to_str().unwrap()).unwrap_err();
        assert!(
            matches!(&error, ManifestError::GlobExpansion { pattern, message } if pattern == "nope/*.md" && message.contains("no files")),
            "{error:?}"
        );
    }

    #[test]
    fn literal_exclusions_remove_selected_files_and_stale_exclusions_fail() {
        let root = tempfile::tempdir().unwrap();
        write_bundle(root.path(), &[("doc/GLOBAL.md", "old"), ("code/GLOBAL.md", "new")]);
        let raw = r#"{"fragments": ["**/GLOBAL.md"], "exclude": ["doc/GLOBAL.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        let expanded = expand_globs(&manifest, root.path().to_str().unwrap()).unwrap();
        assert_eq!(expanded.fragments, vec!["code/GLOBAL.md"]);
        std::fs::remove_file(root.path().join("doc/GLOBAL.md")).unwrap();
        assert!(
            expand_globs(&manifest, root.path().to_str().unwrap())
                .unwrap_err()
                .to_string()
                .contains("excluded path was not selected")
        );
        for excluded in ["../outside", "**/GLOBAL.md"] {
            let raw = format!(r#"{{"fragments":["*.md"],"exclude":["{excluded}"],"output":"AGENTS.md"}}"#);
            assert!(parse_manifest(&raw, "manifest.json").is_err());
        }
    }

    #[cfg(unix)]
    #[test]
    fn glob_skips_symlinked_files_and_directories_but_literals_remain_available() {
        use std::os::unix::fs::symlink;
        let root = tempfile::tempdir().unwrap();
        write_bundle(root.path(), &[("code/js/GLOBAL.md", "js")]);
        symlink("code/js", root.path().join("js")).unwrap();
        symlink("code/js/GLOBAL.md", root.path().join("alias.md")).unwrap();
        let manifest = parse_manifest(r#"{"fragments":["**/GLOBAL.md", "alias.md"],"output":"AGENTS.md"}"#, "manifest.json").unwrap();
        let expanded = expand_globs(&manifest, root.path().to_str().unwrap()).unwrap();
        assert_eq!(expanded.fragments, vec!["code/js/GLOBAL.md", "alias.md"]);
        let only_alias = parse_manifest(r#"{"fragments":["alias.*"],"output":"AGENTS.md"}"#, "manifest.json").unwrap();
        assert!(expand_globs(&only_alias, root.path().to_str().unwrap()).is_err());
    }

    #[test]
    fn expanded_duplicates_are_rejected_by_planning() {
        let root = tempfile::tempdir().unwrap();
        write_bundle(root.path(), &[("code/GLOBAL.md", "g")]);
        let raw = r#"{"fragments": ["code/GLOBAL.md", "code/G*.md"], "output": "AGENTS.md"}"#;
        let manifest = parse_manifest(raw, "manifest.json").unwrap();
        let expanded = expand_globs(&manifest, root.path().to_str().unwrap()).unwrap();
        assert!(matches!(
            plan_assembly(&expanded, "manifest.json", root.path().to_str().unwrap(), None).unwrap_err(),
            ManifestError::DuplicateFragmentPath { .. }
        ));
    }
}
