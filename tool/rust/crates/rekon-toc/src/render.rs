//! Nav-region emission.

use crate::{TocOptions, escape_html_attr, escape_html_text, escape_label};

/// One rendered ToC line's data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RenderEntry {
    pub level: u8,
    pub label: String,
    pub target: String,
    /// Final-document line coordinates (1-based); placeholders during splice
    /// planning.
    pub start: usize,
    pub end: usize,
}

/// Render the managed nav block as lines (no trailing newline).
///
/// Shape:
///
/// ```text
/// <nav id="rekon-toc" aria-label="…">
///
/// - [Title](#target)
///   - [Nested](#nested-target)
///
/// </nav>
/// ```
///
/// With `line_ranges`, each entry is an HTML anchor carrying standard
/// `data-line-start`/`data-line-end` attributes instead of a Markdown link.
/// The entry lines are always plain Markdown list items so the region stays
/// readable and diffable; inline numbers never change the line count.
pub(crate) fn render_nav(entries: &[RenderEntry], options: &TocOptions) -> Vec<String> {
    let mut lines = vec![format!(
        "<nav id=\"{}\" aria-label=\"{}\">",
        crate::structure::NAV_ID,
        escape_html_attr(&options.aria_label)
    )];
    lines.push(String::new());
    if !entries.is_empty() {
        for entry in entries {
            let indent = "  ".repeat(entry.level.saturating_sub(options.min_depth) as usize);
            if options.line_ranges {
                lines.push(format!(
                    "{indent}- <a href=\"#{}\" data-line-start=\"{}\" data-line-end=\"{}\">{}</a>",
                    entry.target,
                    entry.start,
                    entry.end,
                    escape_html_text(&entry.label)
                ));
            } else {
                lines.push(format!(
                    "{indent}- [{}](#{})",
                    escape_label(&entry.label),
                    entry.target
                ));
            }
        }
        lines.push(String::new());
    }
    lines.push("</nav>".to_owned());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> TocOptions {
        TocOptions::default()
    }

    #[test]
    fn markdown_links_by_default() {
        let entries = vec![
            RenderEntry {
                level: 2,
                label: "Alpha".into(),
                target: "alpha".into(),
                start: 0,
                end: 0,
            },
            RenderEntry {
                level: 3,
                label: "Alpha [deep]".into(),
                target: "alpha-deep".into(),
                start: 0,
                end: 0,
            },
        ];
        assert_eq!(
            render_nav(&entries, &opts()),
            vec![
                "<nav id=\"rekon-toc\" aria-label=\"Table of contents\">",
                "",
                "- [Alpha](#alpha)",
                "  - [Alpha \\[deep\\]](#alpha-deep)",
                "",
                "</nav>",
            ]
        );
    }

    #[test]
    fn html_anchors_with_line_ranges() {
        let options = TocOptions {
            line_ranges: true,
            ..TocOptions::default()
        };
        let entries = vec![RenderEntry {
            level: 2,
            label: "A & <b>".into(),
            target: "a-b".into(),
            start: 12,
            end: 40,
        }];
        assert_eq!(
            render_nav(&entries, &options),
            vec![
                "<nav id=\"rekon-toc\" aria-label=\"Table of contents\">",
                "",
                "- <a href=\"#a-b\" data-line-start=\"12\" data-line-end=\"40\">A &amp; &lt;b&gt;</a>",
                "",
                "</nav>",
            ]
        );
    }

    #[test]
    fn empty_region_is_three_lines() {
        assert_eq!(
            render_nav(&[], &opts()),
            vec![
                "<nav id=\"rekon-toc\" aria-label=\"Table of contents\">",
                "",
                "</nav>",
            ]
        );
    }
}
