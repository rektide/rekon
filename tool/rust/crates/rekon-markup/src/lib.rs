//! Static Markdown/MDX structure and scoped OKF metadata. No expressions execute.

use markdown::mdast::{AttributeContent, AttributeValue, Node};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("frontmatter at byte {offset}: {message}")]
    Frontmatter { offset: usize, message: String },
    #[error("markup: {0}")]
    Markup(String),
    #[error("static metadata at byte {offset}: {message}")]
    Metadata { offset: usize, message: String },
}

/// Half-open UTF-8 byte range in the original source, not character offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Frontmatter<'a> {
    pub metadata: Option<Value>,
    pub body: &'a str,
    pub body_offset: usize,
    pub span: Option<SourceRange>,
}

/// Read only a leading YAML block. Return the exact, unmodified body slice.
/// Metadata must be an object with recursively JSON-compatible values.
pub fn parse_frontmatter(source: &str) -> Result<Frontmatter<'_>> {
    let mut lines = source.split_inclusive('\n');
    let first = lines.next().unwrap_or_default();
    if first.trim_end_matches(['\r', '\n']) != "---" {
        return Ok(Frontmatter {
            metadata: None,
            body: source,
            body_offset: 0,
            span: None,
        });
    }
    let mut offset = first.len();
    for line in lines {
        if line.trim_end_matches(['\r', '\n']) == "---" {
            let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(&source[first.len()..offset])
                .map_err(|error| Error::Frontmatter {
                    offset: first.len(),
                    message: error.to_string(),
                })?;
            let metadata = yaml_to_json(yaml, "$", first.len())?;
            if !metadata.is_object() {
                return Err(Error::Frontmatter {
                    offset: first.len(),
                    message: "metadata must be a mapping/object".into(),
                });
            }
            let body_offset = offset + line.len();
            return Ok(Frontmatter {
                metadata: Some(metadata),
                body: &source[body_offset..],
                body_offset,
                span: Some(SourceRange {
                    start: 0,
                    end: body_offset,
                }),
            });
        }
        offset += line.len();
    }
    Err(Error::Frontmatter {
        offset: 0,
        message: "missing closing --- delimiter".into(),
    })
}

fn yaml_to_json(value: serde_yaml_ng::Value, path: &str, offset: usize) -> Result<Value> {
    use serde_yaml_ng::Value as Yaml;
    let unsupported = |message: &str| Error::Frontmatter {
        offset,
        message: format!("{path}: {message}"),
    };
    Ok(match value {
        Yaml::Null => Value::Null,
        Yaml::Bool(value) => Value::Bool(value),
        Yaml::String(value) => Value::String(value),
        Yaml::Number(value) => {
            let number = serde_json::from_str::<Value>(&value.to_string())
                .map_err(|_| unsupported("number is not representable in JSON"))?;
            if !number.is_number() {
                return Err(unsupported("number is not representable in JSON"));
            }
            number
        }
        Yaml::Sequence(values) => Value::Array(
            values
                .into_iter()
                .enumerate()
                .map(|(index, value)| yaml_to_json(value, &format!("{path}[{index}]"), offset))
                .collect::<Result<_>>()?,
        ),
        Yaml::Mapping(values) => {
            let mut object = serde_json::Map::new();
            for (key, value) in values {
                let Yaml::String(key) = key else {
                    return Err(unsupported("JSON object keys must be strings"));
                };
                object.insert(
                    key.clone(),
                    yaml_to_json(value, &format!("{path}.{key}"), offset)?,
                );
            }
            Value::Object(object)
        }
        Yaml::Tagged(_) => return Err(unsupported("tagged YAML values are not JSON literals")),
    })
}

/// Emit a static JSX data prop. This representation is Rekon-specific, not OKF v0.2.
pub fn render_okf(metadata: &Value) -> Result<String> {
    if !metadata.is_object() {
        return Err(Error::Metadata {
            offset: 0,
            message: "data must be a JSON object".into(),
        });
    }
    // JSON escaping is sufficient inside a JSX expression, including `<` and quotes.
    let json = serde_json::to_string(metadata).expect("a serde_json::Value serializes");
    Ok(format!("<Okf data={{{json}}} />"))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    Document,
    Section,
    Article,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Scope {
    pub kind: ScopeKind,
    /// Explicit structural ID; missing IDs remain missing, not synthesized resources.
    pub id: Option<String>,
    pub data_source: Option<String>,
    pub span: SourceRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataOrigin {
    Frontmatter,
    OkfElement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScopedMetadata {
    pub metadata: Value,
    pub owner: Scope,
    pub origin: MetadataOrigin,
    /// Range of the frontmatter block or entire Okf element.
    pub span: SourceRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Heading {
    pub depth: u8,
    /// Plain readable text, with inline formatting removed by the Markdown AST.
    pub text: String,
    pub span: SourceRange,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ParsedDocument {
    pub headings: Vec<Heading>,
    pub metadata: Vec<ScopedMetadata>,
}

/// Parse CommonMark/GFM with static MDX JSX. Comment-only JSX expressions are
/// ignored; other prose braces stay text. Attributes never execute JavaScript.
pub fn parse_document(source: &str) -> Result<ParsedDocument> {
    let frontmatter = parse_frontmatter(source)?;
    let mut body = frontmatter.body.to_owned();
    // MDX does not accept HTML comments. Recognize comments with the Markdown
    // parser first, then blank them without changing any source byte positions.
    let commonmark = markdown::to_mdast(&body, &markdown::ParseOptions::gfm())
        .map_err(|error| Error::Markup(error.to_string()))?;
    mask_comments(&commonmark, &mut body);
    // Disabling HTML flow only for discovery keeps text inside HTML-style
    // containers visible, even without blank lines around their children.
    let mut discovery_options = markdown::ParseOptions::gfm();
    discovery_options.constructs.html_flow = false;
    let discovery = markdown::to_mdast(&body, &discovery_options)
        .map_err(|error| Error::Markup(error.to_string()))?;
    mask_jsx_comments(&discovery, &mut body, frontmatter.body_offset)?;
    let tree = markdown::to_mdast(&body, &static_jsx_options())
        .map_err(|error| Error::Markup(error.to_string()))?;
    let document = Scope {
        kind: ScopeKind::Document,
        id: None,
        data_source: None,
        span: SourceRange {
            start: 0,
            end: source.len(),
        },
    };
    let mut parsed = ParsedDocument {
        headings: Vec::new(),
        metadata: Vec::new(),
    };
    if let Some(metadata) = frontmatter.metadata {
        parsed.metadata.push(ScopedMetadata {
            metadata,
            owner: document.clone(),
            origin: MetadataOrigin::Frontmatter,
            span: frontmatter.span.expect("frontmatter has a range"),
        });
    }
    visit(&tree, &document, frontmatter.body_offset, &mut parsed)?;
    Ok(parsed)
}

fn static_jsx_options() -> markdown::ParseOptions {
    let mut options = markdown::ParseOptions::gfm();
    options.constructs.html_flow = false;
    options.constructs.html_text = false;
    options.constructs.mdx_jsx_flow = true;
    options.constructs.mdx_jsx_text = true;
    // A real JSON parser, rather than the MDX parser's brace-counting fallback,
    // is essential: a JSON string may itself contain unmatched `{` or `}`.
    options.mdx_expression_parse = Some(Box::new(|value, _kind| {
        match serde_json::from_str::<Value>(value) {
            Ok(_) => markdown::MdxSignal::Ok,
            Err(error) if error.is_eof() => markdown::MdxSignal::Eof(
                format!("incomplete static JSON attribute: {error}"),
                Box::new("rekon-markup".into()),
                Box::new("static-json".into()),
            ),
            Err(error) => markdown::MdxSignal::Error(
                format!("attribute must be literal JSON, not a dynamic expression: {error}"),
                0,
                Box::new("rekon-markup".into()),
                Box::new("static-json".into()),
            ),
        }
    }));
    options
}

pub fn extract(source: &str) -> Result<Vec<ScopedMetadata>> {
    Ok(parse_document(source)?.metadata)
}

fn mask_comments(node: &Node, source: &mut String) {
    if let Node::Html(html) = node
        && html.value.trim().starts_with("<!--")
        && html.value.trim().ends_with("-->")
        && let Some(position) = node.position()
    {
        let range = position.start.offset..position.end.offset;
        mask_range(source, range);
    }
    if let Some(children) = node.children() {
        for child in children {
            mask_comments(child, source);
        }
    }
}

fn mask_range(source: &mut String, range: std::ops::Range<usize>) {
    let blank: String = source[range.clone()]
        .bytes()
        .map(|byte| match byte {
            b'\n' | b'\r' => char::from(byte),
            _ => ' ',
        })
        .collect();
    source.replace_range(range, &blank);
}

/// Find comment openers only in Markdown text, never code, HTML attributes, or
/// link destinations. The maintained MDX expression parser determines the
/// expression range and validates the reserved, comment-only syntax. Ordinary
/// prose braces stay ordinary prose, including unmatched braces.
fn mask_jsx_comments(tree: &Node, source: &mut String, offset: usize) -> Result<()> {
    fn candidates(node: &Node, source: &str, starts: &mut Vec<usize>) {
        if let Node::Text(_) = node {
            let span = range(node, 0);
            for (relative, _) in source[span.start..span.end].match_indices("{/*") {
                let start = span.start + relative;
                let escaped = source[..start]
                    .bytes()
                    .rev()
                    .take_while(|byte| *byte == b'\\')
                    .count()
                    % 2
                    == 1;
                if !escaped {
                    starts.push(start);
                }
            }
        }
        if let Some(children) = node.children() {
            for child in children {
                candidates(child, source, starts);
            }
        }
    }
    let mut starts = Vec::new();
    candidates(tree, source, &mut starts);
    let mut masked_until = 0;
    for start in starts {
        if start < masked_until {
            continue;
        }
        if inside_static_jsx_tag(source, start) {
            continue;
        }
        let invalid = || {
            Error::Markup(format!(
                "JSX comment at byte {} requires a single closed block comment inside braces",
                offset + start,
            ))
        };
        let close = source[start + 3..].find("*/").ok_or_else(invalid)? + start + 3 + 2;
        let after = source[close..].trim_start();
        if !after.starts_with('}') {
            return Err(invalid());
        }
        let end = source.len() - after.len() + 1;
        let mut options = markdown::ParseOptions::gfm();
        options.constructs.mdx_expression_flow = true;
        options.constructs.mdx_expression_text = true;
        options.mdx_expression_parse = Some(Box::new(|value, _kind| {
            let Some(comment) = value.trim().strip_prefix("/*") else {
                return markdown::MdxSignal::Error(
                    "only a static block comment is supported here".into(),
                    0,
                    Box::new("rekon-markup".into()),
                    Box::new("static-comment".into()),
                );
            };
            if let Some(close) = comment.find("*/")
                && comment[close + 2..].trim().is_empty()
            {
                markdown::MdxSignal::Ok
            } else {
                markdown::MdxSignal::Eof(
                    "incomplete static block comment".into(),
                    Box::new("rekon-markup".into()),
                    Box::new("static-comment".into()),
                )
            }
        }));
        let parsed = markdown::to_mdast(&source[start..end], &options).map_err(|error| {
            Error::Markup(format!("JSX comment at byte {}: {error}", offset + start))
        })?;
        let children = parsed.children().expect("parsed root has children");
        if children.len() != 1 || !matches!(&children[0], Node::MdxFlowExpression(_)) {
            return Err(invalid());
        }
        let span = range(&children[0], start);
        if span.end != end {
            return Err(invalid());
        }
        mask_range(source, span.start..span.end);
        masked_until = span.end;
    }
    Ok(())
}

/// CommonMark can classify a JSX tag as text because JSON braces are not HTML
/// attribute syntax. Validate candidate opening tags with the native MDX parser
/// before masking, so comment-like strings inside static props stay untouched.
/// The `>` search proposes boundaries only: the parser, including its JSON
/// attribute callback, decides whether each candidate is a complete tag.
fn inside_static_jsx_tag(source: &str, candidate: usize) -> bool {
    for (start, _) in source[..candidate].rmatch_indices('<') {
        if source
            .as_bytes()
            .get(start + 1)
            .is_none_or(|byte| !byte.is_ascii_alphabetic())
        {
            continue;
        }
        for (relative_end, _) in source[start..].match_indices('>') {
            let end = start + relative_end + 1;
            let tag = &source[start..end];
            let tag = if tag[..tag.len() - 1].trim_end().ends_with('/') {
                tag.to_owned()
            } else {
                format!("{} />", &tag[..tag.len() - 1])
            };
            if let Ok(parsed) = markdown::to_mdast(&tag, &static_jsx_options())
                && let Some(children) = parsed.children()
                && children.len() == 1
                && jsx(&children[0]).is_some()
                && children[0].children().is_some_and(Vec::is_empty)
            {
                if end > candidate {
                    return true;
                }
                break;
            }
        }
    }
    false
}

fn range(node: &Node, offset: usize) -> SourceRange {
    let position = node.position().expect("parsed AST nodes have positions");
    SourceRange {
        start: position.start.offset + offset,
        end: position.end.offset + offset,
    }
}

fn jsx(node: &Node) -> Option<(Option<&str>, &[AttributeContent])> {
    match node {
        Node::MdxJsxFlowElement(element) => Some((element.name.as_deref(), &element.attributes)),
        Node::MdxJsxTextElement(element) => Some((element.name.as_deref(), &element.attributes)),
        _ => None,
    }
}

fn static_attribute(
    attributes: &[AttributeContent],
    name: &str,
    offset: usize,
) -> Result<Option<String>> {
    let mut value = None;
    for attribute in attributes {
        match attribute {
            AttributeContent::Expression(_) => {
                return Err(Error::Metadata {
                    offset,
                    message: "spread/dynamic attributes are unsupported on metadata owners".into(),
                });
            }
            AttributeContent::Property(property) if property.name == name => {
                if value.is_some() {
                    return Err(Error::Metadata {
                        offset,
                        message: format!("duplicate {name} attribute"),
                    });
                }
                value = Some(match &property.value {
                    Some(AttributeValue::Literal(value)) => value.clone(),
                    Some(AttributeValue::Expression(expression)) => {
                        serde_json::from_str::<String>(&expression.value).map_err(|_| {
                            Error::Metadata {
                                offset,
                                message: format!("{name} must be a static string"),
                            }
                        })?
                    }
                    None => {
                        return Err(Error::Metadata {
                            offset,
                            message: format!("{name} must be a static string"),
                        });
                    }
                });
            }
            _ => {}
        }
    }
    Ok(value)
}

fn okf_data(attributes: &[AttributeContent], offset: usize) -> Result<Value> {
    if attributes.len() != 1 {
        return Err(Error::Metadata {
            offset,
            message: "Okf requires exactly one static data attribute".into(),
        });
    }
    let AttributeContent::Property(property) = &attributes[0] else {
        return Err(Error::Metadata {
            offset,
            message: "Okf does not support spread/dynamic attributes".into(),
        });
    };
    let Some(AttributeValue::Expression(expression)) = &property.value else {
        return Err(Error::Metadata {
            offset,
            message: "Okf data requires a JSON object inside JSX braces".into(),
        });
    };
    if property.name != "data" {
        return Err(Error::Metadata {
            offset,
            message: "Okf requires a data attribute".into(),
        });
    }
    let value: Value =
        serde_json::from_str(&expression.value).map_err(|error| Error::Metadata {
            offset,
            message: format!("data must be literal JSON, not a dynamic expression: {error}"),
        })?;
    if !value.is_object() {
        return Err(Error::Metadata {
            offset,
            message: "data must be a JSON object".into(),
        });
    }
    Ok(value)
}

fn visit(node: &Node, owner: &Scope, offset: usize, parsed: &mut ParsedDocument) -> Result<()> {
    let mut nested_owner = None;
    if let Some((name, attributes)) = jsx(node) {
        match name {
            Some("section" | "article") => {
                let span = range(node, offset);
                nested_owner = Some(Scope {
                    kind: if name == Some("section") {
                        ScopeKind::Section
                    } else {
                        ScopeKind::Article
                    },
                    id: static_attribute(attributes, "id", span.start)?,
                    data_source: static_attribute(attributes, "data-source", span.start)?,
                    span,
                });
            }
            Some("Okf") => {
                let span = range(node, offset);
                if node.children().is_some_and(|children| !children.is_empty()) {
                    return Err(Error::Metadata {
                        offset: span.start,
                        message: "Okf must not contain body content".into(),
                    });
                }
                parsed.metadata.push(ScopedMetadata {
                    metadata: okf_data(attributes, span.start)?,
                    owner: owner.clone(),
                    origin: MetadataOrigin::OkfElement,
                    span,
                });
            }
            _ => {}
        }
    }
    if let Node::Heading(heading) = node {
        parsed.headings.push(Heading {
            depth: heading.depth,
            text: plain_text(node),
            span: range(node, offset),
        });
    }
    if let Some(children) = node.children() {
        for child in children {
            visit(
                child,
                nested_owner.as_ref().unwrap_or(owner),
                offset,
                parsed,
            )?;
        }
    }
    Ok(())
}

fn plain_text(node: &Node) -> String {
    match node {
        Node::Text(text) => text.value.clone(),
        Node::InlineCode(code) => code.value.clone(),
        Node::Image(image) => image.alt.clone(),
        Node::Break(_) => " ".into(),
        _ => node
            .children()
            .map(|children| children.iter().map(plain_text).collect())
            .unwrap_or_default(),
    }
}
