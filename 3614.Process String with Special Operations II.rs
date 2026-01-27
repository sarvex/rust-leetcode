impl Solution {
    /// Backward tracing to find k-th character without building the string.
    ///
    /// # Intuition
    /// Since the result can be up to 10^15 characters, we can't build it directly.
    /// Instead, track lengths after each operation, then trace backward from k
    /// to determine which original character it maps to.
    ///
    /// # Approach
    /// 1. Forward pass: compute length after each operation (capped to avoid overflow)
    /// 2. Backward pass: transform k based on each operation in reverse
    ///    - '#': if k >= prev_len, map to first half (k -= prev_len)
    ///    - '%': reverse index (k = len - 1 - k)
    ///    - '*': no change to k
    ///    - letter: if k is the last position, return the letter
    ///
    /// # Complexity
    /// - Time: O(n) for both passes
    /// - Space: O(n) for storing lengths
    pub fn process_str(s: String, k: i64) -> char {
        let s = s.as_bytes();
        let n = s.len();

        const CAP: i64 = 2_000_000_000_000_000;

        let mut lens = vec![0i64; n + 1];

        // Forward pass: compute length after each operation
        for (i, &c) in s.iter().enumerate() {
            let prev = lens[i];
            lens[i + 1] = match c {
                b'*' => (prev - 1).max(0),
                b'#' => {
                    if prev > CAP / 2 {
                        CAP
                    } else {
                        prev * 2
                    }
                }
                b'%' => prev,
                _ => {
                    if prev >= CAP {
                        CAP
                    } else {
                        prev + 1
                    }
                }
            };
        }

        if k >= lens[n] {
            return '.';
        }

        // Backward pass: trace k through operations in reverse
        let mut k = k;

        for i in (0..n).rev() {
            let prev_len = lens[i];
            let curr_len = lens[i + 1];

            match s[i] {
                b'#' => {
                    if k >= prev_len {
                        k -= prev_len;
                    }
                }
                b'%' => {
                    k = curr_len - 1 - k;
                }
                b'*' => {}
                c => {
                    if k == curr_len - 1 {
                        return c as char;
                    }
                }
            }
        }

        '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::process_str("a#b%*".to_string(), 1), 'a');
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::process_str("cd%#*#".to_string(), 3), 'd');
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::process_str("z*#".to_string(), 0), '.');
    }

    #[test]
    fn test_simple_append() {
        assert_eq!(Solution::process_str("abc".to_string(), 0), 'a');
        assert_eq!(Solution::process_str("abc".to_string(), 1), 'b');
        assert_eq!(Solution::process_str("abc".to_string(), 2), 'c');
    }

    #[test]
    fn test_out_of_bounds() {
        assert_eq!(Solution::process_str("ab".to_string(), 5), '.');
    }

    #[test]
    fn test_duplicate() {
        // "ab" -> "abab"
        assert_eq!(Solution::process_str("ab#".to_string(), 0), 'a');
        assert_eq!(Solution::process_str("ab#".to_string(), 2), 'a');
        assert_eq!(Solution::process_str("ab#".to_string(), 3), 'b');
    }

    #[test]
    fn test_reverse() {
        // "abc" -> "cba"
        assert_eq!(Solution::process_str("abc%".to_string(), 0), 'c');
        assert_eq!(Solution::process_str("abc%".to_string(), 2), 'a');
    }

    #[test]
    fn test_delete_all() {
        assert_eq!(Solution::process_str("a*".to_string(), 0), '.');
    }

    #[test]
    fn test_large_k() {
        // "a" -> "aa" -> "aaaa" -> ... (many duplications)
        let s = "a".to_string() + &"#".repeat(50);
        assert_eq!(Solution::process_str(s.clone(), 0), 'a');
        assert_eq!(Solution::process_str(s, 1_000_000_000_000), 'a');
    }
}
