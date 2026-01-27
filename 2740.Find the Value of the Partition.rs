impl Solution {
    /// Minimum partition value (difference between adjacent sorted elements).
    ///
    /// # Intuition
    /// Splitting a sorted array between two adjacent elements minimizes the
    /// partition value, which equals max(group1) - min(group2).
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Find the minimum difference between consecutive elements.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_adjacent_difference() {
        assert_eq!(Solution::find_value_of_partition(vec![1, 3, 2, 4]), 1);
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::find_value_of_partition(vec![100, 1]), 99);
    }
}
