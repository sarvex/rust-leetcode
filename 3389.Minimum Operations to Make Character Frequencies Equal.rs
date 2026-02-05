impl Solution {
    /// Minimizes operations to equalize character frequencies via target-sweep DP.
    ///
    /// # Intuition
    /// Every character must appear the same number of times (or not at all).
    /// Operations: delete, insert, or change to the next letter (cost 1 each).
    /// Free chars from deleting one letter can be forwarded to help the next.
    ///
    /// # Approach
    /// 1. Count character frequencies.
    /// 2. For each candidate target frequency 0..=n, run a linear DP over the
    ///    26 letters tracking (free_chars, cost) for "set to target" vs "set to zero".
    /// 3. Return the global minimum cost.
    ///
    /// # Complexity
    /// - Time: O(n × 26) where n is string length
    /// - Space: O(1)
    pub fn make_string_good(s: String) -> i32 {
        let counts = s.as_bytes().iter().fold([0i32; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        });

        (0..=s.len() as i32)
            .map(|target| Self::ops_needed(&counts, target))
            .min()
            .unwrap_or(0)
    }

    fn ops_needed(counts: &[i32; 26], target: i32) -> i32 {
        let mut prev_to_target = (0, 0);
        let mut prev_to_zero = (0, 0);

        for &count in counts {
            let to_zero = (count, count + prev_to_target.1.min(prev_to_zero.1));

            let to_target = if count < target {
                let from_target = (target - count - prev_to_target.0).max(0) + prev_to_target.1;
                let from_zero = (target - count - prev_to_zero.0).max(0) + prev_to_zero.1;
                (0, from_target.min(from_zero))
            } else {
                let excess = count - target;
                (excess, prev_to_target.1.min(prev_to_zero.1) + excess)
            };

            prev_to_target = to_target;
            prev_to_zero = to_zero;
        }

        prev_to_target.1.min(prev_to_zero.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn single_deletion_equalizes() {
        assert_eq!(Solution::make_string_good("acab".to_string()), 1);
    }

    #[test]
    fn already_equal_frequencies() {
        assert_eq!(Solution::make_string_good("wddw".to_string()), 0);
    }

    #[test]
    fn multiple_ops_needed() {
        assert_eq!(Solution::make_string_good("aaabc".to_string()), 2);
    }

    #[test]
    fn mixed_repeated_chars() {
        assert_eq!(Solution::make_string_good("gigigjjggjjgg".to_string()), 3);
    }

    #[test]
    fn complex_frequency_adjustment() {
        assert_eq!(
            Solution::make_string_good("accdddddbebbabbe".to_string()),
            5
        );
    }

    #[test]
    fn all_same_needs_zero_ops() {
        assert_eq!(Solution::make_string_good("aaa".to_string()), 0);
    }

    #[test]
    fn two_distinct_balanced() {
        assert_eq!(Solution::make_string_good("ab".to_string()), 0);
    }
}
