impl Solution {
    /// Count valid substrings by tracking deficits and minimal valid starts.
    ///
    /// # Intuition
    /// A substring is valid iff its character counts dominate `word2`. If we track deficits
    /// (negative counts) for each letter, then the window is valid exactly when every
    /// deficit reaches zero.
    ///
    /// # Approach
    /// - Initialize a count array with negative requirements from `word2`.
    /// - Track `missing_types`, the number of letters whose count is non-zero.
    /// - Extend the right end; when a count reaches zero, decrement `missing_types`.
    /// - While all requirements are met, shrink from the left to find the first invalid
    ///   start; that index equals the number of valid substrings ending at the current right.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        if word2.len() > word1.len() {
            return 0;
        }

        let mut counts = [0_i32; 26];
        for byte in word2 {
            counts[(byte - b'a') as usize] -= 1;
        }

        let mut missing_types = counts.iter().map(|&count| u8::from(count != 0)).sum::<u8>();
        let mut start = 0_usize;
        let mut result = 0_i64;

        for &byte in &word1 {
            let slot = &mut counts[(byte - b'a') as usize];
            *slot += 1;
            missing_types -= u8::from(*slot == 0);

            while missing_types == 0 {
                let slot = &mut counts[(word1[start] - b'a') as usize];
                missing_types += u8::from(*slot == 0);
                *slot -= 1;
                start += 1;
            }

            result += start as i64;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::valid_substring_count("bcca".to_string(), "abc".to_string()),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()),
            10
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()),
            0
        );
    }

    #[test]
    fn word2_longer_than_word1() {
        assert_eq!(
            Solution::valid_substring_count("ab".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn repeated_characters() {
        assert_eq!(
            Solution::valid_substring_count("aaaaa".to_string(), "aa".to_string()),
            10
        );
    }
}
