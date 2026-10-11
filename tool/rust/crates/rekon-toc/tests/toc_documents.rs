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
    let input = concat!(
        "---\n",
        "type: Practice\n",
        "---\n",
        "\n",
        "# Enriched\n",
        "\n",
        r#"<section id="fragment-terms">"#,
        "\n",
        "\n",
        r#"<Okf data={{"type":"Practice","title":"Terms"}} />"#,
        "\n",
        "\n",
        "## Code\n",
        "\n",
        "<dl>\n",
        r#"<dt><dfn id="checkpoint">Checkpoint</dfn></dt>"#,
        "\n",
        "<dd>A preserved state.</dd>\n",
        "</dl>\n",
        "\n",
        r#"<section id="fragment-terms--extra">"#,
        "\n",
        "\n",
        "### Extra terms\n",
        "\n",
        "</section>\n",
        "</section>\n",
        "\n",
        r#"<a id="explicit-heading"></a>"#,
        "\n",
        "\n",
        "## Explicitly anchored\n",
        "\n",
        "## Deploy {env}\n",
        "\n",
        "## Checkpoint term\n",
        "\n",
        "```md\n",
        "## Fenced fake\n",
        "\n",
        r#"<nav id="rekon-toc">"#,
        "\n",
        "- not managed content\n",
        "```\n",
    );
    let updated = assert_idempotent(input, &ranges_opts());

    // Target preference: enclosing section ids, bound anchor id, then slugs.
    // `{env}` is an MDX expression: the heading title is "Deploy". The dfn id
    // "checkpoint" does not collide with the "Checkpoint term" slug.
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
            ("deploy", TargetKind::GeneratedSlug),
            ("checkpoint-term", TargetKind::GeneratedSlug),
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
fn slug_allocation_covers_all_real_headings() {
    // The renderer anchors every heading, listed or not: the excluded H1
    // consumes `#same`, so the listed H2 is `#same-1`.
    let input = "# Same\n\n## Same\n";
    let updated = assert_idempotent(input, &default_opts());
    assert!(updated.contains("- [Same](#same-1)"));
    assert!(!updated.contains("](#same)"));

    // Headings that get explicit or section targets still consume their
    // renderer slug: a later same-titled heading anchors one later.
    let input = concat!(
        r#"<a id="exp"></a>"#,
        "\n\n## Twin\n\n",
        r#"<section id="sec">"#,
        "\n\n## Twin\n\n</section>\n\n## Twin\n",
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
            ("exp", TargetKind::ExplicitAnchor),
            ("sec", TargetKind::EnclosingSection),
            ("twin-2", TargetKind::GeneratedSlug),
        ]
    );
}

#[test]
fn explicit_id_conflicts_are_rejected_not_shifted() {
    // The renderer assigns `## Alpha` the anchor `alpha`; the dfn already
    // owns that id. Silently linking `#alpha-1` would point at a nonexistent
    // anchor (and leave a duplicate id in the rendered document), so the
    // source must be repaired instead.
    let input = "# Doc\n\n<dfn id=\"alpha\">Alpha</dfn>\n\n## Alpha\n";
    let result = update_toc(input, &default_opts());
    assert!(
        matches!(result, Err(TocError::ConflictingIds { ref slug, .. }) if slug == "alpha"),
        "dfn/heading id collision must be rejected, got {result:?}"
    );

    // A heading whose slug would collide with the managed nav id is equally
    // a conflict: the ToC itself owns that id.
    let input = "# Doc\n\n## Rekon ToC\n";
    let result = update_toc(input, &default_opts());
    assert!(
        matches!(result, Err(TocError::ConflictingIds { ref slug, .. }) if slug == "rekon-toc"),
        "nav-id collision must be rejected, got {result:?}"
    );
}

#[test]
fn coinciding_explicit_targets_still_preferred() {
    // Two blessed coincidences: the slug equals the heading's own bound
    // anchor id, or its enclosing section id. Both anchor the same spot; the
    // explicit target is preferred and no error is raised.
    let input = concat!(
        r#"<a id="beta"></a>"#,
        "\n\n## Beta\n\n",
        r#"<section id="gamma">"#,
        "\n\n## Gamma\n\n</section>\n",
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
            ("beta", TargetKind::ExplicitAnchor),
            ("gamma", TargetKind::EnclosingSection),
        ]
    );
}

#[test]
fn okf_metadata_is_opaque() {
    // HTML-looking strings inside static Okf metadata are JSON string
    // content, never structural inline HTML: no fake nav region, no fake
    // section scope, no reserved fake ids, and the metadata bytes survive
    // update unchanged.
    let okf_nav = r#"<Okf data={"description": "<nav id='rekon-toc'>"} />"#;
    let okf_pair = r#"<Okf data={"markup": "<section id=\"fake-section\">paired</section>"} />"#;
    let okf_brace = r#"<Okf data={"json": "brace } inside \"quoted\" string"} />"#;
    let input =
        format!("# Doc\n\n{okf_nav}\n\n## Real one\n\n{okf_pair}\n\n## Real two\n\n{okf_brace}\n");
    let updated = assert_idempotent(&input, &default_opts());
    for line in [okf_nav, okf_pair, okf_brace] {
        assert!(
            updated.contains(line),
            "metadata bytes must survive update unchanged: {line}"
        );
    }
    let analysis = analyze(&updated, &default_opts()).expect("analyze");
    let titles: Vec<&str> = analysis.entries.iter().map(|e| e.title.as_str()).collect();
    assert_eq!(titles, vec!["Real one", "Real two"]);
    assert!(
        analysis.nav_region.is_none()
            || analysis.entries.iter().all(|e| e.target != "fake-section"),
        "no fake section target"
    );
    assert!(
        !analysis.explicit_ids.iter().any(|id| id == "fake-section"),
        "no fake id reserved from metadata strings"
    );
    assert!(updated.contains("- [Real one](#real-one)"));
    assert!(updated.contains("- [Real two](#real-two)"));
}

#[test]
fn jsx_comment_content_is_opaque() {
    // `{/* … */}` block comments produced by combine tooling must stay
    // opaque: fake headings and fake managed navs inside them are comment
    // text.
    let input = concat!(
        "# Doc\n",
        "\n",
        "{/*\n",
        "## Fake heading\n",
        "\n",
        r#"<nav id="rekon-toc" aria-label="Table of contents">"#,
        "\n",
        "\n",
        "- [Fake](#fake)\n",
        "\n",
        "</nav>\n",
        "*/}\n",
        "\n",
        "## Real\n",
    );
    let updated = assert_idempotent(input, &default_opts());
    let analysis = analyze(&updated, &default_opts()).expect("analyze");
    let titles: Vec<&str> = analysis.entries.iter().map(|e| e.title.as_str()).collect();
    assert_eq!(titles, vec!["Real"]);
    assert!(updated.contains("- [Real](#real)"));
}

#[test]
fn multiple_managed_navs_error() {
    let input = concat!(
        "# Doc\n",
        "\n",
        r#"<nav id="rekon-toc" aria-label="Table of contents">"#,
        "\n\n- [One](#one)\n\n</nav>\n",
        "\n",
        "## One\n",
        "\n",
        r#"<nav id="rekon-toc" aria-label="Table of contents">"#,
        "\n\n- [Two](#two)\n\n</nav>\n",
        "\n",
        "## Two\n",
    );
    let result = update_toc(input, &default_opts());
    assert!(
        matches!(result, Err(TocError::MultipleManagedNavs { .. })),
        "two managed regions are ambiguous, got {result:?}"
    );
}

#[test]
fn one_line_managed_nav_is_handled() {
    let input = "# Doc\n\n<nav id=\"rekon-toc\"></nav>\n\n## Alpha\n";
    let updated = assert_idempotent(input, &default_opts());
    assert!(updated.contains("- [Alpha](#alpha)"));
    assert_eq!(updated.matches("<nav id=\"rekon-toc\"").count(), 1);
}

#[test]
fn label_braces_cannot_become_expressions() {
    // `` ## Configure `{name}` `` flattens to a label with literal braces
    // (inline code preserves them). Both render modes must keep them literal
    // — a Markdown label via backslash escapes, an HTML anchor body via
    // numeric entities — so the emitted nav can never introduce an MDX
    // expression at render time.
    let input = "# Doc\n\n## Configure `{name}`\n\n## Deploy `{env:py}`\n";
    let updated = assert_idempotent(input, &default_opts());
    assert!(updated.contains("- [Configure \\{name\\}](#configure-name)"));
    assert!(updated.contains("- [Deploy \\{env:py\\}](#deploy-envpy)"));

    let updated = assert_idempotent(input, &ranges_opts());
    assert!(updated.contains("Configure &#123;name&#125;</a>"));
    assert!(updated.contains("Deploy &#123;env:py&#125;</a>"));

    // A strict re-parse of the emitted document keeps the braces as label
    // text (escaped), not expressions: the entry title round-trips.
    let analysis = analyze(&updated, &ranges_opts()).expect("analyze");
    let titles: Vec<&str> = analysis.entries.iter().map(|e| e.title.as_str()).collect();
    assert_eq!(titles, vec!["Configure {name}", "Deploy {env:py}"]);
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
fn unclosed_nav_with_fenced_fake_close_errors() {
    // The nav element is never closed in real markup; the `</nav>` inside the
    // fence is fenced code and cannot close it. Strict JSX parsing fails the
    // document rather than letting a fake close define a region.
    let input = concat!(
        "# Doc\n",
        "\n",
        r#"<nav id="rekon-toc" aria-label="Table of contents">"#,
        "\n",
        "\n",
        "- [Old](#old)\n",
        "\n",
        "```md\n",
        "</nav>\n",
        "```\n",
    );
    let result = update_toc(input, &default_opts());
    assert!(
        matches!(result, Err(TocError::Parse(_))),
        "unclosed managed nav must error, got {result:?}"
    );
}

#[test]
fn unbalanced_brace_fails_as_mdx() {
    // MDX expressions are enabled so JSX comments stay opaque; an unbalanced
    // brace in prose is an honest parse failure, not silent misreading.
    let input = "# Doc\n\nOops { unclosed brace in prose\n\n## Heading\n";
    let result = update_toc(input, &default_opts());
    assert!(
        matches!(result, Err(TocError::Parse(_))),
        "unbalanced braces must fail loudly, got {result:?}"
    );
}

#[test]
fn balanced_expression_in_heading_title() {
    // `{env}` is an MDX expression node: it contributes nothing to the title.
    let input = "# Doc\n\n## Deploy {env}\n";
    let updated = assert_idempotent(input, &default_opts());
    assert!(updated.contains("- [Deploy](#deploy)"));
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
