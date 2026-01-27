impl Solution {
    /// Verifies words are sorted in an alien dictionary order.
    ///
    /// # Intuition
    /// Map each character to its alien-order rank. Compare adjacent words
    /// lexicographically using these ranks.
    ///
    /// # Approach
    /// Build a rank array from the order string. For each consecutive pair,
    /// compare character-by-character using ranks. If the first differing
    /// character is out of order, or the first word is a prefix of the second
    /// but longer, return false.
    ///
    /// # Complexity
    /// - Time: O(n * L) where n is word count and L is max word length
    /// - Space: O(1) — fixed 26-element rank array
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = [0usize; 26];
        for (i, b) in order.bytes().enumerate() {
            rank[(b - b'a') as usize] = i;
        }

        words.windows(2).all(|pair| {
            let a = pair[0].as_bytes();
            let b = pair[1].as_bytes();
            for i in 0..a.len().min(b.len()) {
                let ra = rank[(a[i] - b'a') as usize];
                let rb = rank[(b[i] - b'a') as usize];
                match ra.cmp(&rb) {
                    std::cmp::Ordering::Less => return true,
                    std::cmp::Ordering::Greater => return false,
                    _ => {}
                }
            }
            a.len() <= b.len()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_sorted() {
        assert!(Solution::is_alien_sorted(
            to_vec(&["hello", "leetcode"]),
            "hlabcdefgijkmnopqrstuvwxyz".to_string(),
        ));
    }

    #[test]
    fn test_not_sorted() {
        assert!(!Solution::is_alien_sorted(
            to_vec(&["word", "world", "row"]),
            "worldabcefghijkmnpqstuvxyz".to_string(),
        ));
    }

    #[test]
    fn test_prefix_rule() {
        assert!(!Solution::is_alien_sorted(
            to_vec(&["apple", "app"]),
            "abcdefghijklmnopqrstuvwxyz".to_string(),
        ));
    }
}
