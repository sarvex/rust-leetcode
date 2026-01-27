impl Solution {
    /// Prefix sum minimum to determine starting value.
    ///
    /// # Intuition
    /// The starting value must compensate for the worst prefix sum deficit.
    /// If the minimum prefix sum is `m`, the start value is `max(1, 1 - m)`.
    ///
    /// # Approach
    /// 1. Compute running prefix sum, tracking the minimum
    /// 2. Return `max(1, 1 - min_prefix_sum)`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let min_prefix = nums
            .iter()
            .scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            })
            .min()
            .unwrap_or(0);
        1.max(1 - min_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_compensation() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    }

    #[test]
    fn all_positive() {
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
    }

    #[test]
    fn mixed() {
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}
