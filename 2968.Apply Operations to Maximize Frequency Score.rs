impl Solution {
    /// Maximizes the most frequent value by equalizing a sorted window to its median.
    ///
    /// # Intuition
    /// The cheapest way to make several numbers equal is to pick a median, because the sum of
    /// absolute differences is minimized there. We want the largest window whose median-cost
    /// fits within the operation budget.
    ///
    /// # Approach
    /// Sort the array, build prefix sums, and use a sliding window. For any window `[left, right]`,
    /// the minimal cost to equalize all elements is the sum of distances to the median index
    /// `mid = (left + right) / 2`. That cost can be computed in O(1) using prefix sums. Move
    /// `right` forward and advance `left` while the cost exceeds `k`, tracking the best length.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn max_frequency_score(nums: Vec<i32>, k: i64) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut nums: Vec<i64> = nums.into_iter().map(i64::from).collect();
        nums.sort_unstable();

        let mut prefix = vec![0_i64; n + 1];
        for (idx, value) in nums.iter().enumerate() {
            prefix[idx + 1] = prefix[idx] + *value;
        }

        let cost = |left: usize, right: usize| -> i64 {
            let mid = (left + right) / 2;
            let median = nums[mid];
            let left_sum = prefix[mid + 1] - prefix[left];
            let right_sum = prefix[right + 1] - prefix[mid + 1];
            let left_count = (mid - left + 1) as i64;
            let right_count = (right - mid) as i64;
            let left_cost = median * left_count - left_sum;
            let right_cost = right_sum - median * right_count;
            left_cost + right_cost
        };

        let mut best = 1_usize;
        let mut left = 0_usize;
        for right in 0..n {
            while left <= right && cost(left, right) > k {
                left += 1;
            }
            let length = right - left + 1;
            if length > best {
                best = length;
            }
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_frequency_score(vec![1, 2, 6, 4], 3), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_frequency_score(vec![1, 4, 4, 2, 4], 0), 3);
    }

    #[test]
    fn test_single_value() {
        assert_eq!(Solution::max_frequency_score(vec![7], 100), 1);
    }

    #[test]
    fn test_full_equalization() {
        assert_eq!(Solution::max_frequency_score(vec![1, 2, 10], 9), 3);
    }

    #[test]
    fn test_even_window_median() {
        assert_eq!(Solution::max_frequency_score(vec![1, 10, 10, 10], 9), 4);
    }
}
