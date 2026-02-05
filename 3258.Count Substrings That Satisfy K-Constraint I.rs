impl Solution {
    /// Counts substrings where at least one of 0s or 1s count is at most k.
    ///
    /// # Intuition
    /// A valid substring has at most k zeros OR at most k ones. Use sliding window
    /// to maintain the smallest valid window ending at each position. The number
    /// of valid substrings ending at position r equals r - l + 1.
    ///
    /// # Approach
    /// 1. Maintain counts of 0s and 1s in current window [l, r].
    /// 2. Expand r, incrementing the count for the new character.
    /// 3. While both counts exceed k, shrink from left by incrementing l.
    /// 4. All substrings [i, r] for i in [l, r] are valid; add (r - l + 1) to answer.
    ///
    /// # Complexity
    /// - Time: O(n) - each character visited at most twice (once by r, once by l)
    /// - Space: O(1) - only two counters needed
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut cnt = [0; 2];
        let mut l = 0;
        let mut ans = 0;
        let s = s.as_bytes();

        for (r, &c) in s.iter().enumerate() {
            cnt[(c - b'0') as usize] += 1;
            while cnt[0] > k && cnt[1] > k {
                cnt[(s[l] - b'0') as usize] -= 1;
                l += 1;
            }
            ans += r - l + 1;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::count_k_constraint_substrings("10101".to_string(), 1),
            12
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::count_k_constraint_substrings("1010101".to_string(), 2),
            25
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::count_k_constraint_substrings("11111".to_string(), 1),
            15
        );
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(
            Solution::count_k_constraint_substrings("000".to_string(), 1),
            6
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::count_k_constraint_substrings("1".to_string(), 0),
            1
        );
    }
}
