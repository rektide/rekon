//! Source-span body surgery: heading shift, nested semantic sections, and
//! MDX-safe prose, applied as edits over the original source bytes.
//!
//! Prose is never re-rendered from events: CommonMark/GFM source is preserved
//! except where MDX would misread it. Fenced code is never touched because it
//! is opaque in the event stream, and all edits are byte-span replacements on
//! the original text. Exotic syntax that cannot be made MDX-safe stays verbatim
//! rather than being rewritten by a growing handwritten scanner.

use std::collections::HashSet;
use std::ops::Range;

use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};

use super::fragment::FragmentError;

/// GFM constructs whose output survives MDX unchanged; used by validation and
/// transformation alike so both agree on what is code, text, or markup.
pub(crate) fn parse_options() -> Options {
    Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS
}

/// Edit priority at equal offsets: section insertions precede the heading
/// text they wrap; inline rewrites never share an offset with those.
const PRIORITY_SECTION: u8 = 1;
const PRIORITY_HEADING: u8 = 2;
const PRIORITY_INLINE: u8 = 3;

struct Edit {
    range: Range<usize>,
    priority: u8,
    text: String,
}

struct SourceHeading {
    depth: u8,
    start: usize,
    top_level: bool,
    text: String,
}

/// Transform a validated fragment body for life inside the assembled document:
/// the leading H1 becomes the fragment section's H2, deeper headings shift by
/// one and gain deterministic nested `<section>` wrappers, and prose is made
/// MDX-safe. Code is never touched; unique wrapper IDs are enforced loudly.
pub(crate) fn build_body(
    prose: &str,
    root_id: &str,
    path: &str,
    used_ids: &mut HashSet<String>,
) -> Result<String, FragmentError> {
    let events: Vec<(Event<'_>, Range<usize>)> = Parser::new_ext(prose, parse_options())
        .into_offset_iter()
        .collect();
    let fail = |message: String| FragmentError::Prose {
        path: path.into(),
        message,
    };

    let mut edits: Vec<Edit> = Vec::new();
    let mut headings: Vec<SourceHeading> = Vec::new();
    let mut containers = 0usize;
    let mut code_blocks = 0usize;

    let mut index = 0usize;
    while index < events.len() {
        let (event, range) = (&events[index].0, events[index].1.clone());
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let end = matching_end(&events, index);
                let depth = *level as u8;
                let text = heading_text(&events, index, end);
                if depth >= 6 {
                    return Err(fail(format!(
                        "heading depth {depth} would shift to {}, past the H6 limit; keep fragment headings at H1-H5: \"{}\"",
                        depth + 1,
                        text.trim()
                    )));
                }
                let raw = &prose[range.clone()];
                let line = raw.split('\n').next().unwrap_or(raw);
                let indent = line.len() - line.trim_start_matches(' ').len();
                if line[indent..].starts_with('#') {
                    // ATX: bump the opening hash run; closing sequences stay valid.
                    let hashes = line[indent..]
                        .bytes()
                        .take_while(|byte| *byte == b'#')
                        .count();
                    edits.push(Edit {
                        range: range.start + indent..range.start + indent + hashes,
                        priority: PRIORITY_HEADING,
                        text: "#".repeat(hashes + 1),
                    });
                } else {
                    // Setext: re-encode as ATX, removing the underline with its
                    // preceding newline; following blank lines stay untouched.
                    let newline = raw.find('\n').expect("setext headings span two lines");
                    let underline = raw[newline + 1..].split('\n').next().unwrap_or("");
                    edits.push(Edit {
                        range: range.start..range.start,
                        priority: PRIORITY_HEADING,
                        text: format!("{} ", "#".repeat(depth as usize + 1)),
                    });
                    edits.push(Edit {
                        range: range.start + newline..range.start + newline + 1 + underline.len(),
                        priority: PRIORITY_HEADING,
                        text: String::new(),
                    });
                }
                headings.push(SourceHeading {
                    depth,
                    start: range.start,
                    top_level: containers == 0,
                    text,
                });
                index = end;
            }
            Event::Start(Tag::Link {
                link_type: LinkType::Autolink | LinkType::Email,
                dest_url,
                ..
            }) => {
                // `<uri>` would open a JSX tag in MDX; an identical markdown
                // link renders the same and stays valid.
                let end = matching_end(&events, index);
                let mut label = String::new();
                for (inner, _) in &events[index + 1..end] {
                    if let Event::Text(text) = inner {
                        label.push_str(text);
                    }
                }
                edits.push(Edit {
                    range: range.start..events[end].1.end,
                    priority: PRIORITY_INLINE,
                    text: format!("[{label}]({dest_url})"),
                });
                index = end;
            }
            Event::Start(tag) => match tag {
                Tag::CodeBlock(_) => code_blocks += 1,
                _ if matches!(
                    tag,
                    Tag::BlockQuote(_) | Tag::List(_) | Tag::Item | Tag::FootnoteDefinition(_)
                ) =>
                {
                    containers += 1;
                }
                _ => {}
            },
            Event::End(end) => match end {
                TagEnd::CodeBlock => code_blocks -= 1,
                _ if matches!(
                    end,
                    TagEnd::BlockQuote(_)
                        | TagEnd::List(_)
                        | TagEnd::Item
                        | TagEnd::FootnoteDefinition
                ) =>
                {
                    containers -= 1;
                }
                _ => {}
            },
            Event::Text(_) if code_blocks == 0 => {
                // Merge the consecutive text run before scanning: parsers
                // split `<Okf` and `<b`-like prefixes into separate
                // one-character events, and static elements must be found
                // whole while escaping stays per-event.
                let mut run: Vec<(Range<usize>, &str)> = Vec::new();
                let mut scan = index;
                while let Some((Event::Text(value), range)) = events.get(scan) {
                    run.push((range.clone(), value));
                    scan += 1;
                }
                let merged: String = run.iter().map(|(_, value)| *value).collect();
                let run_start = run[0].0.start;
                let spans = static_okf_spans(&merged, run_start)
                    .map_err(|problem| fail(format!("static Okf element: {problem}")))?;
                let mut complement: Vec<Range<usize>> = Vec::new();
                let mut covered = 0usize;
                for span in &spans {
                    if covered < span.start {
                        complement.push(covered..span.start);
                    }
                    covered = span.end;
                }
                if covered < merged.len() {
                    complement.push(covered..merged.len());
                }
                let mut offset = 0usize;
                for (range, value) in &run {
                    let event_end = offset + value.len();
                    for piece in &complement {
                        let start = piece.start.max(offset);
                        let end = piece.end.min(event_end);
                        if start < end {
                            let relative = start - offset;
                            escape_edit(
                                &mut edits,
                                prose,
                                range.start + relative..range.start + relative + (end - start),
                                &value[relative..relative + (end - start)],
                            );
                        }
                    }
                    offset = event_end;
                }
                index = scan - 1;
            }
            Event::Html(chunk) | Event::InlineHtml(chunk) => {
                if let Some(converted) = convert_html_comments(chunk) {
                    edits.push(Edit {
                        range,
                        priority: PRIORITY_INLINE,
                        text: converted,
                    });
                }
            }
            _ => {}
        }
        index += 1;
    }

    wrap_sections(prose, &headings, root_id, &mut edits, used_ids, &fail)?;
    Ok(apply_edits(prose, edits))
}

/// Insert semantic section wrappers around top-level headings deeper than the
/// fragment's own H1, with stable `root--slug` chain IDs and no silent reuse.
fn wrap_sections(
    prose: &str,
    headings: &[SourceHeading],
    root_id: &str,
    edits: &mut Vec<Edit>,
    used: &mut HashSet<String>,
    fail: &dyn Fn(String) -> FragmentError,
) -> Result<(), FragmentError> {
    let mut stack: Vec<(u8, String)> = Vec::new();
    for heading in headings {
        if !heading.top_level || heading.depth == 1 {
            continue;
        }
        let mut insertion = String::new();
        let mut closers = 0usize;
        while stack
            .last()
            .is_some_and(|(depth, _)| *depth >= heading.depth)
        {
            stack.pop();
            closers += 1;
        }
        insertion.push_str(sep_before(prose, heading.start));
        for closer in 0..closers {
            if closer > 0 {
                insertion.push_str("\n\n");
            }
            insertion.push_str("</section>");
        }
        let own = slug(&heading.text);
        if own.is_empty() {
            return Err(fail(format!(
                "heading \"{}\" needs text: nested section ids are derived from heading text",
                heading.text.trim()
            )));
        }
        let mut id = String::from(root_id);
        for (_, ancestor) in &stack {
            id.push_str("--");
            id.push_str(ancestor);
        }
        id.push_str("--");
        id.push_str(&own);
        if !used.insert(id.clone()) {
            return Err(fail(format!(
                "generated section id \"{id}\" is duplicated; use distinct heading text and source anchor ids"
            )));
        }
        if closers > 0 {
            insertion.push_str("\n\n");
        }
        insertion.push_str(&format!("<section id=\"{id}\">\n\n"));
        stack.push((heading.depth, own));
        edits.push(Edit {
            range: heading.start..heading.start,
            priority: PRIORITY_SECTION,
            text: insertion,
        });
    }
    let mut tail = String::new();
    for _ in 0..stack.len() {
        tail.push_str("\n\n</section>");
    }
    if !tail.is_empty() {
        edits.push(Edit {
            range: prose.len()..prose.len(),
            priority: PRIORITY_SECTION,
            text: tail,
        });
    }
    Ok(())
}

fn apply_edits(prose: &str, mut edits: Vec<Edit>) -> String {
    edits.sort_by_key(|edit| (edit.range.start, edit.priority));
    let added: usize = edits.iter().map(|edit| edit.text.len()).sum();
    let mut out = String::with_capacity(prose.len() + added);
    let mut cursor = 0usize;
    for edit in edits {
        // An outer replacement (autolink) subsumes inner edits.
        if edit.range.start < cursor {
            continue;
        }
        out.push_str(&prose[cursor..edit.range.start]);
        out.push_str(&edit.text);
        cursor = edit.range.end;
    }
    out.push_str(&prose[cursor..]);
    out
}

/// Escape one text segment as an inline edit, if escaping changes anything.
/// A consumed source escape backslash immediately before the segment is
/// absorbed into the edit, so an escape the parser consumed is not emitted
/// twice; an even run (escaped backslash) is left alone.
fn escape_edit(edits: &mut Vec<Edit>, prose: &str, mut range: Range<usize>, segment: &str) {
    if let Some(escaped) = escape_mdx_text(segment) {
        if escaped.starts_with('\\') {
            let bytes = prose.as_bytes();
            let mut backslashes = 0usize;
            let mut index = range.start;
            while index > 0 && bytes[index - 1] == b'\\' {
                backslashes += 1;
                index -= 1;
            }
            if backslashes % 2 == 1 {
                range.start = index;
            }
        }
        edits.push(Edit {
            range,
            priority: PRIORITY_INLINE,
            text: escaped,
        });
    }
}

/// Index of the End event matching the Start at `start`; the balanced event
/// stream returns depth to zero exactly at the matching End.
fn matching_end(events: &[(Event<'_>, Range<usize>)], start: usize) -> usize {
    let mut depth = 0usize;
    for (index, (event, _)) in events.iter().enumerate().skip(start) {
        match event {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
            }
            _ => {}
        }
    }
    unreachable!("parsed event streams are balanced")
}

fn heading_text(events: &[(Event<'_>, Range<usize>)], start: usize, end: usize) -> String {
    let mut text = String::new();
    for (event, _) in &events[start + 1..end] {
        match event {
            Event::Text(inner) | Event::Code(inner) => text.push_str(inner),
            _ => {}
        }
    }
    text
}

/// Ensure a blank line before an inserted HTML block without adding extra
/// blank lines when the source already has one.
fn sep_before(prose: &str, position: usize) -> &'static str {
    if position == 0 {
        return "";
    }
    let before = &prose[..position];
    if before.ends_with("\n\n") {
        ""
    } else if before.ends_with('\n') {
        "\n"
    } else {
        "\n\n"
    }
}

/// Deterministic heading slug: lowercased (Unicode-aware), spaces to hyphens,
/// other punctuation dropped, remaining letters and digits preserved.
pub(crate) fn slug(text: &str) -> String {
    let mut slug = String::new();
    for character in text.trim().chars() {
        match character {
            'a'..='z' | '0'..='9' | '-' | '_' => slug.push(character),
            'A'..='Z' => slug.push(character.to_ascii_lowercase()),
            c if c.is_whitespace() => slug.push('-'),
            c if c.is_alphanumeric() => slug.extend(c.to_lowercase()),
            _ => {}
        }
    }
    slug
}

/// MDX reads `{`/`}` as expression delimiters and `<` before a word-ish
/// character as a tag; escape exactly those inside Markdown text, plus a
/// backslash that would otherwise swallow the following punctuation escape.
/// Parsers can emit `<` as its own one-character event, so an event-final `<`
/// is escaped too — `\<` renders `<` unconditionally, making over-escaping
/// render-safe where a lookahead is impossible.
fn escape_mdx_text(text: &str) -> Option<String> {
    let mut out = String::new();
    let mut changed = false;
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '{' | '}' => {
                out.push('\\');
                out.push(character);
                changed = true;
            }
            '\\' if characters
                .peek()
                .is_some_and(|next| next.is_ascii_punctuation()) =>
            {
                out.push('\\');
                out.push('\\');
                changed = true;
            }
            '<' if characters
                .peek()
                .is_none_or(|next| matches!(next, 'a'..='z' | 'A'..='Z' | '/' | '!' | '?')) =>
            {
                out.push('\\');
                out.push('<');
                changed = true;
            }
            _ => out.push(character),
        }
    }
    changed.then_some(out)
}

/// Find complete static `<Okf data={...} />` elements in merged Markdown text
/// and return their spans (offsets relative to the merged text; error context
/// uses `base`) so they are preserved verbatim instead of escaped as prose
/// (the wire contract's subsection records). Anything `<Okf`-shaped that is
/// not a valid static data element is an explicit error, never mangled.
fn static_okf_spans(text: &str, base: usize) -> Result<Vec<Range<usize>>, String> {
    let mut spans = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = text[scan..].find("<Okf") {
        let start = scan + found;
        let problem = |detail: String| format!("byte {}: {detail}", base + start);
        let rest = &text[start + 4..];
        let whitespace = rest.len() - rest.trim_start().len();
        if whitespace == 0 {
            return Err(problem(
                "\"<Okf\" must be an element with a data attribute".into(),
            ));
        }
        let rest = rest[whitespace..].trim_start();
        let Some(rest) = rest.strip_prefix("data=") else {
            return Err(problem("Okf requires exactly one data attribute".into()));
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('{') else {
            return Err(problem(
                "Okf data requires a JSON object inside JSX braces".into(),
            ));
        };
        let length = expression_length(rest, 0).map_err(problem)?;
        let json = &rest[..length];
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|error| problem(format!("Okf data must be literal JSON: {error}")))?;
        if !value.is_object() {
            return Err(problem("Okf data must be a JSON object".into()));
        }
        let rest = rest[length..].trim_start();
        let Some(rest) = rest.strip_prefix('}') else {
            return Err(problem("Okf data expression must close with \"}\"".into()));
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix("/>") else {
            return Err(problem("Okf element must close with \"/>\"".into()));
        };
        let end = text.len() - rest.len();
        spans.push(start..end);
        scan = end;
    }
    Ok(spans)
}

/// Length of the JSX expression (a JSON value) between the container braces,
/// respecting string literals so braces inside strings never miscount; the
/// container's own closing brace terminates the expression.
fn expression_length(source: &str, offset: usize) -> Result<usize, String> {
    let bytes = source.as_bytes();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for (relative, byte) in bytes[offset..].iter().enumerate() {
        if in_string {
            match byte {
                b'\\' => escaped = !escaped,
                b'"' if !escaped => in_string = false,
                _ => escaped = false,
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                if depth == 0 {
                    return Ok(relative);
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    Err("expression braces never close".into())
}

/// HTML comments are not JSX; a closed comment becomes an identical JSX block
/// comment. Comments containing `*/` cannot be expressed and stay verbatim.
fn convert_html_comments(chunk: &str) -> Option<String> {
    if !chunk.contains("<!--") {
        return None;
    }
    let mut out = String::with_capacity(chunk.len());
    let mut rest = chunk;
    let mut changed = false;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        match after.find("-->") {
            Some(end) => {
                let comment = &after[..end + 3];
                let body = &comment[4..comment.len() - 3];
                if body.contains("*/") {
                    out.push_str(comment);
                } else {
                    out.push_str("{/*");
                    out.push_str(body);
                    out.push_str("*/}");
                    changed = true;
                }
                rest = &after[end + 3..];
            }
            None => {
                out.push_str(after);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    changed.then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_body(prose: &str, root: &str, path: &str) -> Result<String, FragmentError> {
        super::build_body(prose, root, path, &mut HashSet::new())
    }

    fn body(prose: &str, root: &str) -> String {
        build_body(prose, root, "t.md").unwrap()
    }

    #[test]
    fn shifts_atx_setext_and_trailing_hash_headings() {
        assert_eq!(body("# One\n\n# Only", "r"), "## One\n\n## Only");
        assert_eq!(
            body("# Doc\n\nTitle\n=====\n\nafter", "r"),
            "## Doc\n\n## Title\n\nafter"
        );
        assert_eq!(
            body("# Doc\n\nSub\n---\n\nafter", "r"),
            "## Doc\n\n<section id=\"r--sub\">\n\n### Sub\n\nafter\n\n</section>"
        );
        assert_eq!(body("# Closed ##\n\nafter", "r"), "## Closed ##\n\nafter");
    }

    #[test]
    fn wraps_nested_sections_with_stable_ids_and_closers() {
        let out = body("# Doc\n\nintro\n\n## A\n\na\n\n### B\n\nb\n\ntail", "x");
        let expected = "## Doc\n\nintro\n\n<section id=\"x--a\">\n\n### A\n\na\n\n\
                        <section id=\"x--a--b\">\n\n#### B\n\nb\n\ntail\n\n</section>\n\n</section>";
        assert_eq!(out, expected);
    }

    #[test]
    fn closes_sibling_sections_between_headings() {
        let out = body("# Doc\n\n## A\n\na\n\n## B\n\nb", "x");
        assert_eq!(
            out,
            "## Doc\n\n<section id=\"x--a\">\n\n### A\n\na\n\n</section>\n\n\
             <section id=\"x--b\">\n\n### B\n\nb\n\n</section>"
        );
    }

    #[test]
    fn code_fences_anchors_and_definition_lists_survive_verbatim() {
        let prose = "# D\n\n<dl>\n<dt><dfn id=\"term\">T</dfn></dt>\n<dd>{kept} <b>bold</b></dd>\n</dl>\n\n```sh\n# not a heading\necho \"{raw}\"\n```\n";
        let out = body(prose, "x");
        assert!(out.contains("<dfn id=\"term\">T</dfn>"));
        assert!(out.contains("<dd>{kept} <b>bold</b></dd>"));
        assert!(out.contains("# not a heading\necho \"{raw}\""));
        assert!(!out.contains("\\{"));
    }

    #[test]
    fn escapes_braces_tags_and_converts_comments_and_autolinks() {
        // Parsers can emit `<` as its own one-character event; event-final
        // `<` is escaped render-safely.
        let prose = "# D\n\na {x} and a <b look and < ok and \\{kept\\}\n\n<!-- note {fine} -->\n\n<https://e.x/p>\n";
        let out = body(prose, "x");
        assert!(
            out.contains("a \\{x\\} and a \\<b look and \\< ok and \\{kept\\}"),
            "{out}"
        );
        assert!(out.contains("{/* note {fine} */}"));
        assert!(out.contains("[https://e.x/p](https://e.x/p)"));
        // Comments convert; a comment containing `*/` cannot and stays verbatim.
        assert_eq!(
            body("# D\n\na */ b <!-- c -->\n", "r"),
            "## D\n\na */ b {/* c */}\n"
        );
        assert_eq!(
            body("# D\n\n<!-- keep {raw} */ now -->\n", "r"),
            "## D\n\n<!-- keep {raw} */ now -->\n"
        );
    }

    #[test]
    fn preserves_static_okf_elements_and_rejects_fake_ones() {
        let element = r#"<Okf data={{"type":"Reference","title":"Local terms"}} />"#;
        let out = body(&format!("# T\n\n{element} then {{x}}\n"), "code--t");
        // The authored element is preserved verbatim; only the prose braces
        // around it are escaped.
        assert_eq!(out, format!("## T\n\n{element} then ") + "\\{x\\}\n");

        // Fake Okf in prose is an explicit error, never mangled or escaped.
        let error =
            build_body("# T\n\n<Okf data={{type: not json}} />\n", "r", "t.md").unwrap_err();
        assert!(error.to_string().contains("static Okf element"), "{error}");
        let error = build_body("# T\n\n<Okf data={\"not object\"} />\n", "r", "t.md").unwrap_err();
        assert!(error.to_string().contains("JSON object"), "{error}");
        let error = build_body("# T\n\n<Okf data={\"unfinished\n", "r", "t.md").unwrap_err();
        assert!(error.to_string().contains("static Okf element"), "{error}");

        // Fake Okf inside comments and code stays opaque after conversion.
        let out = body(
            "# T\n\n<!-- fake <Okf data={{not: json}} /> -->\n\n```sh\n<Okf data={{not: json}} />\n```\n",
            "r",
        );
        assert!(
            out.contains("{/* fake <Okf data={{not: json}} /> */}"),
            "{out}"
        );
        assert!(
            out.contains("```sh\n<Okf data={{not: json}} />\n```"),
            "{out}"
        );
    }

    #[test]
    fn shifts_headings_inside_containers_without_wrapping_them() {
        let out = body("# D\n\n> ## Quoted\n\nafter", "x");
        assert_eq!(out, "## D\n\n> ### Quoted\n\nafter");
    }

    #[test]
    fn duplicate_slugs_and_h6_fail_loudly() {
        let error = build_body("# D\n\n## Same\n\n## Same\n", "x", "t.md").unwrap_err();
        assert!(error.to_string().contains("duplicated"), "{error}");
        let error = build_body("# D\n\n###### Six\n", "x", "t.md").unwrap_err();
        assert!(error.to_string().contains("H6"), "{error}");
    }

    #[test]
    fn slug_rules_are_deterministic() {
        assert_eq!(slug("Terms in use"), "terms-in-use");
        assert_eq!(slug("  Mixed CASE-dash  "), "mixed-case-dash");
        assert_eq!(slug("Über cool"), "über-cool");
        assert_eq!(slug("###"), "");
    }

    #[test]
    fn escaping_covers_backslashes_before_punctuation() {
        let once = escape_mdx_text("a {b} <c < d").unwrap();
        assert_eq!(once, "a \\{b\\} \\<c < d");
        assert!(escape_mdx_text("plain").is_none());
        // A literal backslash before punctuation must be doubled, or the
        // emitted `\{` would swallow it as an escape.
        assert_eq!(escape_mdx_text("back\\{x").unwrap(), "back\\\\\\{x");
        assert_eq!(escape_mdx_text("pre\\*x").unwrap(), "pre\\\\*x");
        assert!(escape_mdx_text("path\\to\\file").is_none());
    }
}
