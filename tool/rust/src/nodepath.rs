//! POSIX `node:path` semantics for manifest path planning.
//!
//! The assembler contract is lexical, not filesystem-dependent: the TypeScript
//! reference plans paths with `node:path`'s `normalize`/`resolve`/`dirname` on
//! POSIX, and the ported tests assert those exact spellings (`out//AGENTS.md`
//! is rejected as unnormalized, `../output/AGENTS.md` plans a deliberate
//! parent). `std::path` differs from Node on trailing slashes, `..` at the
//! root, and empty segments, so the Node behaviors are reproduced here on
//! plain strings instead. These are lexical guarantees only; no symlinks are
//! resolved or confined.

/// Mirrors `node:path.posix.normalize` for the paths this crate plans.
pub fn normalize(input: &str) -> String {
    if input.is_empty() {
        return ".".to_string();
    }
    let absolute = input.starts_with('/');
    let trailing_separator = input.ends_with('/');
    let mut out: Vec<&str> = Vec::new();
    for segment in input.split('/').skip(usize::from(absolute)) {
        match segment {
            "" | "." => {}
            ".." => match out.last() {
                Some(&last) if last != ".." => {
                    out.pop();
                }
                _ if !absolute => out.push(".."),
                _ => {}
            },
            other => out.push(other),
        }
    }
    let mut result = out.join("/");
    if result.is_empty() && !absolute {
        result = ".".to_string();
    }
    if absolute {
        result = format!("/{result}");
    }
    if input.len() > 1 && trailing_separator && !result.ends_with('/') {
        result.push('/');
    }
    result
}

/// Mirrors `node:path.posix.resolve(base, path)` for an absolute `base`.
pub fn resolve(base: &str, path: &str) -> String {
    if path.starts_with('/') {
        return normalize(path);
    }
    if path.is_empty() {
        return normalize(base);
    }
    let joined = format!("{}/{}", base.trim_end_matches('/'), path);
    normalize(&joined)
}

/// Mirrors `node:path.posix.dirname`.
pub fn dirname(path: &str) -> String {
    if path.is_empty() {
        return ".".to_string();
    }
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        return "/".to_string();
    }
    match trimmed.rfind('/') {
        None => ".".to_string(),
        Some(0) => "/".to_string(),
        Some(index) => trimmed[..index].to_string(),
    }
}

/// Mirrors `node:path.posix.isAbsolute` on POSIX (`/` prefix only).
pub fn is_absolute(path: &str) -> bool {
    path.starts_with('/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_matches_node_posix_spelling_rules() {
        // Expected values checked against `node -e "console.log(require('path').posix.normalize(x))"`.
        assert_eq!(normalize(""), ".");
        assert_eq!(normalize("."), ".");
        assert_eq!(normalize("./out.md"), "out.md");
        assert_eq!(normalize("out//AGENTS.md"), "out/AGENTS.md");
        assert_eq!(normalize("out/"), "out/");
        assert_eq!(normalize("/"), "/");
        assert_eq!(normalize("//"), "/");
        assert_eq!(normalize("/a//b"), "/a/b");
        assert_eq!(normalize("a/.."), ".");
        assert_eq!(normalize("a/../b"), "b");
        assert_eq!(normalize("../b"), "../b");
        assert_eq!(normalize("a/../../b"), "../b");
        assert_eq!(normalize("/.."), "/");
        assert_eq!(normalize("/a/.."), "/");
        assert_eq!(normalize("/a/./b"), "/a/b");
        assert_eq!(normalize("a/"), "a/");
    }

    #[test]
    fn resolve_plans_manifest_relative_overrides_and_deliberate_parents() {
        assert_eq!(
            resolve("/workspace", "bundle/manifest.json"),
            "/workspace/bundle/manifest.json"
        );
        assert_eq!(
            resolve("/workspace/bundle", "../output/AGENTS.md"),
            "/workspace/output/AGENTS.md"
        );
        assert_eq!(
            resolve("/workspace", "/preview/AGENTS.md"),
            "/preview/AGENTS.md"
        );
        assert_eq!(resolve("/", "a"), "/a");
        assert_eq!(
            resolve("/workspace/bundle", "..notes/GLOBAL.md"),
            "/workspace/bundle/..notes/GLOBAL.md"
        );
    }

    #[test]
    fn dirname_behaves_like_node() {
        assert_eq!(dirname("/a/b/c"), "/a/b");
        assert_eq!(dirname("/a"), "/");
        assert_eq!(dirname("/"), "/");
        assert_eq!(dirname("a"), ".");
        assert_eq!(dirname("/a/b/"), "/a");
        assert_eq!(dirname(""), ".");
    }
}
