impl Solution {
    /// Single-pass DP to find minimum deletions for a balanced string.
    ///
    /// # Intuition
    /// A balanced string has all 'a's before all 'b's. When we encounter an 'a' after seeing
    /// some 'b's, we must either delete this 'a' or delete all preceding 'b's.
    ///
    /// # Approach
    /// Track `b_count` (number of 'b's seen) and `min_del` (minimum deletions so far).
    /// For each 'a', update: `min_del = min(min_del + 1, b_count)` — either delete this 'a'
    /// or reset by deleting all 'b's seen so far.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_deletions(s: String) -> i32 {
        let (mut b_count, mut min_del) = (0, 0);
        for &c in s.as_bytes() {
            if c == b'b' {
                b_count += 1;
            } else {
                min_del = (min_del + 1).min(b_count);
            }
        }
        min_del
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
    }

    #[test]
    fn test_all_a() {
        assert_eq!(Solution::minimum_deletions("aaa".to_string()), 0);
    }

    #[test]
    fn test_all_b() {
        assert_eq!(Solution::minimum_deletions("bbb".to_string()), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::minimum_deletions("a".to_string()), 0);
        assert_eq!(Solution::minimum_deletions("b".to_string()), 0);
    }

    #[test]
    fn test_ab_only() {
        assert_eq!(Solution::minimum_deletions("ab".to_string()), 0);
    }

    #[test]
    fn test_ba_only() {
        assert_eq!(Solution::minimum_deletions("ba".to_string()), 1);
    }
}
