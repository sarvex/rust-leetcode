impl Solution {
    /// Weighted sum of reversed-alphabet ranks by 1-indexed positions.
    ///
    /// # Intuition
    /// Each letter's contribution is independent: its reversed-alphabet value
    /// (`'a'` → 26, `'z'` → 1) multiplied by its 1-indexed position. The
    /// reverse degree is simply the sum of these products.
    ///
    /// # Approach
    /// Walk the bytes once. For character `c` at 0-based index `i`, add
    /// `(b'z' - c + 1) * (i + 1)` to the running total.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes()
            .enumerate()
            .map(|(i, c)| (b'z' - c + 1) as i32 * (i as i32 + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::reverse_degree("abc".to_string()), 148);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::reverse_degree("zaza".to_string()), 160);
    }

    #[test]
    fn test_single_a() {
        assert_eq!(Solution::reverse_degree("a".to_string()), 26);
    }

    #[test]
    fn test_single_z() {
        assert_eq!(Solution::reverse_degree("z".to_string()), 1);
    }

    #[test]
    fn test_all_as() {
        // 26 * (1 + 2 + 3) = 26 * 6 = 156
        assert_eq!(Solution::reverse_degree("aaa".to_string()), 156);
    }

    #[test]
    fn test_alphabet() {
        let s: String = ('a'..='z').collect();
        let expected: i32 = (0..26).map(|i| (26 - i) * (i + 1)).sum();
        assert_eq!(Solution::reverse_degree(s), expected);
    }

    #[test]
    fn test_max_length_all_a() {
        let n = 1000;
        let s = "a".repeat(n);
        // 26 * n * (n + 1) / 2
        let expected = 26 * n as i32 * (n as i32 + 1) / 2;
        assert_eq!(Solution::reverse_degree(s), expected);
    }
}
