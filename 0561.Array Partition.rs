impl Solution {
    /// Maximizes the sum of min(a, b) pairs by sorting.
    ///
    /// # Intuition
    /// Sorting and pairing consecutive elements minimizes "wasted" value,
    /// since each pair's minimum is as large as possible.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Sum every other element (indices 0, 2, 4, ...).
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
    }

    #[test]
    fn test_six_elements() {
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
