impl Solution {
    /// Computes the sum of subarray ranges using pairwise min/max tracking.
    ///
    /// # Intuition
    /// The range of a subarray is `max - min`. By iterating over all subarrays
    /// anchored at each starting index and maintaining running min and max, we
    /// accumulate the total sum of ranges efficiently.
    ///
    /// # Approach
    /// 1. For each starting index `i`, iterate rightward maintaining a running
    ///    minimum and maximum.
    /// 2. At each extension, add `max - min` to the result.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(1)
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        (0..n)
            .map(|i| {
                let mut min_val = nums[i];
                let mut max_val = nums[i];
                (i + 1..n).fold(0i64, |acc, j| {
                    min_val = min_val.min(nums[j]);
                    max_val = max_val.max(nums[j]);
                    acc + (max_val - min_val) as i64
                })
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(Solution::sub_array_ranges(vec![1, 2, 3]), 4);
    }

    #[test]
    fn test_repeated_elements() {
        assert_eq!(Solution::sub_array_ranges(vec![1, 3, 3]), 4);
    }

    #[test]
    fn test_descending() {
        assert_eq!(Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::sub_array_ranges(vec![1]), 0);
    }
}
