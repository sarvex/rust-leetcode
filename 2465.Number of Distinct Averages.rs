impl Solution {
    /// Counts distinct averages formed by repeatedly pairing min and max elements.
    ///
    /// # Intuition
    /// After sorting, pair the smallest with the largest, second smallest with
    /// second largest, etc. The average is proportional to their sum, so we
    /// just need to count distinct sums.
    ///
    /// # Approach
    /// 1. Sort the array
    /// 2. Pair elements from both ends
    /// 3. Use a boolean array (sums range 0..200) to track distinct sums
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) — fixed-size counter array (201 elements)
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let n = nums.len();
        let mut seen = [false; 201];
        let mut count = 0;

        for i in 0..n / 2 {
            let sum = (nums[i] + nums[n - 1 - i]) as usize;
            if !seen[sum] {
                seen[sum] = true;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::distinct_averages(vec![1, 100]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::distinct_averages(vec![3, 3, 3, 3]), 1);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::distinct_averages(vec![1, 2]), 1);
    }
}
