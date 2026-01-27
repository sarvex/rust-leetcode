struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn has_all_prefixes(&self, word: &str) -> bool {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            match node.children[idx].as_ref() {
                Some(child) if child.is_end => node = child,
                _ => return false,
            }
        }
        true
    }
}

impl Solution {
    /// Finds the longest word buildable one character at a time using a trie.
    ///
    /// # Intuition
    /// A word is buildable if every prefix is also in the dictionary. A trie
    /// efficiently verifies that each prefix character leads to a valid end node.
    ///
    /// # Approach
    /// Insert all words into a trie, then iterate over words checking that every
    /// prefix is marked as a complete word. Track the longest (lexicographically
    /// smallest on tie) valid word.
    ///
    /// # Complexity
    /// - Time: O(n * L) where n is word count and L is max word length
    /// - Space: O(n * L) for the trie
    pub fn longest_word(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        for w in &words {
            trie.insert(w);
        }

        words
            .into_iter()
            .filter(|w| trie.has_all_prefixes(w))
            .max_by(|a, b| a.len().cmp(&b.len()).then_with(|| b.cmp(a)))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let words = vec!["w", "wo", "wor", "worl", "world"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_word(words), "world");
    }

    #[test]
    fn test_lexicographic_tie() {
        let words = vec!["a", "banana", "app", "appl", "ap", "apply", "apple"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_word(words), "apple");
    }

    #[test]
    fn test_single_char_words() {
        let words = vec!["a", "b", "c"].into_iter().map(String::from).collect();
        assert_eq!(Solution::longest_word(words), "a");
    }
}
