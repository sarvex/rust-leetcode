impl Solution {
    /// Minimum time using prefix matches from the Z-function.
    ///
    /// # Intuition
    /// After `t` steps, the first `n - t * k` characters are forced to be the
    /// original suffix `word[t * k..]`, while the remaining characters can be
    /// chosen arbitrarily. The word reverts exactly when that forced suffix
    /// matches the original prefix.
    ///
    /// # Approach
    /// Precompute the Z-array so that `z[i]` gives the longest prefix match
    /// starting at `i`. For each `t` with shift `s = t * k < n`, the condition
    /// is `z[s] >= n - s`. The first such `t` is minimal. If no shift works,
    /// then `t = ceil(n / k)` removes all original characters, and we can
    /// rebuild the word in one full cycle.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let bytes = word.as_bytes();
        let n = bytes.len();
        let k = k as usize;
        let max_steps = (n + k - 1) / k;

        let z = Self::build_z(bytes);
        for step in 1..max_steps {
            let shift = step * k;
            if z[shift] >= n - shift {
                return step as i32;
            }
        }

        max_steps as i32
    }

    fn build_z(bytes: &[u8]) -> Vec<usize> {
        let n = bytes.len();
        let mut z = vec![0usize; n];
        let (mut left, mut right) = (0usize, 0usize);

        for i in 1..n {
            if i <= right {
                let mirrored = i - left;
                z[i] = z[mirrored].min(right - i + 1);
            }
            while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > right + 1 {
                left = i;
                right = i + z[i] - 1;
            }
        }

        z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let word = "abacaba".to_string();
        assert_eq!(Solution::minimum_time_to_initial_state(word, 3), 2);
    }

    #[test]
    fn test_example_2() {
        let word = "abacaba".to_string();
        assert_eq!(Solution::minimum_time_to_initial_state(word, 4), 1);
    }

    #[test]
    fn test_example_3() {
        let word = "abcbabcd".to_string();
        assert_eq!(Solution::minimum_time_to_initial_state(word, 2), 4);
    }

    #[test]
    fn test_single_char() {
        let word = "a".to_string();
        assert_eq!(Solution::minimum_time_to_initial_state(word, 1), 1);
    }

    #[test]
    fn test_no_prefix_match_k_one() {
        let word = "abcd".to_string();
        assert_eq!(Solution::minimum_time_to_initial_state(word, 1), 4);
    }
}
