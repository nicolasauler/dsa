#![allow(dead_code)]
mod avl_tree;
mod btree;
mod linked_list;

/// Also called prefix tree
/// Trie comes from reTRIEval maybe?
struct Trie {
    root: Option<Box<TrieNode>>,
}

struct TrieNode {
    children: Option<Vec<Box<TrieNode>>>,
    terminal: bool,
}

fn main() {}
