impl Solution {
    /// Counts incremovable subarrays by keeping an increasing prefix and suffix.
    ///
    /// # Intuition
    /// Removing a subarray leaves a prefix and a suffix. For the remaining array to be strictly
    /// increasing, both parts must be strictly increasing and, if both exist, the last prefix
    /// value must be less than the first suffix value.
    ///
    /// # Approach
    /// Find the longest strictly increasing prefix end `left_max` and the earliest strictly
    /// increasing suffix start `right_min`. Valid removals correspond to choosing a prefix end
    /// `i` in `[-1, left_max]` and a suffix start `j` in `[right_min, n]` with `j >= i + 2`.
    /// For `i >= 0` and `j < n`, we also need `nums[i] < nums[j]`. Because the suffix starting
    /// at `right_min` is strictly increasing, the predicate `nums[i] < nums[j]` is monotone in
    /// `j`, so a two-pointer scan counts all valid `j` for each `i` in O(n).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n <= 1 {
            return n as i64;
        }

        let nums = nums.as_slice();

        let mut left_max = 0_usize;
        while left_max + 1 < n && nums[left_max] < nums[left_max + 1] {
            left_max += 1;
        }

        if left_max + 1 == n {
            let n_i64 = n as i64;
            return n_i64 * (n_i64 + 1) / 2;
        }

        let mut right_min = n - 1;
        while right_min > 0 && nums[right_min - 1] < nums[right_min] {
            right_min -= 1;
        }

        let n_plus_one = (n + 1) as i64;
        let min_prefix_suffix = if right_min > 0 { right_min } else { 1 };
        let mut total = n_plus_one - min_prefix_suffix as i64;

        let mut j_ptr = right_min;
        for i in 0..=left_max {
            let mut start_j = i + 2;
            if start_j < right_min {
                start_j = right_min;
            }
            if start_j > n {
                break;
            }

            if j_ptr < start_j {
                j_ptr = start_j;
            }

            while j_ptr < n && nums[j_ptr] <= nums[i] {
                j_ptr += 1;
            }

            total += n_plus_one - j_ptr as i64;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::incremovable_subarray_count(nums), 10);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![6, 5, 7, 8];
        assert_eq!(Solution::incremovable_subarray_count(nums), 7);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![8, 7, 6, 6];
        assert_eq!(Solution::incremovable_subarray_count(nums), 3);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        assert_eq!(Solution::incremovable_subarray_count(nums), 1);
    }

    #[test]
    fn test_all_equal() {
        let nums = vec![2, 2, 2];
        assert_eq!(Solution::incremovable_subarray_count(nums), 3);
    }

    #[test]
    fn test_large_values() {
        let nums = vec![1, i32::MAX];
        assert_eq!(Solution::incremovable_subarray_count(nums), 3);
    }
}
