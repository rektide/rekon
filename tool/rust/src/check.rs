//! Markdown document checks: structure, local destinations, anchors, and the
//! opt-in OKF knowledge frontmatter contract.
//!
//! The checker accepts explicit document paths — it never claims that every
//! Markdown file needs knowledge frontmatter. The default profile checks what
//! is objective for any document: balanced fenced code blocks, trailing
//! whitespace outside code blocks, local link targets (relative,
//! bundle-root-relative `/...`, and `file://` absolute paths), and heading
//! anchors using Rekon's approximate GitHub-style slugs (allocated across
//! every heading in order, plus explicit `id` attributes).
//! `--okf` adds the knowledge-document contract: required frontmatter fields,
//! at least one native `opencode://ses_…` source, and resolvable source
//! resource paths.
//!
//! Non-goals, deliberately: remote URLs are never fetched or otherwise
//! verified, and nothing here judges semantic or architectural claims. The
//! body grammar is the same GFM option set the assembler validates with
//! (tables, strikethrough, task lists), so what is code, text, or markup
//! agrees with `combine`.
//!
//! Origin: a Python scratch prototype (`.test-agent/diff-dirs-doc/check.py`
//! in the kubeless checkout) commissioned for consumer-document checks; this
//! engine is a Rust reworking on Rekon's own parsers, not a port of its
//! regular expressions. See `/tool/check/README.md` for the maintained
//! contract.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use serde_json::Value;

use crate::assembly::fragment::html_attribute_values;
use crate::assembly::parse_options;
use rekon_toc::slugify;

/// Frontmatter fields required of a knowledge document in `--okf` mode.
pub const OKF_REQUIRED_FIELDS: [&str; 6] = ["type", "title", "description", "resource", "status", "sources"];

/// Filesystem surface the checker needs, so the engine is testable in memory.
pub trait DocumentFiles {
    fn exists(&self, path: &Path) -> bool;
    fn read(&self, path: &Path) -> std::io::Result<String>;
}

/// Per-document options; `root` resolves bundle-root-relative `/...`
/// destinations, and `okf` selects the knowledge frontmatter contract.
pub struct CheckOptions<'a> {
    pub root: &'a Path,
    pub okf: bool,
}

/// One reported problem: document display path, optional 1-based line, message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub document: String,
    pub line: Option<usize>,
    pub message: String,
}

impl Failure {
    pub fn at(document: &str, line: Option<usize>, message: impl Into<String>) -> Self {
        Self {
            document: document.to_string(),
            line,
            message: message.into(),
        }
    }

    /// `path: message` or `path:line: message`, matching compiler conventions.
    pub fn render(&self) -> String {
        match self.line {
            Some(line) => format!("{}:{}: {}", self.document, line, self.message),
            None => format!("{}: {}", self.document, self.message),
        }
    }
}

#[derive(Default)]
pub struct DocumentChecks {
    pub failures: Vec<Failure>,
    pub destinations: usize,
    pub sources: usize,
}

impl DocumentChecks {
    fn fail(&mut self, document: &str, line: Option<usize>, message: impl Into<String>) {
        self.failures.push(Failure::at(document, line, message));
    }
}

/// Check one document. `document` is the checked path used to resolve
/// relative destinations; `display` is how failures report it.
pub fn check_document(raw: &str, document: &Path, display: &str, options: &CheckOptions, files: &dyn DocumentFiles) -> DocumentChecks {
    let mut checks = DocumentChecks::default();
    let frontmatter = match rekon_markup::parse_frontmatter(raw) {
        Ok(frontmatter) => frontmatter,
        Err(error) => {
            checks.fail(display, None, format!("invalid frontmatter: {error}"));
            return checks;
        }
    };
    let body = frontmatter.body;
    let first_line = 1 + raw[..frontmatter.body_offset].matches('\n').count();
    scan_style(body, display, first_line, &mut checks);

    let scanned = scan_markdown(body);
    let (anchors, duplicates) = anchors_of_scanned(&scanned);
    for id in duplicates {
        checks.fail(display, None, format!("duplicate explicit id \"{id}\""));
    }

    if options.okf {
        check_okf(frontmatter.metadata.as_ref(), document, display, options.root, files, &anchors, &mut checks);
    }

    let relative = document
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new(""));
    for destination in scanned.destinations.iter().cloned().chain(reference_definition_destinations(body)) {
        checks.destinations += 1;
        if let Some(problem) = destination_problem(&destination, relative, options.root, files, &anchors) {
            checks.fail(display, None, problem);
        }
    }
    checks
}

/// Fenced-code balance and trailing whitespace (outside code blocks, where a
/// Markdown hard break of exactly two trailing spaces is allowed).
fn scan_style(body: &str, display: &str, first_line: usize, checks: &mut DocumentChecks) {
    let mut code_ranges = Vec::new();
    let mut fence = None;
    for (event, range) in Parser::new_ext(body, parse_options()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                code_ranges.push(range.clone());
                if matches!(kind, CodeBlockKind::Fenced(_)) {
                    let opening = &body[range.start..range.end];
                    let marker = opening.chars().next().unwrap_or('`');
                    let length = opening.chars().take_while(|c| *c == marker).count();
                    let content_start = range.start + opening.find('\n').map_or(opening.len(), |n| n + 1);
                    fence = Some((range, marker, length, content_start));
                }
            }
            Event::Text(_) => {
                if let Some((_, _, _, content_end)) = &mut fence {
                    *content_end = range.end;
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((block, marker, length, content_end)) = fence.take() {
                    let closing = body[content_end..block.end].trim_start_matches([' ', '\t', '>']);
                    if !closes_fence(closing.trim_end_matches(['\r', '\n']), marker, length) {
                        checks.fail(
                            display,
                            Some(first_line + body[..block.start].matches('\n').count()),
                            "unclosed fenced code block",
                        );
                    }
                }
            }
            _ => {}
        }
    }
    let mut offset = 0;
    for (index, raw_line) in body.split('\n').enumerate() {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let number = first_line + index;
        if !code_ranges.iter().any(|range| range.start < offset + raw_line.len() && range.end > offset) {
            let trimmed = line.trim_end_matches([' ', '\t']);
            let trailing = line.len() - trimmed.len();
            if trailing > 0 && !(trailing == 2 && line.ends_with("  ")) {
                checks.fail(display, Some(number), "trailing whitespace");
            }
        }
        offset += raw_line.len() + 1;
    }
}

fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

fn closes_fence(line: &str, fence_char: char, fence_len: usize) -> bool {
    let indent = leading_spaces(line);
    if indent > 3 {
        return false;
    }
    let body = &line[indent..];
    let run = body.chars().take_while(|&c| c == fence_char).count();
    run >= fence_len && body[run..].trim().is_empty()
}

/// The facts anchor and destination checking need from one body.
struct Scanned {
    /// Heading texts in document order, inline code included.
    headings: Vec<String>,
    /// Explicit `id` attribute values in document order.
    ids: Vec<String>,
    /// Link, image, and autolink destinations.
    destinations: Vec<String>,
}

fn scan_markdown(body: &str) -> Scanned {
    let mut scanned = Scanned {
        headings: Vec::new(),
        ids: Vec::new(),
        destinations: Vec::new(),
    };
    let mut heading_text: Option<String> = None;
    for event in Parser::new_ext(body, parse_options()) {
        match event {
            Event::Start(Tag::Heading { .. }) => heading_text = Some(String::new()),
            Event::End(TagEnd::Heading(_)) => {
                if let Some(text) = heading_text.take() {
                    scanned.headings.push(text);
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(buffer) = heading_text.as_mut() {
                    buffer.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some(buffer) = heading_text.as_mut() {
                    buffer.push(' ');
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) | Event::Start(Tag::Image { dest_url, .. }) => {
                scanned.destinations.push(dest_url.into_string());
            }
            Event::Html(chunk) | Event::InlineHtml(chunk) => {
                if chunk.trim_start().starts_with("<!--") {
                    continue;
                }
                for attribute in ["href", "src"] {
                    for value in html_attribute_values(&chunk, attribute) {
                        scanned.destinations.push(value.to_string());
                    }
                }
                for id in html_attribute_values(&chunk, "id") {
                    scanned.ids.push(id.to_string());
                }
            }
            _ => {}
        }
    }
    scanned
}

/// Read the parser's definitions, including unused ones. Code samples are
/// excluded and multiline/nested definitions use the real Markdown grammar.
fn reference_definition_destinations(body: &str) -> Vec<String> {
    let parser = Parser::new_ext(body, parse_options());
    let mut definitions: Vec<_> = parser.reference_definitions().iter().map(|(_, definition)| definition).collect();
    definitions.sort_by_key(|definition| definition.span.start);
    definitions.into_iter().map(|definition| definition.dest.to_string()).collect()
}

/// Rekon ToC-style slugs across every heading in order: collisions gain
/// `-1`, `-2`, … Empty titles allocate nothing; punctuation-only titles
/// use the shared `section` fallback.
fn renderer_slugs(headings: &[String]) -> Vec<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut used = HashSet::new();
    headings
        .iter()
        .filter(|text| !text.trim().is_empty())
        .filter_map(|text| {
            let base = slugify(text.trim());
            if base.is_empty() {
                return None;
            }
            let seen = counts.entry(base.clone()).or_default();
            let mut candidate = base.clone();
            while !used.insert(candidate.clone()) {
                *seen += 1;
                candidate = format!("{base}-{seen}");
            }
            Some(candidate)
        })
        .collect()
}

fn duplicate_ids(ids: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();
    for id in ids {
        if !seen.insert(id.as_str()) && !duplicates.contains(id) {
            duplicates.push(id.clone());
        }
    }
    duplicates
}

/// The anchors a renderer offers for a body: renderer slugs plus every
/// explicit id, with duplicates reported once each.
fn anchors_of_scanned(scanned: &Scanned) -> (HashSet<String>, Vec<String>) {
    let mut anchors: HashSet<String> = renderer_slugs(&scanned.headings).into_iter().collect();
    for id in &scanned.ids {
        anchors.insert(id.clone());
    }
    (anchors, duplicate_ids(&scanned.ids))
}

/// Anchors of any on-disk Markdown file, frontmatter excluded.
fn file_anchors(path: &Path, files: &dyn DocumentFiles) -> Option<HashSet<String>> {
    let raw = files.read(path).ok()?;
    let frontmatter = rekon_markup::parse_frontmatter(&raw).ok()?;
    let scanned = scan_markdown(frontmatter.body);
    Some(anchors_of_scanned(&scanned).0)
}

/// The URI scheme of a destination, when it has one (`file`, `https`, …).
fn scheme_of(text: &str) -> Option<&str> {
    let colon = text.find(':')?;
    let scheme = &text[..colon];
    let mut characters = scheme.chars();
    match characters.next() {
        Some(first) if first.is_ascii_alphabetic() => {}
        _ => return None,
    }
    characters.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-')).then_some(scheme)
}

/// Percent-decode `%XX` escapes; `None` when an escape is malformed.
fn percent_decode(text: &str) -> Option<String> {
    if !text.contains('%') {
        return Some(text.to_string());
    }
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let pair = bytes.get(index + 1..index + 3)?;
            let high = (pair[0] as char).to_digit(16)?;
            let low = (pair[1] as char).to_digit(16)?;
            out.push((high * 16 + low) as u8);
            index += 3;
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Resolve one destination against the filesystem; `Some(message)` is a
/// failure. Remote schemes make no existence claim and pass. Fragment-only
/// destinations check against the linking document's own anchors.
fn destination_problem(destination: &str, relative: &Path, root: &Path, files: &dyn DocumentFiles, self_anchors: &HashSet<String>) -> Option<String> {
    if destination.starts_with("//") {
        return None; // Network-path reference, not a bundle-root path.
    }
    let (before, fragment) = match destination.split_once('#') {
        Some((before, fragment)) => (before, fragment),
        None => (destination, ""),
    };
    let path_part = before.split('?').next().unwrap_or("");
    let target: Option<PathBuf> = if let Some(scheme) = scheme_of(path_part) {
        if !scheme.eq_ignore_ascii_case("file") {
            return None;
        }
        let rest = path_part.get(scheme.len() + 1..)?.strip_prefix("//");
        let Some(rest) = rest else {
            return Some(format!("expected an absolute file:// URI: {destination}"));
        };
        let path = rest
            .strip_prefix("localhost/")
            .map(|path| format!("/{path}"))
            .or_else(|| rest.starts_with('/').then(|| rest.to_string()));
        let Some(path) = path else {
            return Some(format!("unsupported file URI authority: {destination}"));
        };
        let Some(decoded) = percent_decode(&path) else {
            return Some(format!("invalid percent encoding: {destination}"));
        };
        Some(PathBuf::from(decoded))
    } else if path_part.is_empty() {
        if fragment.is_empty() {
            return None; // bare self-link `[a]()`
        }
        let candidates = [percent_decode(fragment), Some(fragment.to_string())];
        if candidates.iter().flatten().any(|f| self_anchors.contains(f)) {
            return None;
        }
        return Some(format!("missing anchor {destination}"));
    } else {
        let Some(decoded) = percent_decode(path_part) else {
            return Some(format!("invalid percent encoding: {destination}"));
        };
        if decoded.starts_with('/') {
            Some(root.join(decoded.trim_start_matches('/')))
        } else {
            Some(relative.join(decoded))
        }
    };
    let Some(target) = target else {
        return Some(format!("missing target {destination}"));
    };
    if !files.exists(&target) {
        return Some(format!("missing target {destination}"));
    }
    let found = &target;
    if !fragment.is_empty() && found.extension().is_some_and(|e| e == "md") {
        let Some(anchors) = file_anchors(found, files) else {
            return Some(format!("unreadable anchor target {destination}"));
        };
        let candidates = [percent_decode(fragment), Some(fragment.to_string())];
        if candidates.iter().flatten().any(|f| anchors.contains(f)) {
            return None;
        }
        return Some(format!("missing anchor {destination}"));
    }
    None
}

/// Recognize a native session authority, including qualified/message sources.
fn is_native_session(resource: &str) -> bool {
    match resource.strip_prefix("opencode://ses_") {
        Some(rest) => {
            let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
            !authority.is_empty() && authority.chars().all(|c| c.is_ascii_alphanumeric())
        }
        None => false,
    }
}

/// The `--okf` contract: required fields, native provenance, source targets.
#[allow(clippy::too_many_arguments)]
fn check_okf(
    metadata: Option<&Value>,
    document: &Path,
    display: &str,
    root: &Path,
    files: &dyn DocumentFiles,
    self_anchors: &HashSet<String>,
    checks: &mut DocumentChecks,
) {
    let Some(metadata) = metadata else {
        checks.fail(
            display,
            None,
            format!(
                "missing OKF frontmatter with {}",
                OKF_REQUIRED_FIELDS.map(|field| format!("\"{field}\"")).join(", ")
            ),
        );
        return;
    };
    for field in OKF_REQUIRED_FIELDS {
        if metadata.get(field).is_none_or(Value::is_null) {
            checks.fail(display, None, format!("missing required field \"{field}\""));
        }
    }
    let Some(Value::Array(entries)) = metadata.get("sources") else {
        if metadata.get("sources").is_some_and(|value| !value.is_null()) {
            checks.fail(display, None, "sources must be a list of entries");
        }
        return;
    };
    let relative = document
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new(""));
    let mut native = false;
    for entry in entries {
        let Some(Value::String(resource)) = entry.get("resource") else {
            checks.fail(display, None, "source entry needs a string resource");
            continue;
        };
        checks.sources += 1;
        native |= is_native_session(resource);
        if let Some(problem) = destination_problem(resource, relative, root, files, self_anchors) {
            checks.fail(display, None, problem);
        }
    }
    if !native {
        checks.fail(display, None, "no native session source (add an \"opencode://ses_…\" sources entry)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// In-memory filesystem: a document maps to its text; presence = mapped.
    struct MapFiles {
        entries: HashMap<PathBuf, String>,
    }

    impl MapFiles {
        fn new(entries: &[(&str, &str)]) -> Self {
            Self {
                entries: entries.iter().map(|(path, text)| (PathBuf::from(path), text.to_string())).collect(),
            }
        }
    }

    impl DocumentFiles for MapFiles {
        fn exists(&self, path: &Path) -> bool {
            self.entries.contains_key(path)
        }

        fn read(&self, path: &Path) -> std::io::Result<String> {
            self.entries
                .get(path)
                .cloned()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "missing"))
        }
    }

    fn run(files: &MapFiles, document: &str, raw: &str, okf: bool) -> DocumentChecks {
        check_document(raw, Path::new(document), document, &CheckOptions { root: Path::new("/"), okf }, files)
    }

    fn messages(checks: &DocumentChecks) -> Vec<String> {
        checks.failures.iter().map(Failure::render).collect()
    }

    #[test]
    fn flags_trailing_whitespace_and_unclosed_fences_with_lines() {
        let files = MapFiles::new(&[]);
        let raw = "# Title\n\ntext \t\nhard break  \n\n```\ncode with trailing   \n```\n";
        let checks = run(&files, "doc.md", raw, false);
        assert_eq!(messages(&checks), vec!["doc.md:3: trailing whitespace".to_string()]);

        let unclosed = "# Title\n\n~~~\nstill open\n";
        let checks = run(&files, "doc.md", unclosed, false);
        assert_eq!(messages(&checks), vec!["doc.md:3: unclosed fenced code block".to_string()]);
    }

    #[test]
    fn closing_fence_must_match_the_opening_character_and_length() {
        let files = MapFiles::new(&[]);
        // A shorter backtick run cannot close a longer fence; only text after
        // the true closing fence is style-checked.
        let mismatched = "# Title\n\n````\ncode\n```\nstill fenced\n````\ntrailing \n";
        let checks = run(&files, "doc.md", mismatched, false);
        assert_eq!(messages(&checks), vec!["doc.md:8: trailing whitespace".to_string()]);
    }

    #[test]
    fn resolves_relative_rooted_file_and_remote_destinations() {
        let files = MapFiles::new(&[
            ("dir/neighbor.md", "# Neighbor\n"),
            ("/rooted/target.md", "# Rooted\n"),
            ("/checkout/external.md", "# External\n"),
        ]);
        let raw = "# Title\n\n[neighbor](neighbor.md) [rooted](/rooted/target.md) \
[external](file:///checkout/external.md) [remote](https://example.org/x.md) [mail](mailto:a@b.c)\n";
        let checks = run(&files, "dir/doc.md", raw, false);
        assert!(checks.failures.is_empty(), "{:?}", messages(&checks));
        assert_eq!(checks.destinations, 5);

        let broken = "# Title\n\n[missing](absent-neighbor.md) [gone](/rooted/absent.md) \
[lost](file:///checkout/absent.md)\n";
        let checks = run(&files, "dir/doc.md", broken, false);
        assert_eq!(
            messages(&checks),
            vec![
                "dir/doc.md: missing target absent-neighbor.md".to_string(),
                "dir/doc.md: missing target /rooted/absent.md".to_string(),
                "dir/doc.md: missing target file:///checkout/absent.md".to_string(),
            ]
        );
    }

    #[test]
    fn checks_anchors_against_slugs_explicit_ids_and_deduplication() {
        let files = MapFiles::new(&[("target.md", "# Same\n\n<a id=\"custom\"></a>\n\n## Same\n\n### Other & Thing\n")]);
        let raw = "# Title\n\n[slug](target.md#other--thing) [explicit](target.md#custom) \
[repeat](target.md#same-1) [self](#title) [gone](target.md#nope)\n";
        let checks = run(&files, "doc.md", raw, false);
        assert_eq!(messages(&checks), vec!["doc.md: missing anchor target.md#nope".to_string()]);
    }

    #[test]
    fn flags_duplicate_explicit_ids_and_ignores_comment_probes() {
        let files = MapFiles::new(&[]);
        let raw = "# Title\n\n<!-- id=\"ghost\" -->\n\n<a id=\"one\"></a>\n\n<dfn id=\"one\">again</dfn>\n";
        let checks = run(&files, "doc.md", raw, false);
        assert_eq!(messages(&checks), vec!["doc.md: duplicate explicit id \"one\"".to_string()]);
    }

    #[test]
    fn checks_html_destinations_and_reference_definitions() {
        let files = MapFiles::new(&[("dir/ok.png", "image"), ("dir/ref.md", "# Ref\n")]);
        let raw = "# Title\n\n<img src=\"ok.png\" alt=\"x\"> <a href=\"absent.md\">gone</a>\n\n\
[unused]: missing.md\n[angled]: <ref.md>\n";
        let checks = run(&files, "dir/doc.md", raw, false);
        assert_eq!(
            messages(&checks),
            vec![
                "dir/doc.md: missing target absent.md".to_string(),
                "dir/doc.md: missing target missing.md".to_string(),
            ]
        );
        assert_eq!(checks.destinations, 4);
    }

    #[test]
    fn code_spans_are_not_links() {
        let files = MapFiles::new(&[]);
        let raw = "# Title\n\nuse `[x](neighbor.md)` inline\n";
        let checks = run(&files, "doc.md", raw, false);
        assert!(checks.failures.is_empty());
        assert_eq!(checks.destinations, 0);
    }

    #[test]
    fn percent_encoded_targets_and_anchors_decode() {
        let files = MapFiles::new(&[("a b.md", "# Héllo\n")]);
        let raw = "# Title\n\n[one](a%20b.md) [two](a%20b.md#h%C3%A9llo)\n";
        let checks = run(&files, "doc.md", raw, false);
        assert!(checks.failures.is_empty(), "{:?}", messages(&checks));
    }

    #[test]
    fn okf_requires_fields_native_provenance_and_resolvable_sources() {
        let files = MapFiles::new(&[("/CONTEXT.md", "# Glossary\n")]);
        let document = |sources: &str| format!("---\ntype: Reference\ntitle: T\ndescription: D\nresource: /doc.md\nstatus: draft\n{sources}---\n\n# Body\n");
        let provenance = "sources:\n  - { resource: \"opencode://ses_abc123\", title: Session }\n";

        let missing_frontmatter = run(&files, "doc.md", "# No frontmatter\n", true);
        assert_eq!(messages(&missing_frontmatter).len(), 1);
        assert!(messages(&missing_frontmatter)[0].contains("missing OKF frontmatter"));

        let checks = run(&files, "doc.md", &document(provenance), true);
        assert!(checks.failures.is_empty(), "{:?}", messages(&checks));
        assert_eq!(checks.sources, 1);

        let no_native = document("sources:\n  - { resource: \"urn:opencode:session:ses_abc\", title: Legacy }\n  - { resource: /CONTEXT.md, title: Home }\n");
        let checks = run(&files, "doc.md", &no_native, true);
        assert_eq!(
            messages(&checks),
            vec!["doc.md: no native session source (add an \"opencode://ses_…\" sources entry)".to_string(),]
        );
        assert_eq!(checks.sources, 2);

        let dangling = document(&format!("{provenance}  - {{ resource: /gone.md, title: Gone }}\n"));
        let checks = run(&files, "doc.md", &dangling, true);
        assert_eq!(messages(&checks), vec!["doc.md: missing target /gone.md".to_string()]);

        let incomplete = "---\ntype: Reference\ntitle: T\nstatus: draft\n---\n\n# Body\n";
        let checks = run(&files, "doc.md", incomplete, true);
        let reported = messages(&checks);
        for field in ["description", "resource", "sources"] {
            assert!(
                reported.iter().any(|message| message.contains(&format!("missing required field \"{field}\""))),
                "{reported:?}"
            );
        }
    }

    #[test]
    fn native_session_grammar() {
        assert!(is_native_session("opencode://ses_abc123"));
        assert!(!is_native_session("opencode://ses_"));
        assert!(is_native_session("opencode://ses_abc/m/msg_1?source=local"));
        assert!(is_native_session("opencode://ses_abc?source=local"));
        assert!(!is_native_session("urn:opencode:session:ses_abc"));
    }

    #[test]
    fn malformed_frontmatter_fails_in_both_profiles() {
        let files = MapFiles::new(&[]);
        let raw = "---\nbroken: [unclosed\n---\n\n# Body\n";
        let checks = run(&files, "doc.md", raw, false);
        assert_eq!(checks.failures.len(), 1);
        assert!(messages(&checks)[0].starts_with("doc.md: invalid frontmatter"));
    }

    #[test]
    fn definitions_use_parser_boundaries_not_code_or_html_examples() {
        let files = MapFiles::new(&[("valid.md", "# V\n")]);
        let raw =
            "# T\n\n```md\n[fake]: missing-code.md\n```\n\n<div>\n[fake]: missing-html.md\n</div>\n\n[real]:\n  valid.md\n\n> [unused]: missing-real.md\n";
        let checked = run(&files, "doc.md", raw, false);
        assert_eq!(messages(&checked), ["doc.md: missing target missing-real.md"]);
        assert_eq!(checked.destinations, 2);
    }

    #[test]
    fn nested_fences_and_literal_fences_follow_parser_blocks() {
        let files = MapFiles::new(&[]);
        for raw in [
            "> ```md\n> code  \n> ```\n",
            "- item\n\n  ~~~\n  code\n  ~~~\n",
            "    ``` literal indented code\n",
            "<pre>\n``` literal HTML\n</pre>\n",
            "```\n```\n",
        ] {
            let checked = run(&files, "doc.md", raw, false);
            assert!(checked.failures.is_empty(), "{raw:?}: {:?}", messages(&checked));
        }
        for raw in ["> ```\n> missing close\n", "- ```\n  missing close\n", "```"] {
            let checked = run(&files, "doc.md", raw, false);
            assert!(messages(&checked).iter().any(|m| m.contains("unclosed fenced")), "{raw:?}");
        }
    }

    #[test]
    fn duplicate_heading_suffixes_do_not_collide_with_real_headings() {
        let files = MapFiles::new(&[]);
        let raw = "# Same\n\n## Same-1\n\n## Same\n\n[third](#same-2)\n";
        assert!(run(&files, "doc.md", raw, false).failures.is_empty());
    }

    #[test]
    fn file_authorities_and_network_paths_are_not_relative_files() {
        let files = MapFiles::new(&[("/external/a.md", "# A\n")]);
        let raw = "[local](file://localhost/external/a.md#a) [network](//example.org/absent.md)";
        assert!(run(&files, "doc.md", raw, false).failures.is_empty());
        let checked = run(&files, "doc.md", "[remote](file://server/path) [bad](a%ZZ.md)", false);
        assert!(messages(&checked)[0].contains("unsupported file URI authority"));
        assert!(messages(&checked)[1].contains("invalid percent encoding"));
    }
}
