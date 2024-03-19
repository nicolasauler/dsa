/// Also called prefix tree
/// Trie comes from reTRIEval maybe?
struct Trie {
    root: Option<Box<TrieNode>>,
}

struct TrieNode {
    children: Option<Vec<Box<TrieNode>>>,
    terminal: bool,
}

