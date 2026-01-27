impl Solution {
    /// Single-pass tracking of zeros-left plus ones-right score.
    ///
    /// # Intuition
    /// Score = zeros in left + ones in right. Precount total ones, then sweep
    /// left-to-right: each '0' adds to left score, each '1' subtracts from
    /// right score. Track the maximum across all valid split points.
    ///
    /// # Approach
    /// 1. Count total ones in the string
    /// 2. Sweep from index 0 to n-2, adjusting left zeros and right ones
    /// 3. Track the maximum score
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut ones: i32 = bytes.iter().filter(|&&b| b == b'1').count() as i32;
        let mut zeros = 0i32;
        let mut best = 0;

        for &b in &bytes[..bytes.len() - 1] {
            if b == b'0' {
                zeros += 1;
            } else {
                ones -= 1;
            }
            best = best.max(zeros + ones);
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_string() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::max_score("00111".to_string()), 5);
    }

    #[test]
    fn minimal() {
        assert_eq!(Solution::max_score("00".to_string()), 1);
    }
}
