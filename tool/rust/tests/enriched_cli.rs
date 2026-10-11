//! Cross-engine proof: compose whole documents, navigate them, recover metadata.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use serde_json::{Value, json};

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rekon"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn success(output: Output) -> Output {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn contains_value(tree: &Value, value: &Value) -> bool {
    tree == value
        || match tree {
            Value::Object(map) => map.values().any(|child| contains_value(child, value)),
            Value::Array(items) => items.iter().any(|child| contains_value(child, value)),
            _ => false,
        }
}

#[test]
fn combined_semantic_content_is_mdx_and_its_metadata_is_recoverable() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    fs::write(
        root.join("agents.manifest.json"),
        r#"{"fragments":["code.md","vocabulary.md"],"output":"AGENTS.md"}"#,
    )
    .unwrap();
    fs::write(root.join("vocabulary.md"), "---\nid: vocabulary\norder: 1\nsource: /vocabulary.md\nstatus: draft\n---\n# Vocabulary\n\n## Terms\n\n<dl>\n<dt><dfn id=\"checkpoint\">Checkpoint</dfn></dt>\n<dd>A preserved state for comparison.</dd>\n</dl>\n").unwrap();
    let metadata = json!({
        "id": "code", "order": 2, "source": "/code.md", "status": "draft",
        "type": "Practice", "title": "Code", "custom": {
            "enabled": true, "weights": [1, 2.5, null],
            "example": "<nav id='rekon-toc'><section id='fake'>metadata, not structure</section></nav>"
        },
        "sources": [{"resource": "urn:example:source", "title": "Known grounds"}]
    });
    let yaml = serde_yaml_ng::to_string(&metadata).unwrap();
    fs::write(root.join("code.md"), format!("---\n{yaml}---\n# Code\n\nKeep all the prose.\n\n## Terms\n\n<Okf data={{{{\"type\":\"Reference\",\"title\":\"Local terms\"}}}} />\n\nUse a [checkpoint](/vocabulary.md#checkpoint).\n\n<!-- <Okf data={{{{\"type\":\"Fake\"}}}} /> -->\n\n```md\n# This is code, not a section\n```\n")).unwrap();

    success(run(root, &["combine"]));
    let combined = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    assert!(combined.contains("<Okf data="));
    assert!(combined.contains("<dfn id=\"checkpoint\">Checkpoint</dfn>"));
    assert!(combined.contains("Keep all the prose."));
    assert!(combined.contains("```md\n# This is code, not a section\n```"));
    assert!(combined.find("## Vocabulary").unwrap() < combined.find("## Code").unwrap());

    let mut options = markdown::ParseOptions::mdx();
    options.constructs.gfm_table = true;
    options.constructs.gfm_footnote_definition = true;
    options.constructs.gfm_label_start_footnote = true;
    markdown::to_mdast(&combined, &options).expect("assembled output parses as MDX");

    let extraction = success(run(
        root,
        &["okf", "extract", "AGENTS.md", "--scope", "fragment-code"],
    ));
    let extracted: Value = serde_json::from_slice(&extraction.stdout).unwrap();
    assert!(
        contains_value(&extracted, &metadata),
        "complete frontmatter must survive: {extracted}"
    );
    let vocabulary = success(run(
        root,
        &[
            "okf",
            "extract",
            "AGENTS.md",
            "--scope",
            "fragment-vocabulary",
        ],
    ));
    let vocabulary: Value = serde_json::from_slice(&vocabulary.stdout).unwrap();
    assert!(contains_value(
        &vocabulary,
        &json!({
            "id": "vocabulary", "order": 1, "source": "/vocabulary.md", "status": "draft"
        })
    ));
    assert!(!contains_value(&vocabulary, &metadata));
    let local = success(run(
        root,
        &[
            "okf",
            "extract",
            "AGENTS.md",
            "--scope",
            "fragment-code--terms",
        ],
    ));
    let local: Value = serde_json::from_slice(&local.stdout).unwrap();
    assert!(contains_value(
        &local,
        &json!({"type":"Reference", "title":"Local terms"})
    ));
    let all = success(run(root, &["okf", "extract", "AGENTS.md"]));
    let all: Value = serde_json::from_slice(&all.stdout).unwrap();
    assert_eq!(
        all.as_array().unwrap().len(),
        3,
        "comments must not create metadata records"
    );
    success(run(root, &["combine", "--check"]));
    success(run(root, &["toc", "AGENTS.md", "--line-ranges", "--check"]));
    assert_eq!(
        fs::read_to_string(root.join("AGENTS.md")).unwrap(),
        combined
    );

    fs::write(root.join("AGENTS.md"), format!("{combined}\nDrift\n")).unwrap();
    let drift = run(root, &["combine", "--check"]);
    assert_eq!(drift.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&drift.stderr).contains("stale"));
    assert!(
        fs::read_to_string(root.join("AGENTS.md"))
            .unwrap()
            .ends_with("Drift\n")
    );
}

#[test]
fn extraction_is_read_only_and_dynamic_metadata_is_rejected() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let source =
        "<section id=\"unsafe\">\n\n<Okf data={process.exit(0)} />\n\n# Example\n\n</section>\n";
    fs::write(root.join("dynamic.mdx"), source).unwrap();
    let result = run(root, &["okf", "extract", "dynamic.mdx"]);
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(
        fs::read_to_string(root.join("dynamic.mdx")).unwrap(),
        source
    );
}

#[test]
fn semantic_vocabulary_remains_parseable_as_static_mdx() {
    let source = include_str!("../../../CONTEXT.md");
    let parsed = rekon_markup::parse_document(source).unwrap();
    assert_eq!(parsed.headings[0].text, "Rekon working vocabulary");
    assert_eq!(parsed.metadata.len(), 1);
    assert!(source.contains("<dfn id=\"home\">Home</dfn>"));
}

#[test]
fn toc_prepares_every_input_before_publishing() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let source = "# Guide\n\n## Work\n\nKeep this intact on failure.\n";
    fs::write(root.join("guide.md"), source).unwrap();
    let result = run(root, &["toc", "guide.md", "missing.md"]);
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(fs::read_to_string(root.join("guide.md")).unwrap(), source);
    assert_eq!(fs::read_dir(root).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn toc_updates_symlink_target_and_preserves_permissions() {
    use std::os::unix::fs::{PermissionsExt, symlink};

    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let target = root.join("guide.md");
    fs::write(&target, "# Guide\n\n## Work\n\nKeep this prose.\n").unwrap();
    fs::set_permissions(&target, fs::Permissions::from_mode(0o444)).unwrap();
    symlink("guide.md", root.join("linked.md")).unwrap();
    success(run(root, &["toc", "linked.md", "--line-ranges"]));
    assert!(
        fs::symlink_metadata(root.join("linked.md"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::metadata(&target).unwrap().permissions().mode() & 0o777,
        0o444
    );
    assert!(
        fs::read_to_string(&target)
            .unwrap()
            .contains("<nav id=\"rekon-toc\"")
    );
    success(run(root, &["toc", "linked.md", "--line-ranges", "--check"]));
    assert_eq!(fs::read_dir(root).unwrap().count(), 2);
}
