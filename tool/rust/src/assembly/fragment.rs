//! Fragment contract: frontmatter metadata, prose shape, and relocation safety.
//!
//! Port of the assembler-specific fragment rules in `tool/md/fragment.ts`,
//! upgraded for enriched assembly: frontmatter is available as scoped
//! [`Okf`][rekon_markup] data, and explicit in-source IDs are guarded so
//! they can never collide with generated section wrappers or the reserved
//! table-of-contents nav. Frontmatter parsing and Okf emission come from the
//! workspace's shared `rekon-markup` library; this module owns only the
//! assembly-specific contract.

use std::collections::HashSet;
use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use serde_json::Value;
use thiserror::Error;

use crate::manifest::is_provenance_text;
use crate::nodepath;

use super::transform::parse_options;

/// The final ToC pass owns this ID in the published document; a source using
/// it could be mistaken for the assembly's own nav and silently replaced.
pub const RESERVED_TOC_ID: &str = "rekon-toc";

/// Assembler-specific fragment metadata; order composes the document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FragmentMeta {
    pub id: String,
    pub order: i64,
    /// Canonical bundle-root-relative home, retained in the Okf metadata.
    /// The actual fragment path separately supplies the wrapper's `data-source`.
    pub source: String,
    pub status: FragmentStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FragmentStatus {
    Draft,
    Stable,
}

impl FragmentStatus {
    fn parse(raw: &str) -> Option<Self> {
        match raw {
            "draft" => Some(Self::Draft),
            "stable" => Some(Self::Stable),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Stable => "stable",
        }
    }
}

/// A validated fragment. `metadata` is the complete frontmatter object,
/// including unknown fields; `okf` is its rendered `<Okf data={...} />` element,
/// which assembly may replace with a manifest-filtered rendering.
#[derive(Clone, Debug)]
pub struct Fragment {
    pub path: String,
    pub meta: FragmentMeta,
    /// Body after CRLF normalization and edge trimming; source otherwise preserved.
    pub prose: String,
    pub prose_bytes: usize,
    pub metadata: Value,
    pub okf: String,
    /// Explicit in-source IDs (inline anchors, `dfn`/HTML `id` attributes).
    pub anchor_ids: Vec<String>,
}

#[derive(Error, Debug)]
pub enum FragmentError {
    #[error("{path}: {message}")]
    Frontmatter { path: String, message: String },
    #[error("{path}: invalid frontmatter: {message}")]
    Metadata { path: String, message: String },
    #[error("{path}: {message}")]
    Prose { path: String, message: String },
    #[error(
        "{path}: destination \"{destination}\" is fragment-relative and would retarget when assembled; use a bundle-root-relative (/...) or protocol destination"
    )]
    Link { path: String, destination: String },
    #[error("{path}: explicit id \"{id}\": {reason}")]
    AnchorId {
        path: String,
        id: String,
        reason: String,
    },
}

/// Validate one fragment without rendering or rewriting its Markdown.
pub fn parse_fragment(raw: &str, path: &str) -> Result<Fragment, FragmentError> {
    let frontmatter =
        rekon_markup::parse_frontmatter(raw).map_err(|error| FragmentError::Frontmatter {
            path: path.into(),
            message: error.to_string(),
        })?;
    let Some(metadata) = frontmatter.metadata else {
        return Err(FragmentError::Frontmatter {
            path: path.into(),
            message: "fragment must declare frontmatter metadata (id, order, source, status)"
                .into(),
        });
    };
    let meta = fragment_metadata(&metadata, path)?;
    let okf = rekon_markup::render_okf(&metadata).map_err(|error| FragmentError::Metadata {
        path: path.into(),
        message: error.to_string(),
    })?;
    let prose = frontmatter.body.replace("\r\n", "\n").trim().to_string();
    let anchor_ids = validate_prose(&prose, path)?;
    let prose_bytes = prose.len();
    Ok(Fragment {
        path: path.into(),
        meta,
        prose,
        prose_bytes,
        metadata,
        okf,
        anchor_ids,
    })
}

fn invalid_field(key: &str, path: &str, expected: &str) -> FragmentError {
    FragmentError::Metadata {
        path: path.into(),
        message: format!("{key} must be {expected}"),
    }
}

fn fragment_metadata(metadata: &Value, path: &str) -> Result<FragmentMeta, FragmentError> {
    let field = |key: &str| -> Result<&Value, FragmentError> {
        metadata.get(key).ok_or_else(|| {
            invalid_field(
                key,
                path,
                &format!("present: missing required field {key:?}"),
            )
        })
    };
    let id = field("id")?
        .as_str()
        .ok_or_else(|| invalid_field("id", path, "a lowercase kebab-case identifier string"))?;
    if !is_kebab_case(id) {
        return Err(invalid_field(
            "id",
            path,
            "a lowercase kebab-case identifier",
        ));
    }
    let order = field("order")?
        .as_i64()
        .ok_or_else(|| invalid_field("order", path, "an integer"))?;
    let source = field("source")?
        .as_str()
        .ok_or_else(|| invalid_field("source", path, "a bundle-root-relative path string"))?;
    validate_bundle_source(source).map_err(|expected| invalid_field("source", path, &expected))?;
    let status = field("status")?
        .as_str()
        .ok_or_else(|| invalid_field("status", path, "a string, either \"draft\" or \"stable\""))?;
    let status = FragmentStatus::parse(status)
        .ok_or_else(|| invalid_field("status", path, "\"draft\" or \"stable\""))?;
    Ok(FragmentMeta {
        id: id.into(),
        order,
        source: source.into(),
        status,
    })
}

fn is_kebab_case(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        })
}

/// Mirrors the TypeScript assembler's `BundleSource` rules.
fn validate_bundle_source(source: &str) -> Result<(), String> {
    if source.len() <= 1 || !source.starts_with('/') || source.starts_with("//") {
        return Err("a bundle-root-relative path starting with a single '/'".into());
    }
    if !is_provenance_text(source) {
        return Err("a provenance-safe path without \"--\" or control characters".into());
    }
    let rest = &source[1..];
    if nodepath::normalize(rest) != rest
        || rest
            .split('/')
            .any(|segment| segment == "." || segment == "..")
    {
        return Err("a normalized path without '.' or '..' segments".into());
    }
    Ok(())
}

/// Validate body shape and relocation safety; return the explicit anchor IDs.
fn validate_prose(prose: &str, path: &str) -> Result<Vec<String>, FragmentError> {
    let fail = |message: String| FragmentError::Prose {
        path: path.into(),
        message,
    };
    let events: Vec<(Event<'_>, Range<usize>)> = Parser::new_ext(prose, parse_options())
        .into_offset_iter()
        .collect();

    let mut containers = 0usize;
    let mut h1_count = 0usize;
    let mut first_h1_end: Option<usize> = None;
    let mut inside_h1 = false;
    let mut h1_text = String::new();
    let mut saw_meaningful = false;
    let mut anchors: Vec<String> = Vec::new();
    let mut seen_anchors: HashSet<String> = HashSet::new();

    for (event, range) in &events {
        if !saw_meaningful && is_meaningful(event, prose, range) {
            saw_meaningful = true;
            let begins = containers == 0
                && matches!(
                    event,
                    Event::Start(Tag::Heading {
                        level: HeadingLevel::H1,
                        ..
                    })
                );
            if !begins {
                return Err(fail(
                    "fragment must begin with a top-level (#) heading".into(),
                ));
            }
        }
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                } if containers == 0 => {
                    h1_count += 1;
                    inside_h1 = true;
                }
                Tag::BlockQuote(_) | Tag::List(_) | Tag::Item | Tag::FootnoteDefinition(_) => {
                    containers += 1;
                }
                Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. } => {
                    validate_destination(dest_url, path)?;
                }
                _ => {}
            },
            Event::End(end) => match end {
                TagEnd::Heading(_) if inside_h1 => {
                    inside_h1 = false;
                    first_h1_end = Some(range.end);
                }
                TagEnd::BlockQuote(_)
                | TagEnd::List(_)
                | TagEnd::Item
                | TagEnd::FootnoteDefinition => {
                    containers -= 1;
                }
                _ => {}
            },
            Event::Text(text) | Event::Code(text) if inside_h1 => h1_text.push_str(text),
            Event::Html(chunk) | Event::InlineHtml(chunk) => {
                for destination in html_attribute_values(chunk, "href")
                    .into_iter()
                    .chain(html_attribute_values(chunk, "src"))
                {
                    validate_destination(destination, path)?;
                }
                for id in html_attribute_values(chunk, "id") {
                    check_anchor_id(id, path, &mut seen_anchors, &mut anchors)?;
                }
            }
            _ => {}
        }
    }

    if !saw_meaningful {
        return Err(fail("fragment has no Markdown content".into()));
    }
    if h1_count != 1 {
        return Err(fail(format!(
            "fragment must contain exactly one top-level (#) heading, found {h1_count}"
        )));
    }
    if h1_text.trim().is_empty() {
        return Err(fail("fragment top-level heading needs plain text".into()));
    }
    let h1_end = first_h1_end.unwrap_or_default();
    if !events
        .iter()
        .any(|(event, range)| range.start >= h1_end && is_meaningful(event, prose, range))
    {
        return Err(fail(
            "fragment needs content beyond its top-level heading".into(),
        ));
    }
    validate_reference_definitions(prose, path)?;
    Ok(anchors)
}

fn is_meaningful(event: &Event<'_>, prose: &str, range: &Range<usize>) -> bool {
    match event {
        Event::Text(text) | Event::Code(text) => !text.trim().is_empty(),
        Event::Html(chunk) | Event::InlineHtml(chunk) => !chunk.trim().is_empty(),
        Event::Rule => true,
        Event::Start(tag) => {
            matches!(
                tag,
                Tag::Paragraph
                    | Tag::Heading { .. }
                    | Tag::CodeBlock(_)
                    | Tag::HtmlBlock
                    | Tag::List(_)
                    | Tag::Item
                    | Tag::BlockQuote(_)
                    | Tag::FootnoteDefinition(_)
                    | Tag::Table(_)
                    | Tag::DefinitionList
                    | Tag::DefinitionListTitle
                    | Tag::DefinitionListDefinition
            ) && !prose[range.clone()].trim().is_empty()
        }
        _ => false,
    }
}

/// Allowed destinations: anchors, empty self-links, single-root-relative `/...`,
/// and protocol URIs. Anything else retargets when fragments are combined.
fn validate_destination(destination: &str, path: &str) -> Result<(), FragmentError> {
    let safe = destination.is_empty()
        || destination.starts_with('#')
        || (destination.starts_with('/') && !destination.starts_with("//"))
        || is_protocol_destination(destination);
    if safe {
        Ok(())
    } else {
        Err(FragmentError::Link {
            path: path.into(),
            destination: destination.into(),
        })
    }
}

fn is_protocol_destination(destination: &str) -> bool {
    let mut characters = destination.chars();
    match characters.next() {
        Some(first) if first.is_ascii_alphabetic() => {
            let mut scheme = false;
            for character in characters {
                if character == ':' {
                    return scheme;
                }
                if character.is_ascii_alphanumeric()
                    || character == '+'
                    || character == '.'
                    || character == '-'
                {
                    scheme = true;
                } else {
                    return false;
                }
            }
            false
        }
        _ => false,
    }
}

fn check_anchor_id(
    id: &str,
    path: &str,
    seen: &mut HashSet<String>,
    anchors: &mut Vec<String>,
) -> Result<(), FragmentError> {
    if id.is_empty() {
        return Ok(());
    }
    let reserved = |reason: String| FragmentError::AnchorId {
        path: path.into(),
        id: id.into(),
        reason,
    };
    if id == RESERVED_TOC_ID {
        return Err(reserved(
            "rekon-toc is reserved for the assembly's generated table of contents; rename this anchor"
                .into(),
        ));
    }
    if !seen.insert(id.to_string()) {
        return Err(reserved(
            "this fragment already defines this explicit id".into(),
        ));
    }
    anchors.push(id.to_string());
    Ok(())
}

/// Reference definitions are not parser events when unused, so scan the source
/// lexically; this covers used and unused definitions alike.
fn validate_reference_definitions(prose: &str, path: &str) -> Result<(), FragmentError> {
    for line in prose.split_inclusive('\n') {
        let indent = line.len() - line.trim_start_matches(' ').len();
        if indent > 3 {
            continue;
        }
        let Some(rest) = line[indent..].strip_prefix('[') else {
            continue;
        };
        let Some(close) = rest.find(']') else {
            continue;
        };
        let Some(after) = rest[close + 1..].strip_prefix(':') else {
            continue;
        };
        let after = after.trim_start();
        let destination = if let Some(inner) = after.strip_prefix('<') {
            match inner.find('>') {
                Some(end) => &inner[..end],
                None => continue,
            }
        } else {
            after
                .split(|character: char| character.is_whitespace())
                .next()
                .unwrap_or("")
        };
        validate_destination(destination, path)?;
    }
    Ok(())
}

/// Extract `name="value"` / `name='value'` / unquoted attribute values from raw
/// HTML chunks. Word-boundary checked, so `data-id=` does not match `id`.
pub(crate) fn html_attribute_values<'a>(chunk: &'a str, wanted: &str) -> Vec<&'a str> {
    let bytes = chunk.as_bytes();
    let mut values = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = chunk[scan..].find(wanted) {
        let name = scan + found;
        scan = name + wanted.len();
        let preceded = name > 0
            && (bytes[name - 1].is_ascii_alphanumeric()
                || bytes[name - 1] == b'-'
                || bytes[name - 1] == b'_');
        if preceded {
            continue;
        }
        let Some(rest) = chunk[scan..].strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim_start_matches(' ');
        let (value, consumed) = if let Some(inner) = rest.strip_prefix('"') {
            match inner.find('"') {
                Some(end) => (&inner[..end], end + 2),
                None => (inner, inner.len() + 1),
            }
        } else if let Some(inner) = rest.strip_prefix('\'') {
            match inner.find('\'') {
                Some(end) => (&inner[..end], end + 2),
                None => (inner, inner.len() + 1),
            }
        } else {
            let end = rest
                .find(|character: char| character.is_whitespace() || character == '>')
                .unwrap_or(rest.len());
            (&rest[..end], end.max(1))
        };
        if !value.is_empty() {
            values.push(value);
        }
        scan = chunk.len() - rest.len() + consumed;
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fragment_source(id: &str, body: &str) -> String {
        format!(
            "---\nid: {id}\norder: 10\nsource: /{id}/README.md\nstatus: stable\n---\n\n{body}\n"
        )
    }

    #[test]
    fn parses_metadata_prose_bytes_and_unknown_metadata() {
        let raw = "---\nid: alpha\norder: 7\nsource: /a/G.md\nstatus: draft\nkeep:\n  nested: {leaf: true}\ntags: [one, two]\n---\n\n# Alpha\n\nBodün.\n";
        let fragment = parse_fragment(raw, "a/G.md").unwrap();
        assert_eq!(fragment.meta.id, "alpha");
        assert_eq!(fragment.meta.order, 7);
        assert_eq!(fragment.meta.source, "/a/G.md");
        assert_eq!(fragment.meta.status, FragmentStatus::Draft);
        assert_eq!(fragment.prose, "# Alpha\n\nBodün.");
        assert_eq!(fragment.prose_bytes, "# Alpha\n\nBodün.".len());
        assert_eq!(fragment.metadata["keep"]["nested"]["leaf"], true);
        assert_eq!(fragment.metadata["tags"][1], "two");
        assert!(fragment.okf.starts_with("<Okf data={"));
        assert!(fragment.okf.ends_with("} />"));
        assert!(fragment.okf.contains("\"leaf\":true"));
        assert_eq!(fragment.anchor_ids, Vec::<String>::new());
    }

    #[test]
    fn rejects_missing_malformed_and_unsafe_metadata() {
        let cases = [
            "# No frontmatter\n",
            "---\nid: alpha\norder: 1\nsource: /a.md\nstatus: draft\n",
            "---\n5\n---\n\n# Alpha\n",
            "---\norder: 1\nsource: /a.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: Alpha\norder: 1\nsource: /a.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1.5\nsource: /a.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: \"1\"\nsource: /a.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1\nsource: a.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1\nsource: //a.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1\nsource: /\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1\nsource: /a/../b.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1\nsource: /a--b.md\nstatus: draft\n---\n\n# A\n\nb\n",
            "---\nid: alpha\norder: 1\nsource: /a.md\nstatus: wip\n---\n\n# A\n\nb\n",
        ];
        for raw in cases {
            let error = parse_fragment(raw, "bad.md").expect_err(raw);
            assert!(
                matches!(
                    error,
                    FragmentError::Frontmatter { .. } | FragmentError::Metadata { .. }
                ),
                "{raw:?} -> {error:?}"
            );
        }
    }

    #[test]
    fn rejects_unshaped_prose() {
        let cases = [
            "",                            // no content
            "Just prose.\n",               // begins with a paragraph
            "# One\n\n# Two\n\nbodies\n",  // two top-level headings
            "#\n\nbody\n",                 // heading without text
            "# Only a heading\n",          // nothing beyond the heading
            "> # Quoted\n\nquoted only\n", // heading inside a container
        ];
        for body in cases {
            let raw = fragment_source("alpha", body);
            let error = parse_fragment(&raw, "bad.md").expect_err(body);
            assert!(
                matches!(error, FragmentError::Prose { .. }),
                "{body:?} -> {error:?}"
            );
        }
    }

    #[test]
    fn rejects_unsafe_destinations_including_unused_definitions_and_html() {
        let cases = [
            "[x](y.md)",
            "[x](../y.md)",
            "<img src=\"y.png\">",
            "<a href=\"y.md\">x</a>",
            "[//host/x]: /ok.md\n\nsee [x](//host/x)\n",
        ];
        for body in cases {
            let raw = fragment_source("alpha", &format!("# Alpha\n\n{body}\n"));
            let error = parse_fragment(&raw, "bad.md").expect_err(body);
            assert!(
                matches!(error, FragmentError::Link { .. }),
                "{body:?} -> {error:?}"
            );
        }
        let safe = "# Alpha\n\n[a]() [b](#c) [d](/e.md) [f](https://g.h) <https://i.j> [u]: /u.md\n\nsee [u]\n";
        parse_fragment(&fragment_source("alpha", safe), "ok.md").unwrap();
    }

    #[test]
    fn guards_explicit_anchor_ids_against_reservation_and_duplicates() {
        {
            let id = RESERVED_TOC_ID;
            let raw = fragment_source("alpha", &format!("# Alpha\n\n<a id=\"{id}\"></a>\n"));
            let error = parse_fragment(&raw, "bad.md").expect_err(id);
            let FragmentError::AnchorId {
                id: found, reason, ..
            } = error
            else {
                panic!("expected an anchor id error for {id}");
            };
            assert_eq!(found, id);
            assert!(!reason.is_empty());
        }
        let duplicated = fragment_source(
            "alpha",
            "# Alpha\n\n<dl>\n<dt><dfn id=\"same\">A</dfn></dt>\n<dd>b</dd>\n</dl>\n\n<a id=\"same\"></a>\n",
        );
        let error = parse_fragment(&duplicated, "bad.md").unwrap_err();
        assert!(matches!(error, FragmentError::AnchorId { .. }), "{error:?}");
        let kept = fragment_source(
            "alpha",
            "# Alpha\n\n<dl>\n<dt><dfn id=\"term\">A</dfn></dt>\n<dd>b</dd>\n</dl>\n",
        );
        let fragment = parse_fragment(&kept, "ok.md").unwrap();
        assert_eq!(fragment.anchor_ids, vec!["term".to_string()]);
    }

    #[test]
    fn html_attribute_scan_respects_word_boundaries_and_quotes() {
        let chunk = r#"<a data-id="nope" id='real one' href=/bare x id=tail>"#;
        assert_eq!(html_attribute_values(chunk, "id"), vec!["real one", "tail"]);
        assert_eq!(html_attribute_values(chunk, "href"), vec!["/bare"]);
        assert!(html_attribute_values("plain text", "id").is_empty());
    }
}
