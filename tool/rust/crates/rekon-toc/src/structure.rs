//! Document structure extraction: headings, section context, explicit IDs,
//! anchor blocks, and the managed `<nav id="rekon-toc">` region.

use std::ops::RangeInclusive;

use markdown::mdast::{AttributeContent, AttributeValue, Node};

use crate::TocError;
use crate::html_scan::{only_tags_and_whitespace, scan_tags};

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
    /// `id` of the closest enclosing `<section>`, if any.
    pub section: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Structure {
    /// Every real heading in document order (fences/frontmatter excluded by
    /// the parser itself).
    pub headings: Vec<RawHeading>,
    /// Line span (1-based, inclusive) of an existing managed nav region.
    pub nav_region: Option<RangeInclusive<usize>>,
    /// End line of the first H1 outside the managed region — the insertion
    /// point for a fresh nav. `None` inserts at the top.
    pub insertion_after: Option<usize>,
    /// Every literal `id=` attribute seen in HTML blocks and JSX elements,
    /// first-seen order, deduplicated. Used to reserve slugs and report
    /// duplicates.
    pub explicit_ids: Vec<String>,
    /// Explicit ids defined more than once in the document.
    pub duplicate_ids: Vec<String>,
}

pub(crate) fn extract(text: &str, parts: &[&str]) -> Result<Structure, TocError> {
    let tree = markdown::to_mdast(text, &crate::parse_options())
        .map_err(|m| TocError::Parse(m.to_string()))?;
    let mut walker = Walker::default();
    walker.visit(&tree, parts);

    let mut structure = Structure {
        headings: walker.headings,
        nav_region: None,
        insertion_after: None,
        explicit_ids: walker.explicit_ids,
        duplicate_ids: walker.duplicate_ids,
    };

    // Resolve the managed region.
    let region = if let Some((start, end)) = walker.nav_element_region {
        Some(start..=end)
    } else if let Some(open) = walker.nav_open_line {
        let close = walker.nav_close_line.or_else(|| {
            // Fallback: line-scan for `</nav>` when the close tag was swallowed
            // by another HTML block (no blank line separated it).
            parts
                .iter()
                .skip(open)
                .position(|l| l.trim_start().starts_with("</nav"))
                .map(|idx| open + idx + 1)
        });
        match close {
            Some(close) => Some(open..=close),
            None => return Err(TocError::UnclosedNav { line: open }),
        }
    } else {
        None
    };
    structure.nav_region = region;

    // First H1 outside the region is the insertion anchor.
    structure.insertion_after = walker
        .first_h1
        .filter(|(_, end)| {
            structure
                .nav_region
                .as_ref()
                .is_none_or(|r| !r.contains(end))
        })
        .map(|(_, end)| end);

    // Drop headings recorded inside the managed region (managed content).
    if let Some(region) = structure.nav_region.clone() {
        structure.headings.retain(|h| !region.contains(&h.line));
    }
    Ok(structure)
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
    /// `<nav id="rekon-toc">` parsed as a JSX container: exact node span.
    nav_element_region: Option<(usize, usize)>,
    /// Line of a raw-HTML `<nav id="rekon-toc">` open tag.
    nav_open_line: Option<usize>,
    /// End line of the matching raw-HTML `</nav>` block.
    nav_close_line: Option<usize>,
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
                let anchor = self.pending_anchor.take().map(|(id, _)| id);
                let section = self.sections.iter().rev().find_map(|s| s.clone());
                self.headings.push(RawHeading {
                    level: h.depth,
                    title: inline_text(node),
                    line: start_line.unwrap_or(0),
                    end_line: end_line.unwrap_or(start_line.unwrap_or(0)),
                    anchor,
                    section,
                });
            }
            Node::Html(html) => self.visit_html(html.value.as_str(), start_line, end_line),
            Node::MdxJsxFlowElement(el) => {
                let id = literal_id(&el.attributes);
                if let Some(id) = &id {
                    self.record_id(id);
                }
                match el.name.as_deref() {
                    Some("nav") if id.as_deref() == Some(NAV_ID) => {
                        if self.nav_open_line.is_none() && self.nav_element_region.is_none() {
                            self.nav_element_region = start_line.zip(end_line);
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
            _ => {
                if let Some(children) = node.children() {
                    for child in children {
                        self.visit(child, parts);
                    }
                }
            }
        }
    }

    fn visit_html(&mut self, value: &str, start_line: Option<usize>, end_line: Option<usize>) {
        let tags = scan_tags(value);
        if let Some(first) = tags.first() {
            if !first.closing
                && first.name == "nav"
                && first.id.as_deref() == Some(NAV_ID)
                && !first.self_closing
                && self.nav_open_line.is_none()
                && self.nav_element_region.is_none()
            {
                self.nav_open_line = start_line;
            }
            if first.closing
                && first.name == "nav"
                && self.nav_close_line.is_none()
                && self.nav_open_line.is_some()
            {
                self.nav_close_line = end_line;
            }
        }
        for tag in &tags {
            if let Some(id) = &tag.id {
                self.record_id(id);
            }
        }
        for tag in &tags {
            if tag.name == "section" {
                if tag.closing {
                    self.sections.pop();
                } else if !tag.self_closing {
                    self.sections.push(tag.id.clone());
                }
            }
        }
        // Anchor block: a single `<a id=…>` (with optional closing tag) and
        // nothing but whitespace around it.
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
            && only_tags_and_whitespace(value)
            && let (Some(id), Some(end)) = (tags[0].id.clone(), end_line)
        {
            self.pending_anchor = Some((id, end));
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
