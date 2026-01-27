impl Solution {
    /// Constructs a permutation matching a DI pattern using greedy extremes.
    ///
    /// # Intuition
    /// For 'I', assign the smallest available number (guarantees increase).
    /// For 'D', assign the largest available (guarantees decrease).
    ///
    /// # Approach
    /// Maintain low and high pointers. For each character, push low on 'I'
    /// and high on 'D'. Push the remaining value at the end.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let mut result = Vec::with_capacity(n + 1);
        let (mut lo, mut hi) = (0i32, n as i32);

        for b in s.bytes() {
            if b == b'I' {
                result.push(lo);
                lo += 1;
            } else {
                result.push(hi);
                hi -= 1;
            }
        }
        result.push(lo);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idid() {
        let result = Solution::di_string_match("IDID".to_string());
        assert_eq!(result.len(), 5);
        for i in 0..4 {
            if b"IDID"[i] == b'I' {
                assert!(result[i] < result[i + 1]);
            } else {
                assert!(result[i] > result[i + 1]);
            }
        }
    }

    #[test]
    fn test_iii() {
        assert_eq!(
            Solution::di_string_match("III".to_string()),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test_ddd() {
        assert_eq!(
            Solution::di_string_match("DDI".to_string()),
            vec![3, 2, 0, 1]
        );
    }
}
