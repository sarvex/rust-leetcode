impl Solution {
    /// Find the k-th bit in the n-th binary string defined recursively.
    ///
    /// # Intuition
    /// The string `S_n` has length `2^n - 1` and is built as:
    /// `S_n = S_{n-1} + "1" + reverse(invert(S_{n-1}))`.
    /// This structure is perfectly symmetric around the middle bit, which is
    /// always `'1'`. Every position on the right half corresponds to a
    /// mirrored position on the left half with its bit inverted.
    ///
    /// # Approach
    /// Let `mid = 2^(n-1)` be the index of the middle bit in `S_n`.
    /// - If `n == 1`, the string is `"0"` and we return `'0'`.
    /// - If `k == mid`, we are at the middle bit, which is always `'1'`.
    /// - If `k < mid`, the k-th bit in `S_n` is exactly the k-th bit in
    ///   `S_{n-1}`, so we recurse on `n - 1` with the same `k`.
    /// - If `k > mid`, the k-th bit lies in the `reverse(invert(S_{n-1}))`
    ///   part. Its mirrored index in `S_{n-1}` is `k' = 2^n - k`, and the bit
    ///   is the inversion of the bit at position `k'` in `S_{n-1}`. We recurse
    ///   on `(n - 1, k')` and flip the resulting bit.
    ///
    /// Because `n <= 20`, recursion depth is at most 20 and is perfectly safe.
    ///
    /// # Complexity
    /// - Time: O(n), at most one recursive step per level from `n` down to `1`.
    /// - Space: O(n) due to recursion stack, no extra allocations.
    ///
    /// # Panics
    /// This function does not panic for valid inputs satisfying
    /// `1 <= n <= 20` and `1 <= k <= 2^n - 1`.
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        fn helper(n: i32, k: i32) -> char {
            if n == 1 {
                return '0';
            }

            let mid = 1 << (n - 1);
            if k == mid {
                '1'
            } else if k < mid {
                helper(n - 1, k)
            } else {
                // Mirror index in S_{n-1}
                let mirrored = (1 << n) - k;
                match helper(n - 1, mirrored) {
                    '0' => '1',
                    _ => '0',
                }
            }
        }

        helper(n, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }

    #[test]
    fn test_first_and_last_bits() {
        // For any n, first bit is always '0' and last bit is always '1'
        for n in 1..=5 {
            let len = (1 << n) - 1;
            assert_eq!(Solution::find_kth_bit(n, 1), '0');
            assert_eq!(Solution::find_kth_bit(n, len), '1');
        }
    }

    #[test]
    fn test_middle_bits() {
        // Middle bit (2^(n-1)) is always '1'
        for n in 1..=5 {
            let mid = 1 << (n - 1);
            assert_eq!(Solution::find_kth_bit(n, mid), '1');
        }
    }

    #[test]
    fn test_additional_cases() {
        // Small n, full enumeration via direct construction for validation
        fn build_string(n: i32) -> String {
            if n == 1 {
                return "0".to_string();
            }
            let prev = build_string(n - 1);
            let inverted_reversed: String = prev
                .chars()
                .rev()
                .map(|c| if c == '0' { '1' } else { '0' })
                .collect();
            let mut s = String::with_capacity(prev.len() * 2 + 1);
            s.push_str(&prev);
            s.push('1');
            s.push_str(&inverted_reversed);
            s
        }

        for n in 2..=5 {
            let s = build_string(n);
            let len = s.len();
            assert_eq!(len as i32, (1 << n) - 1);
            for k in 1..=len as i32 {
                assert_eq!(
                    Solution::find_kth_bit(n, k),
                    s.as_bytes()[(k - 1) as usize] as char
                );
            }
        }
    }
}

