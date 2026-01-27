impl Solution {
    /// Finds the minimum characters to take from both ends to get k of each 'a', 'b', 'c'.
    ///
    /// # Intuition
    /// Equivalently, find the longest middle substring we can skip such that the
    /// remaining characters (from both ends) still contain at least k of each.
    ///
    /// # Approach
    /// 1. Count total occurrences of each character
    /// 2. If any character has fewer than k, return -1
    /// 3. Sliding window: expand right, shrink left when any character in the
    ///    window causes the remaining count to drop below k
    /// 4. Answer = n - max_window_size
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn take_characters(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut total = [0i32; 3];

        for &b in bytes {
            total[(b - b'a') as usize] += 1;
        }

        if total.iter().any(|&c| c < k) {
            return -1;
        }

        let mut window = [0i32; 3];
        let mut max_skip = 0;
        let mut left = 0;

        for right in 0..n {
            window[(bytes[right] - b'a') as usize] += 1;

            while (0..3).any(|c| total[c] - window[c] < k) {
                window[(bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }

            max_skip = max_skip.max(right - left + 1);
        }

        (n - max_skip) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::take_characters("a".to_string(), 1), -1);
    }

    #[test]
    fn test_zero_k() {
        assert_eq!(Solution::take_characters("abc".to_string(), 0), 0);
    }

    #[test]
    fn test_exact_count() {
        assert_eq!(Solution::take_characters("abc".to_string(), 1), 3);
    }
}
