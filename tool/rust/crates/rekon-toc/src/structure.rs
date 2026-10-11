//! Document structure extraction: headings, section context, explicit IDs,
//! anchor blocks, and the managed `<nav id="rekon-toc">` region.

use std::ops::RangeInclusive;

use markdown::mdast::{AttributeContent, AttributeValue, Node};

use crate::TocError;

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
    /// Every literal `id` attribute seen on JSX elements, first-seen order,
    /// deduplicated. Used for conflict detection and duplicate reporting.
    pub explicit_ids: Vec<String>,
    /// Explicit ids defined more than once in the document.
    pub duplicate_ids: Vec<String>,
}

/// A `<nav id="rekon-toc">` occurrence, flow or inline (text) JSX.
#[derive(Debug, Clone)]
struct ManagedNav {
    start_line: usize,
    end_line: usize,
    /// The element owns its start and end lines: only whitespace surrounds
    /// its byte span on both. Wholesale line replacement would otherwise
    /// drop prose, container prefixes (`> `, `- `), or adjacent content.
    isolated: bool,
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
    let line_starts = line_start_offsets(text);
    let mut walker = Walker {
        text,
        line_starts: &line_starts,
        ..Walker::default()
    };
    walker.visit(&tree, parts);

    // The first managed nav defines the region and must own its lines.
    // Further occurrences outside the region are errors — ambiguous if
    // isolated, destructive if not.
    let region = match walker.managed_navs.first() {
        None => None,
        Some(first) if !first.isolated => {
            return Err(TocError::UnisolatedNav {
                line: first.start_line,
            });
        }
        Some(first) => Some(first.start_line..=first.end_line),
    };
    if let Some(region) = &region {
        for nav in &walker.managed_navs[1..] {
            if region.contains(&nav.start_line) {
                continue; // managed content, regenerated wholesale
            }
            return Err(if nav.isolated {
                TocError::MultipleManagedNavs {
                    first: *region.start(),
                    extra_lines: vec![nav.start_line],
                }
            } else {
                TocError::UnisolatedNav {
                    line: nav.start_line,
                }
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

/// Byte offset where each 1-based line starts; `line_starts[n - 1]` is the
/// start of line `n`, and `line_starts.get(n)` (or `text.len()`) ends it.
fn line_start_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (idx, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            offsets.push(idx + 1);
        }
    }
    offsets
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
struct Walker<'a> {
    text: &'a str,
    line_starts: &'a [usize],
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
    managed_navs: Vec<ManagedNav>,
}

impl Walker<'_> {
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
            Node::MdxJsxFlowElement(el) => {
                let id = literal_id(&el.attributes);
                if let Some(id) = &id {
                    self.record_id(id);
                }
                match el.name.as_deref() {
                    Some("nav") if id.as_deref() == Some(NAV_ID) => {
                        self.record_managed_nav(start_line, end_line, pos);
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
            // Inline JSX: a managed nav inside prose must still be caught —
            // recording it as a region candidate makes its (missing)
            // line isolation an explicit error instead of a silent second
            // region or a dropped-prose replacement.
            Node::MdxJsxTextElement(el) => {
                let id = literal_id(&el.attributes);
                if let Some(id) = &id {
                    self.record_id(id);
                }
                if el.name.as_deref() == Some("nav") && id.as_deref() == Some(NAV_ID) {
                    self.record_managed_nav(start_line, end_line, pos);
                }
                for child in &el.children {
                    self.visit(child, parts);
                }
            }
            // Raw-HTML constructs are off in the reader profile; Node::Html
            // appears only in the comment probe above.
            Node::Html(_) => {}
            // Frontmatter and ESM blocks are opaque literals.
            Node::Yaml(_) | Node::Toml(_) | Node::MdxjsEsm(_) => {}
            _ => {
                if let Some(children) = node.children() {
                    for child in children {
                        self.visit(child, parts);
                    }
                }
            }
        }
    }

    fn record_managed_nav(
        &mut self,
        start_line: Option<usize>,
        end_line: Option<usize>,
        pos: Option<&markdown::unist::Position>,
    ) {
        let Some(((start, end), position)) = start_line.zip(end_line).zip(pos) else {
            return;
        };
        let line_start = self.line_starts[start - 1];
        let line_end = self
            .line_starts
            .get(end)
            .copied()
            .unwrap_or(self.text.len());
        let isolated = self.text[line_start..position.start.offset]
            .trim()
            .is_empty()
            && self.text[position.end.offset..line_end].trim().is_empty();
        self.managed_navs.push(ManagedNav {
            start_line: start,
            end_line: end,
            isolated,
        });
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
