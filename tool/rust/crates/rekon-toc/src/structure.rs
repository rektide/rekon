//! Document structure extraction: headings, section context, explicit IDs,
//! anchor blocks, and the managed `<nav id="rekon-toc">` region.

use std::ops::RangeInclusive;

use markdown::mdast::{AttributeContent, AttributeValue, Node};

use crate::TocError;
use crate::html_scan::{Tag, only_tags_and_whitespace, scan_tags_at};

pub(crate) const NAV_ID: &str = "rekon-toc";

/// A heading as it appears in the source, before target resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawHeading {
    pub level: u8,
    pub title: String,
    /// 1-based start line in the original text.
    pub line: usize,
    /// 1-based end line in the original text (setext headings span two lines).
    pub end_line: usize,
    /// `id` of an immediately preceding empty anchor block, if any.
    pub anchor: Option<String>,
    /// Line of that anchor block (managed-region filtering).
    pub anchor_line: Option<usize>,
    /// `id` of the closest enclosing `<section>`, if any.
    pub section: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Structure {
    /// Every real heading in document order (fences, frontmatter, and managed
    /// region content are excluded by the parser or by extraction).
    pub headings: Vec<RawHeading>,
    /// Line span (1-based, inclusive) of the existing managed nav region.
    pub nav_region: Option<RangeInclusive<usize>>,
    /// End line of the first H1 outside the managed region — the insertion
    /// point for a fresh nav. `None` inserts at the top.
    pub insertion_after: Option<usize>,
    /// Every literal `id=` attribute seen in HTML blocks and JSX elements,
    /// first-seen order, deduplicated. Used for conflict detection and
    /// duplicate reporting.
    pub explicit_ids: Vec<String>,
    /// Explicit ids defined more than once in the document.
    pub duplicate_ids: Vec<String>,
}

pub(crate) fn extract(text: &str, parts: &[&str]) -> Result<Structure, TocError> {
    // MDX does not accept HTML comments. Recognize them with a permissive
    // probe parse first, then blank them without moving any byte — the
    // strict JSX parse (and all line positions) see the identical layout,
    // while comment content can never contribute tags, ids, or headings.
    // Comments inside fences or frontmatter are Code/Yaml nodes to the probe
    // and are left untouched.
    let masked = mask_html_comments(text)?;
    let tree = markdown::to_mdast(&masked, &crate::parse_options())
        .map_err(|m| TocError::Parse(m.to_string()))?;
    let mut walker = Walker::default();
    walker.visit(&tree, parts);

    let region = if let Some((start, end)) = walker.managed_nav_spans.first() {
        Some(*start..=*end)
    } else if let Some(&open) = walker.managed_open_lines.first() {
        match match_nav_close(&walker.html_stream, open) {
            Some(close) => Some(open..=close),
            None => return Err(TocError::UnclosedNav { line: open }),
        }
    } else {
        None
    };

    if let Some(region) = &region {
        let (r0, r1) = (*region.start(), *region.end());
        if !walker.managed_nav_spans.is_empty() {
            // JSX-container form: the parser gave the element's exact span;
            // replacing it wholesale can only drop managed content.
        } else {
            // Raw-HTML form: the managed markers must own their lines, or a
            // wholesale region replacement would drop prose sharing them.
            let open_tags = tags_on_line(&walker.html_stream, r0);
            if r0 == r1 {
                let ok = open_tags.len() == 2
                    && open_tags[0].name == "nav"
                    && !open_tags[0].closing
                    && open_tags[0].id.as_deref() == Some(NAV_ID)
                    && open_tags[1].name == "nav"
                    && open_tags[1].closing
                    && only_tags_and_whitespace(parts[r0 - 1]);
                if !ok {
                    return Err(TocError::UnisolatedNav { line: r0 });
                }
            } else {
                let close_tags = tags_on_line(&walker.html_stream, r1);
                let open_ok = open_tags.len() == 1
                    && open_tags[0].name == "nav"
                    && !open_tags[0].closing
                    && open_tags[0].id.as_deref() == Some(NAV_ID)
                    && only_tags_and_whitespace(parts[r0 - 1]);
                let close_ok = close_tags.len() == 1
                    && close_tags[0].name == "nav"
                    && close_tags[0].closing
                    && only_tags_and_whitespace(parts[r1 - 1]);
                if !open_ok || !close_ok {
                    return Err(TocError::UnisolatedNav {
                        line: if !open_ok { r0 } else { r1 },
                    });
                }
            }
        }
        // A second managed nav outside this region is ambiguous.
        let extras: Vec<usize> = walker
            .managed_open_lines
            .iter()
            .chain(walker.managed_nav_spans.iter().map(|(start, _)| start))
            .filter(|&&line| !region.contains(&line))
            .copied()
            .collect();
        if !extras.is_empty() {
            return Err(TocError::MultipleManagedNavs {
                first: *region.start(),
                extra_lines: extras,
            });
        }
    }

    let mut structure = Structure {
        headings: walker.headings,
        nav_region: region,
        insertion_after: None,
        explicit_ids: walker.explicit_ids,
        duplicate_ids: walker.duplicate_ids,
    };

    // First H1 outside the region is the insertion anchor; headings and
    // anchor bindings from inside the managed region are managed content.
    if let Some(region) = structure.nav_region.clone() {
        structure.insertion_after = walker
            .first_h1
            .filter(|(_, end)| !region.contains(end))
            .map(|(_, end)| end);
        for heading in &mut structure.headings {
            if heading
                .anchor_line
                .is_some_and(|line| region.contains(&line))
            {
                heading.anchor = None;
                heading.anchor_line = None;
            }
        }
    } else {
        structure.insertion_after = walker.first_h1.map(|(_, end)| end);
    }
    if let Some(region) = structure.nav_region.clone() {
        structure.headings.retain(|h| !region.contains(&h.line));
    }
    Ok(structure)
}

/// First line of a `</nav>` that matches the managed open at `open_line`,
/// counting nested `nav` opens/closes through the tag stream.
fn match_nav_close(stream: &[Tag], open_line: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut started = false;
    for tag in stream {
        if tag.name != "nav" {
            continue;
        }
        if !tag.closing {
            if tag.line == open_line && !started {
                started = true;
                depth = 1;
                continue;
            }
            if started {
                depth += 1;
            }
        } else if started {
            depth -= 1;
            if depth == 0 {
                return Some(tag.line);
            }
        }
    }
    None
}

fn tags_on_line(stream: &[Tag], line: usize) -> Vec<&Tag> {
    stream.iter().filter(|t| t.line == line).collect()
}

/// Blank out HTML comment blocks in `text`, preserving length, newlines, and
/// every other byte so later positions map 1:1 onto the original.
fn mask_html_comments(text: &str) -> Result<String, TocError> {
    let probe_options = {
        let mut constructs = markdown::Constructs::gfm();
        constructs.frontmatter = true;
        markdown::ParseOptions {
            constructs,
            ..markdown::ParseOptions::default()
        }
    };
    let probe =
        markdown::to_mdast(text, &probe_options).map_err(|m| TocError::Parse(m.to_string()))?;
    let mut spans: Vec<std::ops::Range<usize>> = Vec::new();
    collect_comment_spans(&probe, &mut spans);
    let mut masked = text.to_owned();
    for span in spans {
        let blank: String = masked[span.clone()]
            .bytes()
            .map(|byte| match byte {
                b'\n' | b'\r' => char::from(byte),
                _ => ' ',
            })
            .collect();
        masked.replace_range(span, &blank);
    }
    Ok(masked)
}

fn collect_comment_spans(node: &Node, spans: &mut Vec<std::ops::Range<usize>>) {
    if let Node::Html(html) = node
        && html.value.trim().starts_with("<!--")
        && html.value.trim().ends_with("-->")
        && let Some(position) = node.position()
    {
        spans.push(position.start.offset..position.end.offset);
    }
    if let Some(children) = node.children() {
        for child in children {
            collect_comment_spans(child, spans);
        }
    }
}

#[derive(Default)]
struct Walker {
    headings: Vec<RawHeading>,
    /// Innermost-last stack of enclosing sections; `None` = `<section>` with
    /// no id.
    sections: Vec<Option<String>>,
    /// Empty anchor block waiting to bind to the next heading.
    pending_anchor: Option<(String, usize)>,
    explicit_ids: Vec<String>,
    seen_ids: std::collections::HashSet<String>,
    duplicate_ids: Vec<String>,
    first_h1: Option<(usize, usize)>,
    /// Every raw-HTML tag in document order, from parser-isolated blocks,
    /// with absolute lines.
    html_stream: Vec<Tag>,
    /// Lines of raw-HTML managed `<nav id="rekon-toc">` open tags.
    managed_open_lines: Vec<usize>,
    /// Spans of JSX-container managed navs (element start..=end).
    managed_nav_spans: Vec<(usize, usize)>,
}

impl Walker {
    fn visit(&mut self, node: &Node, parts: &[&str]) {
        let pos = node.position();
        let start_line = pos.map(|p| p.start.line);
        let end_line = pos.map(|p| p.end.line);
        self.invalidate_pending_anchor(start_line, parts);

        match node {
            Node::Heading(h) => {
                if h.depth == 1 && self.first_h1.is_none() {
                    self.first_h1 = start_line.zip(end_line);
                }
                let (anchor, anchor_line) = match self.pending_anchor.take() {
                    Some((id, line)) => (Some(id), Some(line)),
                    None => (None, None),
                };
                let section = self.sections.iter().rev().find_map(|s| s.clone());
                self.headings.push(RawHeading {
                    level: h.depth,
                    title: inline_text(node),
                    line: start_line.unwrap_or(0),
                    end_line: end_line.unwrap_or(start_line.unwrap_or(0)),
                    anchor,
                    anchor_line,
                    section,
                });
            }
            Node::Html(html) => {
                if let Some(start) = start_line {
                    let tags = scan_tags_at(html.value.as_str(), start);
                    if let Some(first) = tags.first()
                        && !first.closing
                        && first.name == "nav"
                        && first.id.as_deref() == Some(NAV_ID)
                        && !first.self_closing
                    {
                        self.managed_open_lines.push(first.line);
                    }
                    for tag in &tags {
                        if let Some(id) = &tag.id {
                            self.record_id(id);
                        }
                    }
                    // Section stack updates apply after this block (HTML
                    // blocks cannot contain headings).
                    for tag in &tags {
                        if tag.name == "section" {
                            if tag.closing {
                                self.sections.pop();
                            } else if !tag.self_closing {
                                self.sections.push(tag.id.clone());
                            }
                        }
                    }
                    // Anchor block: a single `<a id=…>` (with optional closing
                    // tag) and nothing but whitespace around it.
                    let anchor_shape = match tags.as_slice() {
                        [open] => open.name == "a" && !open.closing && open.id.is_some(),
                        [open, close] => {
                            open.name == "a"
                                && !open.closing
                                && open.id.is_some()
                                && close.closing
                                && close.name == "a"
                        }
                        _ => false,
                    };
                    if anchor_shape
                        && only_tags_and_whitespace(html.value.as_str())
                        && let (Some(id), Some(end)) = (tags[0].id.clone(), end_line)
                    {
                        self.pending_anchor = Some((id, end));
                    }
                    self.html_stream.extend(tags);
                }
            }
            Node::MdxJsxFlowElement(el) => {
                let id = literal_id(&el.attributes);
                if let Some(id) = &id {
                    self.record_id(id);
                }
                match el.name.as_deref() {
                    Some("nav") if id.as_deref() == Some(NAV_ID) => {
                        if let Some((start, end)) = start_line.zip(end_line) {
                            self.managed_nav_spans.push((start, end));
                        }
                        // Managed content: do not walk inside.
                    }
                    Some("a") if id.is_some() && el.children.is_empty() => {
                        if let (Some(id), Some(end)) = (id, end_line) {
                            self.pending_anchor = Some((id, end));
                        }
                    }
                    Some("section") => {
                        self.sections.push(id);
                        for child in &el.children {
                            self.visit(child, parts);
                        }
                        self.sections.pop();
                    }
                    _ => {
                        for child in &el.children {
                            self.visit(child, parts);
                        }
                    }
                }
            }
            // Frontmatter and ESM blocks are opaque literals.
            Node::Yaml(_) | Node::Toml(_) | Node::MdxjsEsm(_) => {}
            // Inline JSX (e.g. a nested <dfn> on a <dt> line): record its
            // literal ids, then walk phrasing children.
            Node::MdxJsxTextElement(el) => {
                if let Some(id) = literal_id(&el.attributes) {
                    self.record_id(&id);
                }
                for child in &el.children {
                    self.visit(child, parts);
                }
            }
            _ => {
                if let Some(children) = node.children() {
                    for child in children {
                        self.visit(child, parts);
                    }
                }
            }
        }
    }

    /// An anchor binds only to a heading that follows it with nothing but
    /// blank lines between; any other block in between invalidates it.
    fn invalidate_pending_anchor(&mut self, start_line: Option<usize>, parts: &[&str]) {
        let Some((_, anchor_end)) = self.pending_anchor.clone() else {
            return;
        };
        let Some(start) = start_line else {
            self.pending_anchor = None;
            return;
        };
        if start <= anchor_end {
            return;
        }
        // Lines anchor_end+1 ..= start-1 must all be blank
        // (1-based lines → 0-based indices anchor_end..start-1).
        let blank = parts[anchor_end..start - 1]
            .iter()
            .all(|l| l.trim().is_empty());
        if !blank {
            self.pending_anchor = None;
        }
    }

    fn record_id(&mut self, id: &str) {
        if self.seen_ids.insert(id.to_owned()) {
            self.explicit_ids.push(id.to_owned());
        } else if !self.duplicate_ids.iter().any(|d| d == id) {
            self.duplicate_ids.push(id.to_owned());
        }
    }
}

/// Plain text of a heading: `Text` and inline code contents, image alt text,
/// recursing through phrasing containers. MDX expressions and raw inline HTML
/// contribute nothing.
fn inline_text(node: &Node) -> String {
    match node {
        Node::Text(t) => t.value.clone(),
        Node::InlineCode(c) => c.value.clone(),
        Node::Image(i) => i.alt.clone(),
        _ => node
            .children()
            .map(|children| children.iter().map(inline_text).collect())
            .unwrap_or_default(),
    }
}

/// Literal `id` attribute of a JSX element, when statically knowable.
fn literal_id(attributes: &[AttributeContent]) -> Option<String> {
    attributes.iter().find_map(|attr| match attr {
        AttributeContent::Property(prop) if prop.name == "id" => match &prop.value {
            Some(AttributeValue::Literal(value)) => Some(value.clone()),
            _ => None,
        },
        _ => None,
    })
}
