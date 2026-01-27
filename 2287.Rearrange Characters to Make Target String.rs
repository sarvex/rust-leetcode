impl Solution {
    /// Finds maximum copies of target formable by rearranging characters from s.
    ///
    /// # Intuition
    /// The bottleneck is the character with the smallest ratio of available count
    /// to required count across all characters in the target.
    ///
    /// # Approach
    /// 1. Count character frequencies in both `s` and `target` using byte arrays
    /// 2. For each character needed in target, compute `available / needed`
    /// 3. Return the minimum ratio across all required characters
    ///
    /// # Complexity
    /// - Time: O(n + m) where n = s.len(), m = target.len()
    /// - Space: O(1) — two fixed-size arrays of 26 elements
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut source_freq = [0i32; 26];
        let mut target_freq = [0i32; 26];

        for &b in s.as_bytes() {
            source_freq[(b - b'a') as usize] += 1;
        }
        for &b in target.as_bytes() {
            target_freq[(b - b'a') as usize] += 1;
        }

        (0..26)
            .filter(|&i| target_freq[i] > 0)
            .map(|i| source_freq[i] / target_freq[i])
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::rearrange_characters("ilovecodingonleetcode".to_string(), "code".to_string()),
            2
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::rearrange_characters("abcba".to_string(), "abc".to_string()),
            1
        );
    }

    #[test]
    fn test_example_three() {
        assert_eq!(
            Solution::rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()),
            1
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            Solution::rearrange_characters("xyz".to_string(), "abc".to_string()),
            0
        );
    }
}
