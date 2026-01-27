/// Magic dictionary supporting search with exactly one character difference.
///
/// Uses a trie to efficiently search for words that differ by exactly one
/// character from the query.
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
