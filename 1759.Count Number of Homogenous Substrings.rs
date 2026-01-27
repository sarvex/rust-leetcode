impl Solution {
    /// Counts homogenous substrings by tracking consecutive character runs.
    ///
    /// # Intuition
    /// A run of length k contributes k*(k+1)/2 homogenous substrings. Track the
    /// start of each run and accumulate contributions incrementally.
    ///
    /// # Approach
    /// 1. Iterate with a sliding start pointer `i`.
    /// 2. When the character changes, reset `i` to the current position.
    /// 3. Add `j - i + 1` at each step (number of homogenous substrings ending here).
    /// 4. Take results modulo 10^9 + 7.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_homogenous(s: String) -> i32 {
        const MOD: usize = 1_000_000_007;
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut ans = 0usize;
        for j in 0..bytes.len() {
            if bytes[j] != bytes[start] {
                start = j;
            }
            ans = (ans + j - start + 1) % MOD;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::count_homogenous("abbcccaa".to_string()), 13);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::count_homogenous("xy".to_string()), 2);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::count_homogenous("z".to_string()), 1);
    }
}
