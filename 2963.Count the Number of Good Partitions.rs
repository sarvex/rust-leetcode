impl Solution {
    /// Counts good partitions by grouping overlapping value ranges.
    ///
    /// # Intuition
    /// Any value that appears multiple times must stay within one subarray. So the array splits
    /// into minimal segments that cover all occurrences of values, and we can only cut between
    /// these segments.
    ///
    /// # Approach
    /// Record the last index of every value. Scan the array, tracking the farthest last index
    /// seen so far. When the scan index reaches this farthest position, a minimal segment ends.
    /// If there are `k` minimal segments, each of the `k - 1` gaps can be cut or merged
    /// independently, yielding `2^(k - 1)` good partitions.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        if nums.is_empty() {
            return 0;
        }

        let last_index: std::collections::HashMap<_, _> =
            nums.iter().enumerate().map(|(i, &v)| (v, i)).collect();

        let segments = nums
            .iter()
            .enumerate()
            .fold((0_i64, 0_usize), |(seg, current_end), (i, &value)| {
                let new_end = current_end.max(last_index[&value]);
                let new_seg = if i == new_end { seg + 1 } else { seg };
                (new_seg, new_end)
            })
            .0;

        if segments <= 1 {
            return 1;
        }

        let mut result = 1_i64;
        let mut base = 2_i64;
        let mut exp = segments - 1;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % MOD;
            }
            base = (base * base) % MOD;
            exp >>= 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::number_of_good_partitions(nums), 8);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(Solution::number_of_good_partitions(nums), 1);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 1, 3];
        assert_eq!(Solution::number_of_good_partitions(nums), 2);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        assert_eq!(Solution::number_of_good_partitions(nums), 1);
    }

    #[test]
    fn test_two_segments() {
        let nums = vec![5, 6, 5, 7, 8];
        assert_eq!(Solution::number_of_good_partitions(nums), 2);
    }

    #[test]
    fn test_all_distinct_five() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::number_of_good_partitions(nums), 16);
    }
}
