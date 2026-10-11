//! rekon-toc — table-of-contents generation and managed-region updating for
//! Rekon Markdown/MDX documents.
//!
//! The library parses a whole document with markdown-rs using the strict JSX
//! reader profile (see [`parse_options`]) — the same grammar rekon-markup's
//! writer validates, with HTML comments masked first the same way — so both
//! crates agree on one document grammar. Static MDX is opaque structure, not
//! text to be re-scanned: `<Okf data={…} />` and strings inside its JSON
//! never contribute tags, ids, or headings; `{/* … */}` comments are opaque
//! expressions. From that tree the library builds the heading hierarchy and
//! renders a semantic managed region:
//!
//! ```md
//! # Document
//!
//! <nav id="rekon-toc" aria-label="Table of contents">
//!
//! - [Section](#section)
//!   - [Subsection](#subsection)
//!
//! </nav>
//! ```
//!
//! [`update_toc`] inserts the region after the document's first H1 (or at the
//! top when there is none) and regenerates it in place afterwards. The
//! operation is pure (text in, text out), deterministic, and idempotent:
//! updating an already-updated document returns identical bytes, including
//! when line ranges are enabled — entry coordinates are computed on the final
//! layout, because inline numbers never change the region's line count. The
//! managed region is the parser-confirmed `<nav id="rekon-toc">` element
//! span: a `</nav>` inside fenced code cannot close it, a second managed nav
//! is an error rather than a silent choice, and prose outside the element is
//! never touched.
//!
//! Link targets prefer real document structure over generated slugs: an
//! explicit empty-anchor id (`<a id="…"></a>`) bound to the heading, then the
//! closest enclosing `<section id="…">`, then the heading's renderer slug —
//! the id a Markdown renderer assigns it. Renderer slugs are allocated across
//! *every* real heading in document order (the renderer anchors unlisted
//! H1s and explicitly-targeted headings too), and a slug that would collide
//! with an unrelated explicit id — or with the managed nav id — is rejected
//! as a source-repair problem rather than silently shifted to a nonexistent
//! target. Two coincidences are valid: the slug may equal the heading's own
//! bound anchor id or its enclosing section id, where the explicit target is
//! preferred.
//!
//! Headings inside code fences, YAML/TOML frontmatter, JSX expression
//! comments, and the managed nav region itself are never listed. Line
//! coordinates are whole-file, physical, 1-based — frontmatter lines count.
//!
//! # Example
//!
//! ```
//! use rekon_toc::{update_toc, TocOptions};
//!
//! let updated = update_toc("# Title\n\n## Alpha\n\ntext\n", &TocOptions::default()).unwrap();
//! assert!(updated.contains("<nav id=\"rekon-toc\""));
//! assert!(updated.contains("- [Alpha](#alpha)"));
//! // Idempotent:
//! assert_eq!(update_toc(&updated, &TocOptions::default()).unwrap(), updated);
//! ```
//!
//! No cleanup or pruning is performed: a document whose headings all fall
//! outside the configured depth window keeps (or gets, when a region already
//! exists) an empty region rather than having one removed.

mod render;
mod slug;
mod structure;

use std::collections::HashSet;

use markdown::{Constructs, ParseOptions};
use render::RenderEntry;

pub(crate) use slug::{escape_html_attr, escape_html_text, escape_label};

/// `id` of the managed ToC region.
pub const NAV_ID: &str = structure::NAV_ID;

/// How a ToC entry's link target was chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetKind {
    /// `id` of an empty anchor block immediately before the heading.
    ExplicitAnchor,
    /// `id` of the closest enclosing `<section>`.
    EnclosingSection,
    /// Generated GitHub-style heading slug (possibly disambiguated).
    GeneratedSlug,
}

/// Options for ToC generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocOptions {
    /// Smallest heading level listed. Default 2 (document H1 stays out).
    pub min_depth: u8,
    /// Largest heading level listed. Default 6.
    pub max_depth: u8,
    /// Emit `data-line-start`/`data-line-end` attributes on entry anchors,
    /// pointing at 1-based whole-file line ranges of the final document.
    /// Default false.
    pub line_ranges: bool,
    /// Accessible label for the nav region. Default "Table of contents".
    pub aria_label: String,
}

impl Default for TocOptions {
    fn default() -> Self {
        Self {
            min_depth: 2,
            max_depth: 6,
            line_ranges: false,
            aria_label: "Table of contents".to_owned(),
        }
    }
}

impl TocOptions {
    /// Options matching the planned `rekon toc` CLI defaults with line ranges
    /// enabled: headings 2..=6 plus final-document line ranges.
    pub fn with_line_ranges() -> Self {
        Self {
            line_ranges: true,
            ..Self::default()
        }
    }
}

/// A listed ToC entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocEntry {
    pub level: u8,
    pub title: String,
    pub target: String,
    pub target_kind: TargetKind,
    /// 1-based line of the heading in the *original* text (before any splice).
    pub line: usize,
}

/// Structural analysis of a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocAnalysis {
    /// Entries within the depth window, in document order, with resolved
    /// targets and original-document line coordinates.
    pub entries: Vec<TocEntry>,
    /// Every literal id in the document, first-seen order, deduplicated
    /// (sections, anchors, `<dfn>` terms, metadata elements).
    pub explicit_ids: Vec<String>,
    /// Literal ids defined more than once in the document (document problem;
    /// reported, not rewritten).
    pub duplicate_ids: Vec<String>,
    /// 1-based inclusive line span of the existing managed nav region.
    pub nav_region: Option<std::ops::RangeInclusive<usize>>,
    /// End line of the first H1 (the insertion point for a fresh region),
    /// or `None` when the region would be inserted at the top.
    pub insertion_after_line: Option<usize>,
}

/// Drift check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocCheck {
    /// True when the document already carries exactly the managed region
    /// [`update_toc`] would produce.
    pub up_to_date: bool,
    /// The regenerated document; compare with the input to see the drift.
    pub updated: String,
}

/// Errors from ToC generation.
#[derive(Debug, thiserror::Error)]
pub enum TocError {
    /// The document failed to parse as GFM+frontmatter Markdown with MDX
    /// expressions. Unbalanced `{`/`}` in prose and unclosed JSX elements
    /// (including an unclosed `<nav id="rekon-toc">`) fail here — wrap such
    /// text in code spans or fences, or close the element.
    #[error("markdown parse failed: {0}")]
    Parse(String),
    /// `min_depth`/`max_depth` are outside 1..=6 or inverted.
    #[error("invalid heading depth window: min {min}, max {max}")]
    InvalidDepth { min: u8, max: u8 },
    /// A managed nav does not own its lines: prose before or after it on
    /// the same line, or a container prefix (blockquote `> `, list `- `),
    /// would be dropped by wholesale region replacement. Embedding the nav
    /// in prose also cannot define a region. Keep the element on its own
    /// lines.
    #[error("managed nav at line {line} shares its line with other content")]
    UnisolatedNav { line: usize },
    /// More than one managed nav region exists; ToC would silently operate
    /// on one of them.
    #[error("multiple managed nav regions: first at line {first}, others at {extra_lines:?}")]
    MultipleManagedNavs {
        first: usize,
        extra_lines: Vec<usize>,
    },
    /// A heading's renderer slug collides with an explicit id elsewhere in
    /// the document (or with the managed nav id). The rendered document
    /// would carry duplicate ids, and shifting the slug would emit a
    /// nonexistent target. Repair the source: rename the heading, change
    /// the explicit id, or bind an explicit anchor to the heading.
    #[error(
        "heading at line {heading_line} would render with id {slug:?}, which already exists as an explicit id; repair the source rather than retargeting silently"
    )]
    ConflictingIds { slug: String, heading_line: usize },
}

/// The Rekon Markdown/MDX reader profile, converged with rekon-markup's
/// strict JSX grammar on the same upstream markdown-rs parser: GFM plus
/// frontmatter, MDX expressions, and JSX, with raw-HTML constructs off. Tags
/// are JSX elements — `<Okf data={…} />` and its JSON strings stay opaque
/// attribute expressions (an HTML-looking string inside metadata is string
/// content, never structural inline HTML), `{/* … */}` comments are opaque
/// expressions, and `<section>`/`<nav>` nest as real containers. Documents
/// with unbalanced braces in prose or unbalanced raw HTML fail the parse —
/// an honest MDX error, not silent misreading. A custom expression validator
/// extends the default brace counter to respect JSON strings and block
/// comments, so `{"a":"}"}`-shaped metadata parses.
pub fn parse_options() -> ParseOptions {
    let mut constructs = Constructs::gfm();
    constructs.frontmatter = true;
    constructs.mdx_expression_flow = true;
    constructs.mdx_expression_text = true;
    constructs.mdx_jsx_flow = true;
    constructs.mdx_jsx_text = true;
    constructs.html_flow = false;
    constructs.html_text = false;
    ParseOptions {
        constructs,
        mdx_expression_parse: Some(Box::new(|value, _kind| validate_expression(value))),
        ..ParseOptions::default()
    }
}

/// Accept an expression iff its braces balance when JSON strings (with
/// escapes) and `/* … */` block comments are respected. Strictly more
/// accepting than markdown-rs's default brace counter, which mis-ends
/// expressions like `{"a":"}"}`.
fn validate_expression(value: &str) -> markdown::MdxSignal {
    use markdown::MdxSignal;
    let source: Box<String> = Box::new("rekon-toc".to_string());
    let rule: Box<String> = Box::new("balanced-expression".to_string());
    let chars: Vec<char> = value.chars().collect();
    let mut depth: i64 = 0;
    let mut in_string = false;
    let mut escaped = false;
    let mut in_comment = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_comment {
            if c == '*' && chars.get(i + 1) == Some(&'/') {
                in_comment = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match c {
            '/' if chars.get(i + 1) == Some(&'*') => {
                in_comment = true;
                i += 2;
                continue;
            }
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth < 0 {
                    return MdxSignal::Error(
                        "closing brace without an opening brace in expression".to_string(),
                        i,
                        source,
                        rule,
                    );
                }
            }
            _ => {}
        }
        i += 1;
    }
    if in_string || in_comment || depth > 0 {
        return MdxSignal::Eof(
            "unterminated string, comment, or open brace in expression".to_string(),
            source,
            rule,
        );
    }
    MdxSignal::Ok
}

/// Analyze a document's heading structure without modifying it.
pub fn analyze(text: &str, options: &TocOptions) -> Result<TocAnalysis, TocError> {
    validate(options)?;
    let parts: Vec<&str> = text.split('\n').collect();
    let structure = structure::extract(text, &parts)?;
    let entries = resolve_targets(&structure, options)?
        .into_iter()
        .map(|e| TocEntry {
            level: e.level,
            title: e.title,
            target: e.target,
            target_kind: e.kind,
            line: e.line,
        })
        .collect();
    Ok(TocAnalysis {
        entries,
        explicit_ids: structure.explicit_ids,
        duplicate_ids: structure.duplicate_ids,
        nav_region: structure.nav_region,
        insertion_after_line: structure.insertion_after,
    })
}

/// Insert or regenerate the managed `<nav id="rekon-toc">` region.
///
/// Pure: returns the updated document. With no existing region and no entries
/// in the depth window the input is returned unchanged.
pub fn update_toc(text: &str, options: &TocOptions) -> Result<String, TocError> {
    validate(options)?;
    let parts: Vec<&str> = text.split('\n').collect();
    let structure = structure::extract(text, &parts)?;
    let resolved = resolve_targets(&structure, options)?;
    if structure.nav_region.is_none() && resolved.is_empty() {
        return Ok(text.to_owned());
    }

    // The rendered region's line count is independent of the inline line
    // numbers, so plan the splice with placeholders, map coordinates onto the
    // final layout, then render the real region into the reserved slot.
    let placeholders: Vec<RenderEntry> = resolved
        .iter()
        .map(|e| RenderEntry {
            level: e.level,
            label: e.title.clone(),
            target: e.target.clone(),
            start: 0,
            end: 0,
        })
        .collect();
    let nav_len = render::render_nav(&placeholders, options).len();

    let mut new_parts: Vec<String> = Vec::new();

    let boundary: usize; // original lines <= boundary keep their position

    let rest: &[&str];
    match (structure.nav_region.clone(), structure.insertion_after) {
        (Some(region), _) => {
            let (r0, r1) = (*region.start(), *region.end());
            new_parts.extend(parts[..r0 - 1].iter().map(|s| (*s).to_owned()));
            let sep_before = usize::from(r0 > 1 && !parts[r0 - 2].trim().is_empty());
            for _ in 0..sep_before {
                new_parts.push(String::new());
            }
            rest = &parts[r1..];
            boundary = r1;
        }
        (None, insertion) => {
            let at = insertion.unwrap_or(0);
            new_parts.extend(parts[..at].iter().map(|s| (*s).to_owned()));
            let sep_before = usize::from(at > 0 && !parts[at - 1].trim().is_empty());
            for _ in 0..sep_before {
                new_parts.push(String::new());
            }
            rest = &parts[at..];
            boundary = at;
        }
    }
    // Reserve the nav slot, then the separator that keeps following content
    // off the region's heels.
    let nav_at: usize = new_parts.len();
    for _ in 0..nav_len {
        new_parts.push(String::new());
    }
    let sep_after = usize::from(!rest.is_empty() && !rest[0].trim().is_empty());
    for _ in 0..sep_after {
        new_parts.push(String::new());
    }
    // Original lines after the splice boundary shift by the layout delta.
    let post_delta: i64 = new_parts.len() as i64 - boundary as i64;

    // Map entry coordinates onto the final layout and render the real region.
    let final_line = |line: usize| -> usize {
        if line <= boundary {
            line
        } else {
            (line as i64 + post_delta) as usize
        }
    };
    let starts: Vec<usize> = resolved.iter().map(|e| final_line(e.line)).collect();
    // A trailing empty part from the final newline is a phantom, not a line;
    // our own emission always ends on `</nav>` or a separator line.
    let phantom = usize::from(rest.last().is_some_and(|l| l.is_empty()));
    let total_real = new_parts.len() + rest.len() - phantom;
    let mut with_ends: Vec<RenderEntry> = resolved
        .iter()
        .zip(&starts)
        .map(|(e, start)| RenderEntry {
            level: e.level,
            label: e.title.clone(),
            target: e.target.clone(),
            start: *start,
            end: 0,
        })
        .collect();
    for (idx, entry) in with_ends.iter_mut().enumerate() {
        entry.end = match starts.get(idx + 1) {
            Some(next_start) => *next_start - 1,
            None => total_real,
        };
    }
    let nav_lines = render::render_nav(&with_ends, options);
    debug_assert_eq!(nav_lines.len(), nav_len);
    for (idx, line) in nav_lines.into_iter().enumerate() {
        new_parts[nav_at + idx] = line;
    }
    new_parts.extend(rest.iter().map(|s| (*s).to_owned()));
    Ok(new_parts.join("\n"))
}

/// Check a document for ToC drift.
pub fn check_toc(text: &str, options: &TocOptions) -> Result<TocCheck, TocError> {
    let updated = update_toc(text, options)?;
    let up_to_date = updated == text;
    Ok(TocCheck {
        up_to_date,
        updated,
    })
}

fn validate(options: &TocOptions) -> Result<(), TocError> {
    let valid = (1..=6).contains(&options.min_depth)
        && (1..=6).contains(&options.max_depth)
        && options.min_depth <= options.max_depth;
    if valid {
        Ok(())
    } else {
        Err(TocError::InvalidDepth {
            min: options.min_depth,
            max: options.max_depth,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedEntry {
    level: u8,
    title: String,
    target: String,
    kind: TargetKind,
    line: usize,
}

/// Choose link targets for every listed heading.
///
/// Two-phase, modeling what a Markdown renderer actually emits:
///
/// 1. **Renderer slugs** are allocated across *every* real heading in
///    document order — including unlisted H1s, headings outside the depth
///    window, and headings that will get an explicit target. The renderer
///    anchors those headings regardless of the ToC, so `# Same` followed by
///    `## Same` must link the H2 to `#same-1`.
/// 2. **Conflicts are rejected, not worked around.** If a heading's
///    renderer slug equals an explicit id elsewhere in the document (or the
///    managed nav id), the rendered document would carry duplicate ids, and
///    a silently shifted slug would point at a nonexistent anchor. Two
///    coincidences are valid and exempt: the slug may equal the heading's
///    own bound anchor id or its enclosing section id — both anchor the same
///    spot, and the ToC prefers the explicit target.
/// 3. Listed headings then pick, once each: bound anchor id, enclosing
///    section id, renderer slug. A preference already claimed by an earlier
///    heading falls through to the next.
fn resolve_targets(
    structure: &structure::Structure,
    options: &TocOptions,
) -> Result<Vec<ResolvedEntry>, TocError> {
    // Phase 1: renderer slug per heading (headings only compete with
    // headings; empty-title headings render without an anchor).
    let explicit: HashSet<&str> = structure.explicit_ids.iter().map(String::as_str).collect();
    let mut slug_counts: std::collections::HashMap<String, usize> = Default::default();
    let renderer_slugs: Vec<Option<String>> = structure
        .headings
        .iter()
        .map(|heading| {
            let title = heading.title.trim();
            if title.is_empty() {
                return None;
            }
            let base = slug::slugify(title);
            let seen = slug_counts.entry(base.clone()).or_default();
            *seen += 1;
            Some(if *seen == 1 {
                base
            } else {
                format!("{base}-{}", *seen - 1)
            })
        })
        .collect();

    // Phase 2: reject implicit/explicit id conflicts needing source repair.
    for (heading, slug) in structure.headings.iter().zip(&renderer_slugs) {
        let Some(slug) = slug else { continue };
        let conflicts = slug == NAV_ID
            || (explicit.contains(slug.as_str())
                && heading.anchor.as_deref() != Some(slug.as_str())
                && heading.section.as_deref() != Some(slug.as_str()));
        if conflicts {
            return Err(TocError::ConflictingIds {
                slug: slug.clone(),
                heading_line: heading.line,
            });
        }
    }

    // Phase 3: allocate ToC targets for listed headings.
    let mut claimed: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for (heading, slug) in structure.headings.iter().zip(&renderer_slugs) {
        if heading.title.trim().is_empty()
            || heading.level < options.min_depth
            || heading.level > options.max_depth
        {
            continue;
        }
        let mut chosen: Option<(String, TargetKind)> = None;
        if let Some(anchor) = &heading.anchor
            && !claimed.contains(anchor)
        {
            chosen = Some((anchor.clone(), TargetKind::ExplicitAnchor));
        }
        if chosen.is_none()
            && let Some(section) = &heading.section
            && !claimed.contains(section)
        {
            chosen = Some((section.clone(), TargetKind::EnclosingSection));
        }
        let (target, kind) = match chosen {
            Some((target, kind)) => (target, kind),
            None => (
                slug.clone()
                    .expect("non-empty headings have renderer slugs"),
                TargetKind::GeneratedSlug,
            ),
        };
        claimed.insert(target.clone());
        out.push(ResolvedEntry {
            level: heading.level,
            title: heading.title.trim().to_owned(),
            target,
            kind,
            line: heading.line,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_validation() {
        assert!(validate(&TocOptions::default()).is_ok());
        assert!(
            validate(&TocOptions {
                min_depth: 3,
                max_depth: 3,
                ..TocOptions::default()
            })
            .is_ok()
        );
        assert!(matches!(
            validate(&TocOptions {
                min_depth: 4,
                max_depth: 2,
                ..TocOptions::default()
            }),
            Err(TocError::InvalidDepth { min: 4, max: 2 })
        ));
        assert!(
            validate(&TocOptions {
                min_depth: 0,
                max_depth: 6,
                ..TocOptions::default()
            })
            .is_err()
        );
        assert!(
            validate(&TocOptions {
                min_depth: 2,
                max_depth: 7,
                ..TocOptions::default()
            })
            .is_err()
        );
    }

    #[test]
    fn no_headings_no_region_is_noop() {
        let text = "Just a paragraph.\n\nAnother one.\n";
        assert_eq!(update_toc(text, &TocOptions::default()).unwrap(), text);
    }
}
