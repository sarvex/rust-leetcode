impl Solution {
    /// Count prefix-suffix pairs using KMP borders and a trie.
    ///
    /// # Intuition
    /// A previous word matches iff it is a border (prefix = suffix) of the
    /// current word. Borders are obtained from the KMP prefix function.
    ///
    /// # Approach
    /// Maintain a trie of all prior words with an end counter at each node.
    /// For each word:
    /// - Build the KMP prefix function and mark all border lengths.
    /// - Walk the trie along the word's prefix once; at each position, if the
    ///   prefix length is a border, add the stored end count.
    /// - Insert the word into the trie.
    ///
    /// # Complexity
    /// - Time: O(total_length)
    /// - Space: O(total_length)
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut trie = Trie::new();
        let mut answer = 0i64;

        for word in words {
            let prefix = Self::build_prefix(word.as_bytes());
            let borders = Self::build_borders(&prefix);
            answer += trie.count_borders(&word, &borders);
            trie.insert(&word);
        }

        answer
    }

    fn build_prefix(bytes: &[u8]) -> Vec<usize> {
        let n = bytes.len();
        let mut prefix = vec![0usize; n];
        for i in 1..n {
            let mut j = prefix[i - 1];
            while j > 0 && bytes[i] != bytes[j] {
                j = prefix[j - 1];
            }
            if bytes[i] == bytes[j] {
                j += 1;
            }
            prefix[i] = j;
        }
        prefix
    }

    fn build_borders(prefix: &[usize]) -> Vec<bool> {
        let n = prefix.len();
        let mut borders = vec![false; n + 1];
        let mut len = n;
        while len > 0 {
            borders[len] = true;
            len = prefix[len - 1];
        }
        borders
    }
}

struct Trie {
    next: Vec<[i32; 26]>,
    end_count: Vec<i64>,
}

impl Trie {
    fn new() -> Self {
        Self {
            next: vec![[-1; 26]],
            end_count: vec![0],
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = 0usize;
        for &byte in word.as_bytes() {
            let idx = (byte - b'a') as usize;
            let child = self.next[node][idx];
            if child == -1 {
                let new_index = self.next.len() as i32;
                self.next[node][idx] = new_index;
                self.next.push([-1; 26]);
                self.end_count.push(0);
                node = new_index as usize;
            } else {
                node = child as usize;
            }
        }
        self.end_count[node] += 1;
    }

    fn count_borders(&self, word: &str, borders: &[bool]) -> i64 {
        let mut node = 0usize;
        let mut total = 0i64;
        for (i, &byte) in word.as_bytes().iter().enumerate() {
            let idx = (byte - b'a') as usize;
            let child = self.next[node][idx];
            if child == -1 {
                break;
            }
            node = child as usize;
            if borders[i + 1] {
                total += self.end_count[node];
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec!["a", "aba", "ababa", "aa"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 4);
    }

    #[test]
    fn test_example_2() {
        let words = vec!["pa", "papa", "ma", "mama"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 2);
    }

    #[test]
    fn test_example_3() {
        let words = vec!["abab", "ab"].into_iter().map(String::from).collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 0);
    }

    #[test]
    fn test_all_same_single_char() {
        let words = vec!["a", "a", "a"].into_iter().map(String::from).collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 3);
    }

    #[test]
    fn test_mixed_duplicates() {
        let words = vec!["abc", "ab", "abc", "bc", "abc"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 3);
    }
}
