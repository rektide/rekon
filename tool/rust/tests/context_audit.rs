//! Exercise the original failure through both real CLIs: fresh does not mean complete.
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use sha2::{Digest, Sha256};

fn run(binary: &str, args: &[&str], cwd: &Path) -> Output {
    Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn successful(output: Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn fresh_assembly_still_fails_when_beads_is_omitted_or_its_instruction_is_lost() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let binary = env!("CARGO_BIN_EXE_rekon");
    let baseline = "# beads\n\nAlways supply explicit --id.\n";
    let hash: String = Sha256::digest(baseline)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    fs::write(root.join("legacy.md"), baseline).unwrap();
    fs::write(
        root.join("common.md"),
        "---\nid: common\norder: 1\nsource: /common.md\nstatus: draft\n---\n# Common\n\nCommon guidance.\n",
    )
    .unwrap();
    let beads = "---\nid: beads\norder: 2\nsource: /beads.md\nstatus: draft\n---\n# Beads\n\nAlways supply explicit `--id`.\n";
    fs::write(root.join("beads.md"), beads).unwrap();
    fs::write(
        root.join("coverage.json"),
        serde_json::json!({
            "baseline": "legacy.md", "sha256": hash,
            "sections": [{"heading": "beads", "note": "Keep IDs ambient", "checks": [{"fragment":"beads.md", "contains":["Always supply explicit --id."]}]}]
        })
        .to_string(),
    )
    .unwrap();
    let full_manifest = r#"{"fragments":["common.md","beads.md"],"output":"AGENTS.md"}"#;
    fs::write(root.join("agents.manifest.json"), full_manifest).unwrap();
    let audit = ["audit-context", "--coverage", "coverage.json"];
    successful(run(binary, &["combine"], root));
    successful(run(binary, &audit, root));

    // Audit and combine must select the same expanded and excluded paths.
    fs::write(
        root.join("agents.manifest.json"),
        r#"{"fragments":["*.md"],"exclude":["legacy.md","AGENTS.md"],"output":"AGENTS.md"}"#,
    )
    .unwrap();
    successful(run(binary, &audit, root));

    // The unselected file still exists, exactly as it did after the original cutover.
    fs::write(
        root.join("agents.manifest.json"),
        r#"{"fragments":["common.md"],"output":"AGENTS.md"}"#,
    )
    .unwrap();
    successful(run(binary, &["combine"], root));
    successful(run(binary, &["combine", "--check"], root));
    let before = fs::read(root.join("AGENTS.md")).unwrap();
    let missing = run(binary, &audit, root);
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("beads.md is not selected"));
    assert_eq!(fs::read(root.join("AGENTS.md")).unwrap(), before);

    // Selection is not sufficient: removing its operational instruction must fail too.
    fs::write(root.join("agents.manifest.json"), full_manifest).unwrap();
    fs::write(
        root.join("beads.md"),
        beads.replace("Always supply explicit `--id`.", "Use tickets."),
    )
    .unwrap();
    successful(run(binary, &["combine"], root));
    successful(run(binary, &["combine", "--check"], root));
    let missing = run(binary, &audit, root);
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("missing required instruction"));
}

#[test]
fn malformed_declaration_and_missing_inputs_fail_without_touching_files() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    let binary = env!("CARGO_BIN_EXE_rekon");
    fs::write(
        root.join("agents.manifest.json"),
        r#"{"fragments":["beads.md"],"output":"AGENTS.md"}"#,
    )
    .unwrap();
    fs::write(root.join("coverage.json"), "not JSON").unwrap();
    let result = run(
        binary,
        &["audit-context", "--coverage", "coverage.json"],
        root,
    );
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains("invalid coverage declaration"));
    assert!(!root.join("AGENTS.md").exists());
    let result = run(
        binary,
        &["audit-context", "--coverage", "missing.json"],
        root,
    );
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains("missing.json"));
}
