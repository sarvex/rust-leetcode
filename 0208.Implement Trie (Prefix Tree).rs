struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Trie (prefix tree) using array-based children for O(1) character lookup.
    ///
    /// # Intuition
    /// A trie stores strings character-by-character in a tree structure where
    /// each node has up to 26 children (for lowercase English letters).
    ///
    /// # Approach
    /// Each node contains an array of 26 optional children and a boolean flag
    /// marking end-of-word. Insert, search, and starts_with all traverse the
    /// trie following the character path.
    ///
    /// # Complexity
    /// - Time: O(m) per operation where m is the word/prefix length
    /// - Space: O(n * 26) where n is total characters inserted
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for idx in word.bytes().map(|b| (b - b'a') as usize) {
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find_node(&word).map_or(false, |node| node.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find_node(&prefix).is_some()
    }

    fn find_node(&self, s: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for idx in s.bytes().map(|b| (b - b'a') as usize) {
            match &node.children[idx] {
                Some(child) => node = child,
                None => return None,
            }
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_search_prefix() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }

    #[test]
    fn search_nonexistent() {
        let trie = Trie::new();
        assert!(!trie.search("hello".to_string()));
        assert!(!trie.starts_with("he".to_string()));
    }
}
