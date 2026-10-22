//! Public command: portable roots, opt-in knowledge rules, and read-only failures.
use std::{fs, process::Command};

#[test]
fn checks_another_repository_and_reports_failure_without_writes() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    fs::create_dir(root.join(".jj")).unwrap();
    fs::create_dir(root.join("docs")).unwrap();
    fs::write(root.join("README.md"), "# Home\n").unwrap();
    let document = root.join("docs/note.md");
    let raw = "---\ntype: Guide\ntitle: Note\ndescription: Example\nresource: /docs/note.md\nstatus: draft\nsources:\n  - resource: opencode://ses_example?source=local\n  - resource: /README.md#home\n---\n# Note\n\n[home](/README.md#home)\n";
    fs::write(&document, raw).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rekon"))
        .args(["check", "--okf"])
        .arg(&document)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(fs::read_to_string(&document).unwrap(), raw);

    let broken = "# Note\n\n[missing](/missing.md)\n";
    fs::write(&document, broken).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rekon"))
        .args(["check", "--root"])
        .arg(root)
        .arg(&document)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing target /missing.md"));
    assert_eq!(fs::read_to_string(&document).unwrap(), broken);
    assert!(!root.join("missing.md").exists());
}
