use anyhow::Result;
use crate::models::{TreeNode, build_tree_from_providers};
use crate::cache::CacheManager;
use std::path::Path;

pub fn run_interactive(config_dir: &Path, cache_dir: &Path) -> Result<()> {
    println!("oc-variance interactive mode");
    println!("Config dir: {}", config_dir.display());
    println!("Cache dir: {}", cache_dir.display());

    let cache_manager = CacheManager::new(cache_dir);
    let cache = cache_manager.get_or_fetch()?;

    let tree_root = build_tree_from_providers(&cache.providers);
    println!("\nTree structure with {} providers:", cache.providers.len());
    print_tree(&tree_root, 0);

    println!("\nTotal nodes in tree: {}", tree_root.count_nodes());

    Ok(())
}

fn print_tree(node: &TreeNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let expanded = if node.is_expanded() { "[-]" } else { "[+]" };
    let kind_marker = match &node.kind {
        crate::models::TreeNodeKind::Provider(_) => "P",
        crate::models::TreeNodeKind::Model(_) => "M",
        crate::models::TreeNodeKind::Variant(_) => "V",
    };
    println!("{}{} {} {}", indent, expanded, kind_marker, node.name);

    if node.is_expanded() {
        for child in &node.children {
            print_tree(child, depth + 1);
        }
    }
}
