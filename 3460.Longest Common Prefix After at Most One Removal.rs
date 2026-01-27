impl Solution {
    /// Finds longest common prefix of s and t after removing at most one char from s.
    ///
    /// # Intuition
    /// Walk both strings in tandem. On the first mismatch, skip the character
    /// in s (the one removal allowed) and continue matching. The matched
    /// length in t is the answer.
    ///
    /// # Approach
    /// 1. Compare s[i] with t[j]; on match advance both pointers.
    /// 2. On first mismatch, set a removal flag and advance only i.
    /// 3. On second mismatch, stop.
    ///
    /// # Complexity
    /// - Time: O(min(n, m))
    /// - Space: O(1)
    pub fn longest_common_prefix(s: String, t: String) -> i32 {
        let (sb, tb) = (s.as_bytes(), t.as_bytes());
        let (n, m) = (sb.len(), tb.len());
        let (mut i, mut j) = (0, 0);
        let mut removed = false;

        while i < n && j < m {
            if sb[i] != tb[j] {
                if removed {
                    break;
                }
                removed = true;
            } else {
                j += 1;
            }
            i += 1;
        }

        j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removal_extends_prefix() {
        assert_eq!(
            Solution::longest_common_prefix("abcde".to_string(), "acde".to_string()),
            4
        );
    }

    #[test]
    fn no_removal_needed() {
        assert_eq!(
            Solution::longest_common_prefix("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn completely_different_strings() {
        assert_eq!(
            Solution::longest_common_prefix("xyz".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn single_char_removal_at_start() {
        assert_eq!(
            Solution::longest_common_prefix("xabc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn empty_result_when_t_empty() {
        assert_eq!(
            Solution::longest_common_prefix("abc".to_string(), String::new()),
            0
        );
    }
}
