impl Solution {
    /// One-pass DP that only blocks taking adjacent houses with equal color.
    ///
    /// # Intuition
    /// This is a variant of House Robber where adjacency is only forbidden when
    /// the two neighboring houses have the same color. So at each index we only
    /// need to know:
    /// - best sum if current house is taken (`take`)
    /// - best sum if current house is skipped (`skip`)
    ///
    /// # Approach
    /// For each house `i`:
    /// - `skip_new = max(take, skip)` (skip current, keep best prefix result).
    /// - If `colors[i] == colors[i - 1]`, taking `i` forces `i - 1` to be skipped:
    ///   `take_new = nums[i] + skip`.
    /// - Otherwise, there is no conflict with `i - 1`:
    ///   `take_new = nums[i] + max(take, skip)`.
    ///
    /// Initialize with index `0`: `take = nums[0]`, `skip = 0`, then scan once.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn rob(nums: Vec<i32>, colors: Vec<i32>) -> i64 {
        if nums.is_empty() {
            return 0;
        }

        let mut take = i64::from(nums[0]);
        let mut skip = 0_i64;

        for i in 1..nums.len() {
            let best_prev = take.max(skip);
            let skip_new = best_prev;
            let take_new = if colors[i] == colors[i - 1] {
                i64::from(nums[i]) + skip
            } else {
                i64::from(nums[i]) + best_prev
            };

            take = take_new;
            skip = skip_new;
        }

        take.max(skip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::rob(vec![1, 4, 3, 5], vec![1, 1, 2, 2]), 9);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::rob(vec![3, 1, 2, 4], vec![2, 3, 2, 2]), 8);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::rob(vec![10, 1, 3, 9], vec![1, 1, 1, 2]), 22);
    }

    #[test]
    fn test_all_same_color() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1], vec![5, 5, 5, 5, 5]), 12);
    }

    #[test]
    fn test_all_different_colors() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1], vec![1, 2, 3, 4, 5]), 22);
    }

    #[test]
    fn test_single_house() {
        assert_eq!(Solution::rob(vec![42], vec![9]), 42);
    }
}
