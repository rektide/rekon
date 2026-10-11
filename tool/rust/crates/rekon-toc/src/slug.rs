//! GitHub-style heading slugs and label escaping for ToC emission.

/// Slugify heading text the way GitHub anchor slugs roughly do:
/// lowercase, keep alphanumerics/`_`/`-`, turn each whitespace character into
/// `-`, drop other punctuation. An empty result falls back to `section`.
pub(crate) fn slugify(text: &str) -> String {
    let lowered: String = text.to_lowercase();
    let mut out = String::new();
    for c in lowered.chars() {
        if c.is_whitespace() {
            out.push('-');
        } else if c.is_alphanumeric() || c == '_' || c == '-' {
            out.push(c);
        }
        // Everything else (punctuation, symbols) is dropped.
    }
    if out.is_empty() {
        out.push_str("section");
    }
    out
}

/// Escape a ToC link label for Markdown square-bracket syntax.
pub(crate) fn escape_label(label: &str) -> String {
    let mut out = String::with_capacity(label.len());
    for c in label.chars() {
        match c {
            '\\' | '[' | ']' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

/// Escape text placed inside an HTML element (anchor bodies).
pub(crate) fn escape_html_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape text placed inside a double-quoted HTML attribute value.
pub(crate) fn escape_html_attr(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_style_slugs() {
        assert_eq!(
            slugify("Code — stage and change discipline"),
            "code--stage-and-change-discipline"
        );
        assert_eq!(slugify("Terms in use"), "terms-in-use");
        assert_eq!(slugify("名前と住所"), "名前と住所");
        assert_eq!(slugify("Über Straße"), "über-straße");
        assert_eq!(slugify("a_b-c"), "a_b-c");
        assert_eq!(slugify("1. Introduction"), "1-introduction");
        assert_eq!(slugify("!!!"), "section");
    }

    #[test]
    fn label_escaping() {
        assert_eq!(
            escape_label("[Brackets] and \\ backslash"),
            "\\[Brackets\\] and \\\\ backslash"
        );
        assert_eq!(escape_html_text("a & <b>"), "a &amp; &lt;b&gt;");
        assert_eq!(escape_html_attr(r#"say "hi""#), "say &quot;hi&quot;");
    }
}
