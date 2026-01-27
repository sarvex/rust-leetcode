/// Trie node for prefix-complete word search.
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

    fn all_prefixes_exist(&self, word: &str) -> bool {
        let mut node = self;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            match &node.children[idx] {
                Some(child) => {
                    node = child.as_ref();
                    if !node.is_end {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}

impl Solution {
    /// Finds the longest word where every prefix is also in the dictionary.
    ///
    /// # Intuition
    /// A trie efficiently validates that all prefixes of a word exist as
    /// complete words. Among valid candidates, pick the longest (or
    /// lexicographically smallest on ties).
    ///
    /// # Approach
    /// 1. Insert all words into a trie.
    /// 2. For each word, check if every prefix ends at a trie node marked as complete.
    /// 3. Track the best candidate by length then lexicographic order.
    ///
    /// # Complexity
    /// - Time: O(n * L) where L is average word length
    /// - Space: O(n * L)
    pub fn longest_word(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        for word in &words {
            trie.insert(word);
        }

        let mut result = String::new();
        for word in &words {
            if (word.len() > result.len() || (word.len() == result.len() && *word < result))
                && trie.all_prefixes_exist(word)
            {
                result = word.clone();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::longest_word(
                vec!["k", "ki", "kit", "kite", "kites"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            "kites"
        );
    }

    #[test]
    fn test_missing_prefix() {
        assert_eq!(
            Solution::longest_word(
                vec!["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            "apple"
        );
    }
}
