use rekon_markup::{
    MetadataOrigin, ScopeKind, extract, parse_document, parse_frontmatter, render_okf,
};
use serde_json::json;

#[test]
fn unknown_nested_frontmatter_roundtrips_through_static_jsx() {
    let source = "---\ntype: Strange New Type\nresource: /not-the-section\nunknown:\n  nested: [true, null, 12, 1.5, 'λ <tag> & \"quoted\"']\nverified: {by: human:someone, at: 2026-10-11T00:00:00Z}\n---\n# Body\n";
    let frontmatter = parse_frontmatter(source).unwrap();
    assert_eq!(frontmatter.body, "# Body\n");
    assert_eq!(
        frontmatter.metadata.as_ref().unwrap()["unknown"]["nested"][4],
        "λ <tag> & \"quoted\""
    );
    let generated = format!(
        "<section id=\"source\" data-source=\"/source.md\">\n\n{}\n\n## Body\n\n</section>",
        render_okf(frontmatter.metadata.as_ref().unwrap()).unwrap()
    );
    let records = extract(&generated).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].metadata, frontmatter.metadata.unwrap());
    assert_eq!(records[0].owner.id.as_deref(), Some("source"));
    assert_eq!(records[0].owner.data_source.as_deref(), Some("/source.md"));
    assert_eq!(records[0].owner.kind, ScopeKind::Section);
    assert_eq!(
        &generated[records[0].span.start..records[0].span.end],
        render_okf(&records[0].metadata).unwrap()
    );
}

#[test]
fn nearest_owner_only_and_document_records_in_source_order() {
    let source = "---\ntype: File\n---\n<Okf data={{\"type\":\"Document\"}} />\n\n<article id=\"outer\" data-source=\"/outer.md\">\n\n<Okf data={{\"type\":\"Outer\",\"verified\":true}} />\n\n<section id={\"inner\"}>\n\n<aside><Okf data={{\"type\":\"Inner\"}} /></aside>\n\n</section>\n\n<Okf data={{\"type\":\"Outer again\"}} />\n\n</article>\n";
    let records = extract(source).unwrap();
    assert_eq!(records.len(), 5);
    assert_eq!(records[0].origin, MetadataOrigin::Frontmatter);
    assert_eq!(records[0].owner.kind, ScopeKind::Document);
    assert_eq!(records[1].owner.kind, ScopeKind::Document);
    assert_eq!(records[2].owner.id.as_deref(), Some("outer"));
    assert_eq!(records[3].owner.id.as_deref(), Some("inner"));
    assert_eq!(records[3].owner.data_source, None);
    assert!(records[3].metadata.get("verified").is_none());
    assert_eq!(records[4].owner.id.as_deref(), Some("outer"));
    let inner = &source[records[3].owner.span.start..records[3].owner.span.end];
    assert!(inner.starts_with("<section"));
    assert!(inner.ends_with("</section>"));
}

#[test]
fn fences_inline_code_and_comments_do_not_extract_metadata_or_headings() {
    let source = "<!-- Generated metadata: λ -->\n\n# Real *heading* with `code`\n\n```mdx\n# Fake\n<Okf data={dynamic} />\n<section id=\"fake\">\n```\n\n`<Okf data={dynamic} />`\n\n    <Okf data={dynamic} />\n\n<!-- <Okf data={dynamic} /> -->\n\nSetext title\n------------\n";
    let parsed = parse_document(source).unwrap();
    assert!(parsed.metadata.is_empty());
    assert_eq!(parsed.headings.len(), 2);
    assert_eq!(parsed.headings[0].text, "Real heading with code");
    assert_eq!(parsed.headings[1].depth, 2);
    assert_eq!(
        &source[parsed.headings[0].span.start..parsed.headings[0].span.end],
        "# Real *heading* with `code`"
    );
}

#[test]
fn static_json_string_special_characters_roundtrip() {
    for text in [
        "}",
        "{",
        "{unbalanced",
        "}unbalanced",
        "</Okf>",
        "a\\\"b",
        "\n\r\t",
        "λ 🦀",
    ] {
        let value = json!({"type": "Test", "text": text});
        let output = render_okf(&value).unwrap();
        assert_eq!(extract(&output).unwrap()[0].metadata, value, "{output}");
    }
}

#[test]
fn rejects_nonliteral_and_malformed_props() {
    for source in [
        "<Okf data={metadata} />",
        "<Okf data={{type: 'Practice'}} />",
        "<Okf data={{\"type\": call()}} />",
        "<Okf data={{...metadata}} />",
        "<Okf {...metadata} />",
        "<Okf data=\"{}\" />",
        "<Okf data={[]} />",
        "<Okf data={null} />",
        "<Okf data={{\"x\":1}} data={{\"x\":2}} />",
        "<Okf data={{\"x\":1}} extra={run()} />",
        "<Okf />",
        "<Okf data={{\"x\":1}}>body</Okf>",
        "<section id={compute()}><Okf data={{}} /></section>",
        "<section {...props}><Okf data={{}} /></section>",
        "<Okf data={{\"x\":1} />",
    ] {
        assert!(extract(source).is_err(), "accepted: {source}");
    }
}

#[test]
fn rejects_unsupported_yaml_without_silent_coercion() {
    for source in [
        "---\ntype: Test\nx: .nan\n---\n",
        "---\ntype: Test\nx: .inf\n---\n",
        "---\ntype: Test\nx: !custom value\n---\n",
        "---\ntype: Test\nx: {12: value}\n---\n",
        "---\n- not\n- an object\n---\n",
        "---\nx: 1\nx: 2\n---\n",
        "---\ntype: Test\n",
    ] {
        assert!(parse_frontmatter(source).is_err(), "accepted: {source}");
    }
}

#[test]
fn crlf_offsets_and_exact_body_without_frontmatter() {
    let source = "---\r\ntype: Test\r\n---\r\n\r\n# λ\r\n";
    let frontmatter = parse_frontmatter(source).unwrap();
    assert_eq!(frontmatter.body, "\r\n# λ\r\n");
    assert_eq!(&source[frontmatter.body_offset..], frontmatter.body);
    let parsed = parse_document(source).unwrap();
    assert_eq!(
        &source[parsed.headings[0].span.start..parsed.headings[0].span.end],
        "# λ"
    );
    assert_eq!(
        parse_frontmatter("# No metadata\n---\n").unwrap().metadata,
        None
    );
}

#[test]
fn headings_within_sections_are_structural_not_html_block_text() {
    let source = "<section id=\"s\">\n# Heading\n\n## Next\n</section>";
    let parsed = parse_document(source).unwrap();
    assert_eq!(parsed.headings.len(), 2);
}

#[test]
fn actual_global_metadata_survives_a_document_section() {
    let source = include_str!("../../../../../code/GLOBAL.md");
    let frontmatter = parse_frontmatter(source).unwrap();
    let metadata = frontmatter.metadata.as_ref().unwrap();
    let output = format!(
        "<!-- generated test assembly -->\n\n# Combined\n\n<section id=\"fragment-code\" data-source=\"/code/GLOBAL.md\">\n\n{}\n\n{}\n\n</section>\n",
        render_okf(metadata).unwrap(),
        frontmatter.body
    );
    let records = extract(&output).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(&records[0].metadata, metadata);
    assert_eq!(records[0].owner.id.as_deref(), Some("fragment-code"));
    assert_eq!(
        records[0].owner.data_source.as_deref(),
        Some("/code/GLOBAL.md")
    );
}

#[test]
fn actual_knowledge_label_preserves_sources() {
    let metadata = parse_frontmatter(include_str!("../../../../../okf/README.md"))
        .unwrap()
        .metadata
        .unwrap();
    assert!(metadata["sources"].is_array());
    let output = render_okf(&metadata).unwrap();
    assert_eq!(extract(&output).unwrap()[0].metadata, metadata);
}

#[test]
fn multiline_definition_containers_require_flow_jsx_syntax() {
    let invalid =
        "<dl>\n<dt><dfn id=\"term\">Term</dfn></dt>\n<dd>First line.\nSecond line.\n</dd>\n</dl>";
    assert!(parse_document(invalid).is_err());
    let valid =
        "<dl>\n<dt><dfn id=\"term\">Term</dfn></dt>\n<dd>\nFirst line.\nSecond line.\n</dd>\n</dl>";
    assert!(parse_document(valid).is_ok());
}

#[test]
fn jsx_comment_static_okf_is_not_a_record() {
    let source = "{/* <Okf data={{\"type\":\"Fake\"}} /> */}";
    assert!(extract(source).unwrap().is_empty());
}

#[test]
fn jsx_comments_never_produce_metadata_or_structure() {
    let source = "---\ntype: File\n---\n# Visible\n\n<section id=\"real\">\n\n{/* <Okf data={{\"type\":\"Fake\"}} /> */}\n\n{/*\n# Hidden heading\n<section id=\"fake\">\n<Okf data={run()} />\n</section>\n} unmatched comment braces {\n*/}\n\nInline {/* <Okf data={{\"type\":\"Inline fake\"}} /> */} prose.\n\n<Okf data={{\"type\":\"Real\"}} />\n\n## Last\n\n</section>\n";
    let parsed = parse_document(source).unwrap();
    assert_eq!(parsed.metadata.len(), 2);
    assert_eq!(parsed.metadata[1].metadata["type"], "Real");
    assert_eq!(parsed.metadata[1].owner.id.as_deref(), Some("real"));
    assert_eq!(parsed.headings.len(), 2);
    assert_eq!(parsed.headings[1].text, "Last");
    assert_eq!(
        &source[parsed.headings[1].span.start..parsed.headings[1].span.end],
        "## Last"
    );
}

#[test]
fn comment_syntax_in_literal_metadata_is_preserved() {
    for text in [
        "{/* comment */}",
        "<fake> {/* comment */}",
        "{/* unterminated",
        "a > {/* <Okf data={dynamic} /> */}",
    ] {
        let value = json!({"type": "Test", "text": text});
        let source = render_okf(&value).unwrap();
        assert_eq!(extract(&source).unwrap()[0].metadata, value);
    }
}

#[test]
fn comments_inside_inline_scope_containers_are_ignored() {
    let source = "<section id=\"real\">{/* <Okf data={{\"type\":\"Fake\"}} /> */}<Okf data={{\"type\":\"Real\"}} /></section>";
    let records = extract(source).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].metadata["type"], "Real");
    assert_eq!(records[0].owner.id.as_deref(), Some("real"));
}

#[test]
fn jsx_comment_unicode_crlf_and_following_ranges_are_preserved() {
    let source = "---\r\ntype: File\r\n---\r\n{/* λ 🦀\r\n<Okf data={dynamic} />\r\n*/ }\r\n\r\n# Visible\r\n\r\n<Okf data={{\"type\":\"Real\"}} />\r\n";
    let parsed = parse_document(source).unwrap();
    assert_eq!(parsed.metadata.len(), 2);
    let record = &parsed.metadata[1];
    assert_eq!(
        &source[record.span.start..record.span.end],
        "<Okf data={{\"type\":\"Real\"}} />"
    );
    assert_eq!(
        &source[parsed.headings[0].span.start..parsed.headings[0].span.end],
        "# Visible"
    );
}

#[test]
fn malformed_or_dynamic_comment_expressions_fail_explicitly() {
    for source in [
        "{/* not closed",
        "{/* comment */ call()}",
        "{/* first */ /* second */}",
    ] {
        assert!(parse_document(source).is_err(), "accepted: {source}");
    }
}

#[test]
fn prose_braces_and_code_comment_examples_remain_static() {
    let source = "# Literal { prose\n\nUnmatched { and } } stay prose.\n\n`{/* example */}`\n\n\\{/* escaped */}\n\n```mdx\n{/* <Okf data={dynamic} /> */}\n```\n\n<Okf data={{\"type\":\"Real\"}} />\n";
    let parsed = parse_document(source).unwrap();
    assert_eq!(parsed.headings[0].text, "Literal { prose");
    assert_eq!(parsed.metadata.len(), 1);
    assert_eq!(parsed.metadata[0].metadata["type"], "Real");
}
