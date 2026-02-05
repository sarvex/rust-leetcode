/// Magic dictionary supporting search with exactly one character difference.
///
/// # Intuition
/// Uses a trie to efficiently search for words that differ by exactly one
/// character from the query.
///
/// # Approach
/// Build a trie from the dictionary words. During search, recursively explore
/// the trie tracking the number of character differences. Accept paths with
/// exactly one difference.
///
/// # Complexity
/// - Time: O(n * m) for build where n is number of words and m is average length
/// - Time: O(m * 26) for search where m is query length
/// - Space: O(ALPHABET * N) where N is total characters in dictionary
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
        for &b in word.as_bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn search_with_diff(&self, word: &[u8], diff: i32) -> bool {
        if word.is_empty() {
            return diff == 1 && self.is_end;
        }
        let idx = (word[0] - b'a') as usize;
        if let Some(child) = &self.children[idx] {
            if child.search_with_diff(&word[1..], diff) {
                return true;
            }
        }
        if diff == 0 {
            for (i, child) in self.children.iter().enumerate() {
                if i != idx {
                    if let Some(c) = child {
                        if c.search_with_diff(&word[1..], 1) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

struct MagicDictionary {
    trie: Trie,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in &dictionary {
            self.trie.insert(word);
        }
    }

    fn search(&self, search_word: String) -> bool {
        self.trie.search_with_diff(search_word.as_bytes(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_magic_dictionary() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);

        // "hello" with 'e' -> 'a' gives "hallo" (not in dict, false)
        assert!(!dict.search("hello".to_string()));

        // "hello" with second 'l' -> 'e' gives "heelo" (not in dict, but one diff from "hello")
        assert!(dict.search("hhllo".to_string()));

        // "hell" cannot match anything with exactly one change
        assert!(!dict.search("hell".to_string()));

        // "leetcoded" has one extra char, cannot match with one change
        assert!(!dict.search("leetcoded".to_string()));
    }

    #[test]
    fn test_single_character_words() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["a".to_string(), "b".to_string()]);

        // "a" can become "b" with one change
        assert!(dict.search("a".to_string()));

        // "b" can become "a" with one change
        assert!(dict.search("b".to_string()));

        // "c" can become "a" or "b" with one change
        assert!(dict.search("c".to_string()));
    }

    #[test]
    fn test_empty_dictionary() {
        let dict = MagicDictionary::new();

        assert!(!dict.search("hello".to_string()));
        assert!(!dict.search("a".to_string()));
    }

    #[test]
    fn test_same_length_different_words() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec![
            "cat".to_string(),
            "bat".to_string(),
            "rat".to_string(),
        ]);

        // "cat" can become "bat" or "rat" with one change
        assert!(dict.search("cat".to_string()));

        // "hat" can become "cat", "bat", or "rat" with one change
        assert!(dict.search("hat".to_string()));

        // "car" can become "cat" with one change
        assert!(dict.search("car".to_string()));

        // "xyz" cannot become any word with exactly one change
        assert!(!dict.search("xyz".to_string()));
    }

    #[test]
    fn test_longer_words() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["apple".to_string(), "apply".to_string()]);

        // "apple" can become "apply" with one change
        assert!(dict.search("apple".to_string()));

        // "apply" can become "apple" with one change
        assert!(dict.search("apply".to_string()));

        // "appla" can become "apple" with one change
        assert!(dict.search("appla".to_string()));

        // "appl" is too short
        assert!(!dict.search("appl".to_string()));

        // "bpple" can become "apple" with one change
        assert!(dict.search("bpple".to_string()));
    }

    #[test]
    fn test_exact_one_difference() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string()]);

        // Zero differences: not valid (same word needs exactly one change)
        assert!(!dict.search("hello".to_string()));

        // One difference: valid
        assert!(dict.search("hella".to_string()));
        assert!(dict.search("hallo".to_string()));
        assert!(dict.search("jello".to_string()));

        // Different length: not valid
        assert!(!dict.search("hell".to_string()));
        assert!(!dict.search("hellos".to_string()));
    }
}
