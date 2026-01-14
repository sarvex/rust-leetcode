impl Solution {
    /// # Substring With Largest Variance
    ///
    /// Finds the largest variance among all substrings where variance is defined
    /// as the difference between the max and min character frequencies.
    ///
    /// # Intuition
    /// The variance in a substring equals max_freq - min_freq of any two characters.
    /// Instead of checking all substrings (O(n²)), we fix a pair of characters
    /// (major, minor) and find the substring maximizing count(major) - count(minor).
    ///
    /// # Approach
    /// Use a modified Kadane's algorithm for each character pair:
    /// - Treat major character as +1, minor as -1, ignore others
    /// - Track remaining minor characters to know when we can safely reset
    /// - Only reset sum when negative AND more minors exist ahead
    /// - Early exit when theoretical maximum is reached
    ///
    /// # Complexity
    /// - Time: O(k² × n) where k = unique chars (at most 26)
    /// - Space: O(1) constant extra space
    pub fn largest_variance(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();

        if n < 2 {
            return 0;
        }

        // Count frequencies
        let mut freq = [0i32; 26];
        for &b in bytes {
            freq[(b - b'a') as usize] += 1;
        }

        // Collect unique characters with their frequencies
        let mut unique: [(u8, i32); 26] = [(0, 0); 26];
        let mut num_unique = 0usize;

        for c in 0u8..26 {
            if freq[c as usize] > 0 {
                unique[num_unique] = (c, freq[c as usize]);
                num_unique += 1;
            }
        }

        if num_unique < 2 {
            return 0;
        }

        // Theoretical maximum: max_freq - 1 (need at least 1 of the minor char)
        let theoretical_max = unique[..num_unique]
            .iter()
            .map(|&(_, f)| f)
            .max()
            .unwrap_or(0)
            - 1;

        let mut max_variance = 0;

        'outer: for i in 0..num_unique {
            let (major, _) = unique[i];
            let major_byte = major + b'a';

            for j in 0..num_unique {
                if i == j {
                    continue;
                }

                let (minor, minor_freq) = unique[j];
                let minor_byte = minor + b'a';

                let mut sum = 0i32;
                let mut minor_count = 0i32;
                let mut remaining = minor_freq;

                for &b in bytes {
                    if b == major_byte {
                        sum += 1;
                    } else if b == minor_byte {
                        remaining -= 1;
                        minor_count += 1;
                        sum -= 1;
                    } else {
                        continue;
                    }

                    if minor_count > 0 && sum > max_variance {
                        max_variance = sum;
                        if max_variance >= theoretical_max {
                            break 'outer;
                        }
                    }

                    if sum < 0 && remaining > 0 {
                        sum = 0;
                        minor_count = 0;
                    }
                }
            }
        }

        max_variance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::largest_variance("aababbb".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::largest_variance("abcde".to_string()), 0);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::largest_variance("a".to_string()), 0);
    }

    #[test]
    fn test_two_same_characters() {
        assert_eq!(Solution::largest_variance("aa".to_string()), 0);
    }

    #[test]
    fn test_two_different_characters() {
        assert_eq!(Solution::largest_variance("ab".to_string()), 0);
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(Solution::largest_variance("aaab".to_string()), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::largest_variance("aaaaaaa".to_string()), 0);
    }

    #[test]
    fn test_alternating() {
        assert_eq!(Solution::largest_variance("ababab".to_string()), 1);
    }

    #[test]
    fn test_minor_at_boundaries() {
        assert_eq!(Solution::largest_variance("baaaa".to_string()), 3);
    }

    #[test]
    fn test_complex_string() {
        assert_eq!(Solution::largest_variance("lripaa".to_string()), 1);
    }

    #[test]
    fn test_multiple_minors_between_majors() {
        assert_eq!(Solution::largest_variance("baaabbbaaaaaab".to_string()), 6);
    }
}
