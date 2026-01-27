impl Solution {
    /// Find the longest balanced substring of consecutive 0s followed by 1s.
    ///
    /// # Intuition
    /// Track consecutive runs of 0s and 1s. When transitioning from 1 to 0
    /// (or at the end), the balanced length is 2 * min(zeros, ones).
    ///
    /// # Approach
    /// 1. Scan the binary string, counting consecutive 0s and 1s
    /// 2. When a 0-run starts after a 1-run, finalize the balanced segment
    /// 3. Track the maximum balanced length throughout
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut zeros = 0;
        let mut ones = 0;
        let mut ans = 0;

        for &b in bytes {
            match b {
                b'0' => {
                    if ones > 0 {
                        zeros = 0;
                        ones = 0;
                    }
                    zeros += 1;
                }
                _ => {
                    ones += 1;
                    ans = ans.max(2 * zeros.min(ones));
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("01000111".into()),
            6
        );
    }

    #[test]
    fn test_no_balanced() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("111".into()),
            0
        );
    }

    #[test]
    fn test_perfect() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("00111".into()),
            4
        );
    }

    #[test]
    fn test_empty_like() {
        assert_eq!(Solution::find_the_longest_balanced_substring("0".into()), 0);
    }
}
