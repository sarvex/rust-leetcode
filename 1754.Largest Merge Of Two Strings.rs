impl Solution {
    /// Greedy merge choosing the lexicographically larger remaining suffix.
    ///
    /// # Intuition
    /// At each step, compare the remaining suffixes of both strings. Pick the
    /// character from whichever suffix is lexicographically greater to ensure
    /// the result is as large as possible.
    ///
    /// # Approach
    /// 1. Use two indices into the byte slices.
    /// 2. Compare remaining suffixes; pick from the larger one.
    /// 3. Append any leftover characters from either string.
    ///
    /// # Complexity
    /// - Time: O((m + n)²) worst case due to suffix comparison
    /// - Space: O(m + n)
    pub fn largest_merge(word1: String, word2: String) -> String {
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let mut result = String::with_capacity(w1.len() + w2.len());
        let (mut i, mut j) = (0, 0);
        while i < w1.len() && j < w2.len() {
            if w1[i..] > w2[j..] {
                result.push(w1[i] as char);
                i += 1;
            } else {
                result.push(w2[j] as char);
                j += 1;
            }
        }
        w1[i..].iter().for_each(|&b| result.push(b as char));
        w2[j..].iter().for_each(|&b| result.push(b as char));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::largest_merge("cabaa".to_string(), "bcaaa".to_string()),
            "cbcabaaaaa"
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::largest_merge("abcabc".to_string(), "abdcaba".to_string()),
            "abdcabcabcaba"
        );
    }
}
