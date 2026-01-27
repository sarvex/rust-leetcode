impl Solution {
    /// Finds the maximum sum of any trionic subarray using three-state DP
    ///
    /// # Intuition
    /// A trionic subarray follows an "up-down-up" pattern: strictly increasing,
    /// then strictly decreasing, then strictly increasing again. Each phase must
    /// have at least 2 elements, with boundary elements shared between adjacent phases.
    ///
    /// # Approach
    /// Use dynamic programming with three states tracking each phase:
    /// - s0: best sum of strictly increasing subarray (length >= 2) ending at current
    /// - s1: best sum of up-down subarray (up >= 2, down >= 2) ending at current
    /// - s2: best sum of trionic subarray (all three phases >= 2) ending at current
    ///
    /// State transitions depend on whether current step is increasing or decreasing:
    /// - Increasing: extend s0, or start/extend final up phase (s2) from valid s1
    /// - Decreasing: extend s1, or start down phase from valid s0
    ///
    /// # Complexity
    /// - Time: O(n) - single pass through the array
    /// - Space: O(1) - only tracking three state variables
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(i64::from).collect();
        let n = nums.len();

        const NEG_INF: i64 = i64::MIN / 2;

        let mut s0 = NEG_INF;
        let mut s1 = NEG_INF;
        let mut s2 = NEG_INF;
        let mut ans = NEG_INF;

        (1..n).for_each(|i| {
            let (new_s0, new_s1, new_s2) = match nums[i].cmp(&nums[i - 1]) {
                std::cmp::Ordering::Greater => {
                    (nums[i - 1].max(s0) + nums[i], NEG_INF, s1.max(s2) + nums[i])
                }
                std::cmp::Ordering::Less => (NEG_INF, s0.max(s1) + nums[i], NEG_INF),
                std::cmp::Ordering::Equal => (NEG_INF, NEG_INF, NEG_INF),
            };

            s0 = new_s0;
            s1 = new_s1;
            s2 = new_s2;
            ans = ans.max(s2);
        });

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trionic_with_negatives() {
        assert_eq!(Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]), -4);
    }

    #[test]
    fn trionic_simple_up_down_up() {
        assert_eq!(Solution::max_sum_trionic(vec![1, 4, 2, 7]), 14);
    }

    #[test]
    fn trionic_longer_sequence() {
        assert_eq!(
            Solution::max_sum_trionic(vec![1, 3, 5, 2, 1, 4, 6]),
            1 + 3 + 5 + 2 + 1 + 4 + 6
        );
    }

    #[test]
    fn trionic_minimal_four_elements() {
        assert_eq!(Solution::max_sum_trionic(vec![1, 2, 1, 2]), 6);
    }

    #[test]
    fn trionic_all_negative_values() {
        assert_eq!(
            Solution::max_sum_trionic(vec![-5, -3, -4, -2]),
            -5 + -3 + -4 + -2
        );
    }

    #[test]
    fn trionic_multiple_candidates_selects_max() {
        assert_eq!(
            Solution::max_sum_trionic(vec![1, 5, 3, 10, 2, 8, 4, 20]),
            3 + 10 + 2 + 8 + 4 + 20
        );
    }
}
