impl Solution {
    /// Finds the maximum sum of any trionic subarray.
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

        let mut s0 = NEG_INF; // increasing with len >= 2
        let mut s1 = NEG_INF; // up-down with up >= 2, down >= 2
        let mut s2 = NEG_INF; // trionic with all phases >= 2
        let mut ans = NEG_INF;

        (1..n).for_each(|i| {
            let (new_s0, new_s1, new_s2) = match nums[i].cmp(&nums[i - 1]) {
                std::cmp::Ordering::Greater => {
                    // Increasing step
                    (
                        nums[i - 1].max(s0) + nums[i], // start new or extend increasing
                        NEG_INF,                       // can't continue down with increase
                        s1.max(s2) + nums[i],          // start final_up from s1 or extend s2
                    )
                }
                std::cmp::Ordering::Less => {
                    // Decreasing step
                    (
                        NEG_INF,              // can't continue up with decrease
                        s0.max(s1) + nums[i], // start down from s0 or extend s1
                        NEG_INF,              // can't continue final_up with decrease
                    )
                }
                std::cmp::Ordering::Equal => {
                    // Equal breaks all patterns
                    (NEG_INF, NEG_INF, NEG_INF)
                }
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
    fn example_1() {
        assert_eq!(
            Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]),
            -4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_sum_trionic(vec![1, 4, 2, 7]), 14);
    }

    #[test]
    fn longer_trionic() {
        // [1, 3, 5, 2, 1, 4, 6] - up: [1,3,5], down: [5,2,1], up: [1,4,6]
        assert_eq!(
            Solution::max_sum_trionic(vec![1, 3, 5, 2, 1, 4, 6]),
            1 + 3 + 5 + 2 + 1 + 4 + 6
        );
    }

    #[test]
    fn minimal_trionic() {
        // Minimum 4 elements: up=[a,b], down=[b,c], up=[c,d]
        assert_eq!(Solution::max_sum_trionic(vec![1, 2, 1, 2]), 6);
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::max_sum_trionic(vec![-5, -3, -4, -2]),
            -5 + -3 + -4 + -2
        );
    }

    #[test]
    fn multiple_trionics() {
        // Multiple valid trionics, should find the maximum sum
        assert_eq!(
            Solution::max_sum_trionic(vec![1, 5, 3, 10, 2, 8, 4, 20]),
            3 + 10 + 2 + 8 + 4 + 20
        );
    }
}
