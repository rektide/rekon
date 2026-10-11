//! HTML tag scanning over raw `Html` node values.
//!
//! markdown-rs hands raw HTML blocks to us as flat text. Enriched Rekon
//! documents carry structure in those blocks (`<section id=…>`, `</section>`,
//! `<a id=…>`, `<nav id=…>`), so this module tokenizes the tags we care about
//! without pretending to be a full HTML parser: comments are skipped, quoted
//! attribute values may contain `>`, and only the `id` attribute is read.

/// A single opening or closing tag found in an HTML block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tag {
    pub name: String,
    pub closing: bool,
    pub self_closing: bool,
    pub id: Option<String>,
}

/// Scan `value` for HTML tags in document order, skipping comments.
pub(crate) fn scan_tags(value: &str) -> Vec<Tag> {
    let chars: Vec<char> = value.chars().collect();
    let mut tags = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '<' {
            i += 1;
            continue;
        }
        // Comment: skip to `-->`.
        if starts_with(&chars, i, "<!--") {
            match find(&chars, i + 4, "-->") {
                Some(end) => i = end + 3,
                None => break,
            }
            continue;
        }
        let next = chars.get(i + 1).copied();
        let is_tag = match next {
            Some('/') => chars.get(i + 2).is_some_and(|c| c.is_ascii_alphabetic()),
            Some(c) => c.is_ascii_alphabetic(),
            None => false,
        };
        if !is_tag {
            i += 1;
            continue;
        }
        if let Some((tag, end)) = parse_tag(&chars, i) {
            tags.push(tag);
            i = end;
        } else {
            i += 1;
        }
    }
    tags
}

/// True when `value` holds only tags and whitespace between them — the shape
/// of an anchor block such as `<a id="x"></a>` or `<a id="x" />`.
pub(crate) fn only_tags_and_whitespace(value: &str) -> bool {
    let mut rest = value;
    for raw in raw_tag_texts(value) {
        match rest.find(raw) {
            Some(at) => {
                if !rest[..at].trim().is_empty() {
                    return false;
                }
                rest = &rest[at + raw.len()..];
            }
            None => return false,
        }
    }
    rest.trim().is_empty()
}

fn raw_tag_texts(value: &str) -> Vec<&str> {
    // Re-derive raw tag slices by re-scanning; scan_tags already proved shape.
    let mut out = Vec::new();
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        if value[i..].starts_with("<!--") {
            match value[i + 4..].find("-->") {
                Some(end) => {
                    i += 4 + end + 3;
                    continue;
                }
                None => break,
            }
        }
        let next = value[i + 1..].chars().next();
        let is_tag = match next {
            Some('/') => value[i + 2..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic()),
            Some(c) => c.is_ascii_alphabetic(),
            None => false,
        };
        if !is_tag {
            i += 1;
            continue;
        }
        // Find the tag end respecting quotes.
        let mut quote = None;
        let mut j = i + 1;
        let mut end = None;
        while j < value.len() {
            let c = value[j..].chars().next().unwrap();
            let len = c.len_utf8();
            match quote {
                Some(q) if c == q => quote = None,
                Some(_) => {}
                None if c == '"' || c == '\'' => quote = Some(c),
                None if c == '>' => {
                    end = Some(j + 1);
                    break;
                }
                None => {}
            }
            j += len;
        }
        match end {
            Some(e) => {
                out.push(&value[i..e]);
                i = e;
            }
            None => break,
        }
    }
    out
}

fn parse_tag(chars: &[char], start: usize) -> Option<(Tag, usize)> {
    let mut i = start + 1;
    let closing = chars.get(i) == Some(&'/');
    if closing {
        i += 1;
    }
    let name_start = i;
    while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '-') {
        i += 1;
    }
    if i == name_start {
        return None;
    }
    let name: String = chars[name_start..i].iter().collect();
    let mut id = None;
    let mut self_closing = false;
    // Parse attributes until the closing `>`.
    loop {
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        match chars.get(i) {
            None => return None, // unterminated tag
            Some('>') => {
                i += 1;
                break;
            }
            Some('/') => {
                i += 1;
                if chars.get(i) == Some(&'>') {
                    self_closing = true;
                    i += 1;
                    break;
                }
                continue;
            }
            _ => {}
        }
        // Attribute name.
        let attr_start = i;
        while i < chars.len()
            && !chars[i].is_whitespace()
            && chars[i] != '='
            && chars[i] != '>'
            && chars[i] != '/'
        {
            i += 1;
        }
        if i == attr_start {
            i += 1; // stray character; skip it defensively
            continue;
        }
        let attr: String = chars[attr_start..i].iter().collect();
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        let mut value = None;
        if chars.get(i) == Some(&'=') {
            i += 1;
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            match chars.get(i) {
                Some(q @ ('"' | '\'')) => {
                    i += 1;
                    let v_start = i;
                    while i < chars.len() && chars[i] != *q {
                        i += 1;
                    }
                    value = Some(chars[v_start..i].iter().collect::<String>());
                    if i < chars.len() {
                        i += 1; // closing quote
                    }
                }
                _ => {
                    let v_start = i;
                    while i < chars.len()
                        && !chars[i].is_whitespace()
                        && chars[i] != '>'
                        && chars[i] != '/'
                    {
                        i += 1;
                    }
                    if i > v_start {
                        value = Some(chars[v_start..i].iter().collect::<String>());
                    }
                }
            }
        }
        if attr == "id" && id.is_none() {
            id = value;
        }
    }
    Some((
        Tag {
            name,
            closing,
            self_closing,
            id,
        },
        i,
    ))
}

fn starts_with(chars: &[char], at: usize, prefix: &str) -> bool {
    chars[at..].iter().zip(prefix.chars()).all(|(c, p)| *c == p)
}

fn find(chars: &[char], from: usize, needle: &str) -> Option<usize> {
    let n: Vec<char> = needle.chars().collect();
    let mut i = from;
    while i + n.len() <= chars.len() {
        if chars[i..i + n.len()] == n[..] {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_open_close_and_ids() {
        let tags = scan_tags(r#"<section id="a" data-source="/x.md">"#);
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "section");
        assert_eq!(tags[0].id.as_deref(), Some("a"));
        assert!(!tags[0].closing);
        assert!(!tags[0].self_closing);

        let tags = scan_tags("</section>\n</section>");
        assert_eq!(tags.len(), 2);
        assert!(tags.iter().all(|t| t.closing && t.name == "section"));

        let tags = scan_tags(r#"<nav id='rekon-toc' aria-label="x > y">"#);
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].id.as_deref(), Some("rekon-toc"));

        let tags = scan_tags(r#"<a id="z" />"#);
        assert_eq!(tags.len(), 1);
        assert!(tags[0].self_closing);
        assert_eq!(tags[0].id.as_deref(), Some("z"));
    }

    #[test]
    fn skips_comments_and_fake_angles() {
        let tags = scan_tags("<!-- <section id=\"no\"> -->\n<p>1 < 2</p>");
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].name, "p");
        assert!(!tags[0].closing);
        assert_eq!(tags[1].name, "p");
        assert!(tags[1].closing);
        assert!(tags.iter().all(|t| t.id.is_none()));
    }

    #[test]
    fn bare_id_attribute() {
        let tags = scan_tags("<section id=bare>");
        assert_eq!(tags[0].id.as_deref(), Some("bare"));
    }

    #[test]
    fn anchor_only_whitespace_between_tags() {
        assert!(only_tags_and_whitespace(r#"<a id="x"></a>"#));
        assert!(only_tags_and_whitespace(r#"<a id="x"> "#));
        assert!(only_tags_and_whitespace("<a id=\"x\" />\n"));
        assert!(!only_tags_and_whitespace(r#"<a id="x">real link text</a>"#));
    }
}
