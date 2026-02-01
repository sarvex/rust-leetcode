impl Solution {
    /// Find the earliest window with at most one mismatch using Z-algorithm matches.
    ///
    /// # Intuition
    /// Two strings differ by at most one character if their common prefix and common suffix
    /// cover all but possibly one position.
    ///
    /// # Approach
    /// - Compute the longest common prefix between `pattern` and each `s[i..]` using the
    ///   Z-algorithm on `pattern + '#' + s`.
    /// - Compute the longest common suffix between `pattern` and each `s[i..i+m)` by
    ///   reversing both strings and running the Z-algorithm again.
    /// - For each start index `i`, accept if `prefix == m` (exact match) or
    ///   `prefix + suffix >= m - 1` (at most one mismatch).
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m)
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        fn z_algorithm(bytes: &[u8]) -> Vec<usize> {
            let n = bytes.len();
            let mut z = vec![0_usize; n];
            let (mut left, mut right) = (0_usize, 0_usize);
            for i in 1..n {
                if i <= right {
                    let k = i - left;
                    let remaining = right - i + 1;
                    z[i] = z[k].min(remaining);
                }
                while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
                    z[i] += 1;
                }
                if z[i] > 0 {
                    let new_right = i + z[i] - 1;
                    if new_right > right {
                        left = i;
                        right = new_right;
                    }
                }
            }
            z[0] = n;
            z
        }

        let s_bytes = s.as_bytes();
        let pattern_bytes = pattern.as_bytes();
        let n = s_bytes.len();
        let m = pattern_bytes.len();
        if m > n {
            return -1;
        }

        let mut forward = Vec::with_capacity(m + 1 + n);
        forward.extend_from_slice(pattern_bytes);
        forward.push(b'#');
        forward.extend_from_slice(s_bytes);
        let z_forward = z_algorithm(&forward);

        let rev_pattern: Vec<u8> = pattern_bytes.iter().rev().copied().collect();
        let rev_s: Vec<u8> = s_bytes.iter().rev().copied().collect();
        let mut backward = Vec::with_capacity(m + 1 + n);
        backward.extend_from_slice(&rev_pattern);
        backward.push(b'#');
        backward.extend_from_slice(&rev_s);
        let z_backward = z_algorithm(&backward);

        for i in 0..=n - m {
            let prefix = z_forward[m + 1 + i];
            if prefix == m {
                return i as i32;
            }
            let rev_start = n - i - m;
            let suffix = z_backward[m + 1 + rev_start];
            if prefix + suffix >= m - 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::min_starting_index("abcdefg".to_string(), "bcdffg".to_string()),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_starting_index("ababbababa".to_string(), "bacaba".to_string()),
            4
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::min_starting_index("abcd".to_string(), "dba".to_string()),
            -1
        );
    }

    #[test]
    fn example_four() {
        assert_eq!(Solution::min_starting_index("dde".to_string(), "d".to_string()), 0);
    }

    #[test]
    fn pattern_length_one() {
        assert_eq!(Solution::min_starting_index("xyz".to_string(), "a".to_string()), 0);
    }

    #[test]
    fn no_almost_match() {
        assert_eq!(
            Solution::min_starting_index("aaaaaa".to_string(), "bbb".to_string()),
            -1
        );
    }
}
