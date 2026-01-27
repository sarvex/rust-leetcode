impl Solution {
    /// Counts minimum increments to make the array strictly increasing.
    ///
    /// # Intuition
    /// Each element must be at least one more than the previous. Track
    /// the required minimum and accumulate the difference.
    ///
    /// # Approach
    /// 1. Fold over the array, maintaining the current required minimum.
    /// 2. For each element, add the deficit (if any) to the operation count.
    /// 3. Update the minimum to be one more than the effective value.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(ops, prev_min), &val| {
                let needed = (prev_min + 1 - val).max(0);
                (ops + needed, val.max(prev_min + 1))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needs_operations() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
    }

    #[test]
    fn test_already_increasing() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
    }
}
