//! Document-level tests: enriched sections, managed-region replacement,
//! idempotence, final line-range accuracy, and parser-hardening cases.

use rekon_toc::{TargetKind, TocCheck, TocError, TocOptions, analyze, check_toc, update_toc};

fn default_opts() -> TocOptions {
    TocOptions::default()
}

fn ranges_opts() -> TocOptions {
    TocOptions::with_line_ranges()
}

/// Update twice; the second pass must be byte-identical (stable final bytes).
fn assert_idempotent(text: &str, options: &TocOptions) -> String {
    let once = update_toc(text, options).expect("first update");
    let twice = update_toc(&once, options).expect("second update");
    assert_eq!(once, twice, "update_toc must be idempotent");
    once
}

/// Extract `(target, start, end)` triples from the data attributes of a
/// rendered nav region.
fn data_ranges(doc: &str) -> Vec<(String, usize, usize)> {
    let mut out = Vec::new();
    for line in doc.lines() {
        let Some(rest) = line.trim_start().strip_prefix("- <a href=\"#") else {
            continue;
        };
        let Some(href_end) = rest.find('"') else {
            continue;
        };
        let target = rest[..href_end].to_owned();
        let rest = &rest[href_end..];
        let start = attribute(rest, "data-line-start");
        let end = attribute(rest, "data-line-end");
        out.push((target, start, end));
    }
    out
}

fn attribute(text: &str, name: &str) -> usize {
    let needle = format!("{name}=\"");
    let at = text.find(&needle).expect("attribute present") + needle.len();
    let end = text[at..].find('"').expect("attribute closed") + at;
    text[at..end].parse().expect("numeric attribute")
}

#[test]
fn plain_markdown_golden() {
    let input = "# Plain\n\n## Alpha\n\nalpha text\n\n### Alpha nested\n\n## Beta\n";
    let updated = assert_idempotent(input, &default_opts());
    let expected = "\
# Plain

<nav id=\"rekon-toc\" aria-label=\"Table of contents\">

- [Alpha](#alpha)
  - [Alpha nested](#alpha-nested)
- [Beta](#beta)

</nav>

## Alpha

alpha text

### Alpha nested

## Beta
";
    assert_eq!(updated, expected);
}

#[test]
fn enriched_sections_okf_and_dfn() {
    let input = "\
---

<section id=\"fragment-terms\">

<Okf data={{\"type\":\"Practice\",\"title\":\"Terms\"}} />

## Code

<dl>
<dt><dfn id=\"checkpoint\">Checkpoint</dfn></dt>
<dd>A preserved state.</dd>
</dl>

<section id=\"fragment-terms--extra\">

### Extra terms

</section>
</section>

<a id=\"explicit-heading\"></a>

## Explicitly anchored

## Deploy {env}

## Checkpoint

## Checkpoint

```md
## Fenced fake

<nav id=\"rekon-toc\">
- not managed content
```
";
    let updated = assert_idempotent(input, &ranges_opts());

    // Target preference: enclosing section ids, bound anchor id, then slugs.
    // `checkpoint` is reserved by the <dfn> id, so the duplicate headings
    // disambiguate to checkpoint-1 and checkpoint-2.
    let analysis = analyze(&updated, &ranges_opts()).expect("analyze");
    let targets: Vec<(&str, TargetKind)> = analysis
        .entries
        .iter()
        .map(|e| (e.target.as_str(), e.target_kind))
        .collect();
    assert_eq!(
        targets,
        vec![
            ("fragment-terms", TargetKind::EnclosingSection),
            ("fragment-terms--extra", TargetKind::EnclosingSection),
            ("explicit-heading", TargetKind::ExplicitAnchor),
            ("deploy-env", TargetKind::GeneratedSlug),
            ("checkpoint-1", TargetKind::GeneratedSlug),
            ("checkpoint-2", TargetKind::GeneratedSlug),
        ]
    );

    // Line ranges must point at the final bytes: each start is the heading's
    // actual line, each end is one before the next entry's heading.
    let ranges = data_ranges(&updated);
    let lines: Vec<&str> = updated.lines().collect();
    for (target, start, _end) in &ranges {
        let line = lines
            .get(start - 1)
            .unwrap_or_else(|| panic!("start {start} out of range for {target}"));
        assert!(
            line.starts_with('#'),
            "data-line-start {start} for {target} must be a heading line, got {line:?}"
        );
    }
    let starts: Vec<usize> = ranges.iter().map(|(_, s, _)| *s).collect();
    for pair in starts.windows(2) {
        assert!(pair[1] > pair[0], "entries must be ordered");
    }
    for (idx, (_, _start, end)) in ranges.iter().enumerate() {
        if idx + 1 < ranges.len() {
            assert_eq!(*end, ranges[idx + 1].1 - 1, "end is next start minus one");
        }
    }
    let last_end = ranges.last().unwrap().2;
    assert_eq!(last_end, updated.lines().count(), "last range runs to EOF");
}

#[test]
fn fenced_frontmatter_and_nav_fakes_are_ignored() {
    let input = "\
---
type: Practice
# fake: heading in yaml
---

# Real title

```md
## Fenced fake one
```

    ## Indented fake

~~~
## Fenced fake two

<nav id=\"rekon-toc\">
- would be managed if real
</nav>
~~~

## Real section

```md
</nav>
```
";
    let updated = assert_idempotent(input, &default_opts());
    let analysis = analyze(&updated, &default_opts()).expect("analyze");
    let titles: Vec<&str> = analysis.entries.iter().map(|e| e.title.as_str()).collect();
    assert_eq!(titles, vec!["Real section"]);
    // The fenced fake nav is not a managed region: a fresh one was inserted
    // after the H1 rather than replacing anything.
    assert!(
        analysis.nav_region.is_none() || {
            // If a region exists it must be the freshly inserted one.
            updated.matches("<nav id=\"rekon-toc\"").count() == 2
        }
    );
    assert!(updated.contains("- [Real section](#real-section)"));
}

#[test]
fn stale_region_is_replaced_and_check_detects_drift() {
    let stale = "\
# Doc

<nav id=\"rekon-toc\" aria-label=\"Table of contents\">

- [Gone](#gone)

</nav>

## Present
";
    let check = check_toc(stale, &default_opts()).expect("check");
    assert!(!check.up_to_date, "stale region must be drift");

    let updated = update_toc(stale, &default_opts()).expect("update");
    assert!(
        !updated.contains("Gone"),
        "old entries are regenerated away"
    );
    assert!(updated.contains("- [Present](#present)"));
    assert_eq!(updated.matches("<nav id=\"rekon-toc\"").count(), 1);

    let check = check_toc(&updated, &default_opts()).expect("check");
    assert!(check.up_to_date, "fresh region is drift-free");
    let TocCheck {
        up_to_date,
        updated: again,
    } = check;
    assert!(up_to_date);
    assert_eq!(again, updated);
}

#[test]
fn empty_window_keeps_empty_region() {
    let input = "\
# Doc

<nav id=\"rekon-toc\" aria-label=\"Table of contents\">

- [Listed](#listed)

</nav>

## Listed
";
    let options = TocOptions {
        min_depth: 4,
        max_depth: 6,
        ..TocOptions::default()
    };
    let updated = assert_idempotent(input, &options);
    let nav_start = updated.find("<nav id=\"rekon-toc\"").expect("region kept");
    let nav_end =
        updated[nav_start..].find("</nav>").expect("region closed") + nav_start + "</nav>".len();
    let region = &updated[nav_start..nav_end];
    assert_eq!(
        region, "<nav id=\"rekon-toc\" aria-label=\"Table of contents\">\n\n</nav>",
        "region is empty but present"
    );
    assert!(!updated.contains("[Listed]"));
}

#[test]
fn unicode_and_escaping() {
    let input = "\
# Doc

## 名前と住所

## Über — Straße ✨

## [Brackets] and `code`

## See [the guide](/guide.md)

## Multiple  spaces
";
    let updated = assert_idempotent(input, &default_opts());
    assert!(updated.contains("- [名前と住所](#名前と住所)"));
    assert!(updated.contains("- [Über — Straße ✨](#über--straße-)"));
    assert!(updated.contains("- [\\[Brackets\\] and code](#brackets-and-code)"));
    assert!(updated.contains("- [See the guide](#see-the-guide)"));
    assert!(updated.contains("- [Multiple  spaces](#multiple--spaces)"));
}

#[test]
fn anchor_forms_bind_to_next_heading() {
    let input = concat!(
        "# Doc\n",
        "\n",
        r#"<a id="jsx-anchor"></a>"#,
        "\n",
        "\n",
        "## First\n",
        "\n",
        r#"<a id="html-anchor" />"#,
        "\n",
        "\n",
        "## Second\n",
        "\n",
        r#"<a id="stolen-anchor"></a>"#,
        "\n",
        "\n",
        "Interposing paragraph breaks the bind.\n",
        "\n",
        "## Third\n",
    );
    let analysis = analyze(input, &default_opts()).expect("analyze");
    let targets: Vec<(&str, TargetKind)> = analysis
        .entries
        .iter()
        .map(|e| (e.target.as_str(), e.target_kind))
        .collect();
    assert_eq!(
        targets,
        vec![
            ("jsx-anchor", TargetKind::ExplicitAnchor),
            ("html-anchor", TargetKind::ExplicitAnchor),
            ("third", TargetKind::GeneratedSlug),
        ]
    );
}

#[test]
fn setext_headings_with_min_depth_one() {
    let input = "\
Setext title
============

Body text.

Sub heading
-----------
";
    let options = TocOptions {
        min_depth: 1,
        ..TocOptions::default()
    };
    let updated = assert_idempotent(input, &options);
    assert!(updated.contains("- [Setext title](#setext-title)"));
    assert!(updated.contains("  - [Sub heading](#sub-heading)"));
    // Insertion happens after the two-line setext H1.
    let nav_line = updated
        .lines()
        .position(|l| l.starts_with("<nav id=\"rekon-toc\""))
        .expect("nav");
    assert_eq!(nav_line, 3, "nav follows the setext H1 plus a blank line");
}

#[test]
fn duplicate_slugs_disambiguate_deterministically() {
    let input = "# Doc\n\n## Same\n\n## Same\n\n## Same\n";
    let updated = assert_idempotent(input, &default_opts());
    assert!(updated.contains("- [Same](#same)"));
    assert!(updated.contains("- [Same](#same-1)"));
    assert!(updated.contains("- [Same](#same-2)"));
}

#[test]
fn no_trailing_newline_is_preserved() {
    let input = "# Doc\n\n## Alpha\n\nbody";
    let updated = assert_idempotent(input, &default_opts());
    assert!(!updated.ends_with('\n'), "no newline invented");
    assert!(updated.ends_with("body"));
}

#[test]
fn region_at_end_of_document() {
    let input = "# Doc\n\n## Alpha\n\nbody\n\n<nav id=\"rekon-toc\" aria-label=\"Table of contents\">\n\n- [Stale](#stale)\n\n</nav>\n";
    let updated = assert_idempotent(input, &default_opts());
    assert!(
        updated.ends_with("</nav>\n"),
        "region replaced in place at EOF"
    );
    assert!(updated.contains("- [Alpha](#alpha)"));
}

#[test]
fn invalid_depth_window_is_rejected() {
    let options = TocOptions {
        min_depth: 5,
        max_depth: 2,
        ..TocOptions::default()
    };
    assert!(matches!(
        update_toc("# D\n\n## H\n", &options),
        Err(TocError::InvalidDepth { .. })
    ));
}

#[test]
fn unclosed_nav_region_is_an_error() {
    let input = "# Doc\n\n<nav id=\"rekon-toc\">\n\n## Heading\n";
    assert!(matches!(
        update_toc(input, &default_opts()),
        Err(TocError::UnclosedNav { line: 3 })
    ));
}

#[test]
fn prose_braces_stay_text() {
    // MDX expressions are not enabled in the tolerant reader profile: an
    // unbalanced brace in prose must not fail ToC generation, and brace text
    // stays part of the heading title.
    let input = "# Doc\n\nOops { unclosed brace in prose\n\n## Deploy {env}\n";
    let updated = update_toc(input, &default_opts()).expect("tolerant of prose braces");
    assert!(updated.contains("- [Deploy {env}](#deploy-env)"));
    assert_idempotent(input, &default_opts());
}

#[test]
fn aria_label_and_depth_options_shape_output() {
    let input = "# Doc\n\n## Two\n\n#### Four\n";
    let options = TocOptions {
        min_depth: 2,
        max_depth: 2,
        aria_label: "Contents".to_owned(),
        ..TocOptions::default()
    };
    let updated = assert_idempotent(input, &options);
    assert!(updated.contains("aria-label=\"Contents\""));
    assert!(updated.contains("- [Two](#two)"));
    assert!(!updated.contains("[Four]"));
}
