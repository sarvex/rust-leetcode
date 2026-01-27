impl Solution {
    /// Sort by popcount with value as tiebreaker.
    ///
    /// # Intuition
    /// Sorting by a composite key `(count_ones, value)` achieves the required
    /// ordering: primary sort by bit count, secondary sort by value.
    ///
    /// # Approach
    /// 1. Sort using `sort_unstable_by_key` with key `(count_ones, value)`
    /// 2. Return the sorted array
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) in-place sort
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by_key(|&x| (x.count_ones(), x));
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_bits() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
    }

    #[test]
    fn large_values() {
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
