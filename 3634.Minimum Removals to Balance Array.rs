impl Solution {
    /// For each possible minimum in the sorted array, count how many elements we can keep
    /// (all in [min, min*k]), then minimize removals = n - max_kept.
    ///
    /// # Intuition
    /// After removal, the remaining set has some min and max with max ≤ min·k. So we want the
    /// largest subset whose max ≤ min·k. In sorted order, for a fixed minimum at index i, we can
    /// keep all elements from i up to the last j with nums[j] ≤ nums[i]*k — a contiguous segment.
    ///
    /// # Approach
    /// Sort the array. For each index i as candidate minimum, use partition_point to count
    /// elements in nums[i..] with value ≤ nums[i]*k (i64 to avoid overflow). Take the maximum
    /// kept count; answer is n - max_kept.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(log n) for sort.
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        let k64 = k as i64;
        let max_kept = (0..n)
            .map(|i| {
                let limit = nums[i] as i64 * k64;
                nums[i..].partition_point(|&x| x as i64 <= limit)
            })
            .max()
            .unwrap_or(0);
        return n as i32 - max_kept as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_removal(vec![2, 1, 5], 2), 1);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::min_removal(vec![1, 6, 2, 9], 3), 2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::min_removal(vec![4, 6], 2), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_removal(vec![7], 1), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::min_removal(vec![3, 3, 3], 2), 0);
    }

    #[test]
    fn test_remove_all_but_one() {
        assert_eq!(Solution::min_removal(vec![1, 100], 1), 1);
    }
}
