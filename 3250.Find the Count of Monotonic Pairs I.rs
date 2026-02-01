impl Solution {
    /// Prefix-sum DP over feasible `arr1` values.
    ///
    /// # Intuition
    /// Fixing `arr1[i] = a_i` forces `arr2[i] = nums[i] - a_i`. The two monotone constraints
    /// translate into a single lower bound on how much `a_i` can increase between indices.
    ///
    /// # Approach
    /// - Let `a_i` be `arr1[i]`, where `0 <= a_i <= nums[i]`.
    /// - From `arr2` non-increasing: `a_i - a_{i-1} >= nums[i] - nums[i-1]`.
    /// - Combine with `arr1` non-decreasing to get `a_i >= a_{i-1} + max(0, nums[i] - nums[i-1])`.
    /// - Use DP where `dp[v]` is the count of sequences ending with `a_i = v`.
    ///   A prefix sum lets each transition compute a range sum in O(1).
    ///
    /// # Complexity
    /// - Time: O(n * max(nums))
    /// - Space: O(max(nums))
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![1_i64; nums[0] as usize + 1];
        for i in 1..nums.len() {
            let prev_max = nums[i - 1] as usize;
            let curr_max = nums[i] as usize;
            let delta = nums[i] - nums[i - 1];
            let min_inc = if delta > 0 { delta as usize } else { 0 };

            let mut prefix = vec![0_i64; prev_max + 1];
            let mut running = 0_i64;
            for (idx, value) in dp.iter().enumerate() {
                running += *value;
                if running >= MOD {
                    running -= MOD;
                }
                prefix[idx] = running;
            }

            let mut next = vec![0_i64; curr_max + 1];
            for value in 0..=curr_max {
                if value < min_inc {
                    continue;
                }
                let max_prev = (value - min_inc).min(prev_max);
                next[value] = prefix[max_prev];
            }
            dp = next;
        }

        let mut total = 0_i64;
        for value in dp {
            total += value;
            if total >= MOD {
                total -= MOD;
            }
        }
        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 3, 2];
        assert_eq!(Solution::count_of_pairs(nums), 4);
    }

    #[test]
    fn example_two() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(Solution::count_of_pairs(nums), 126);
    }

    #[test]
    fn single_element() {
        let nums = vec![3];
        assert_eq!(Solution::count_of_pairs(nums), 4);
    }

    #[test]
    fn decreasing_total() {
        let nums = vec![2, 1];
        assert_eq!(Solution::count_of_pairs(nums), 3);
    }

    #[test]
    fn increasing_gap() {
        let nums = vec![1, 3];
        assert_eq!(Solution::count_of_pairs(nums), 3);
    }
}
