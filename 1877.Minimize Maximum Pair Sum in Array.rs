impl Solution {
    /// Minimizes the maximum pair sum by pairing smallest with largest elements
    ///
    /// # Intuition
    /// To minimize the maximum pair sum, we should pair the smallest elements with the largest elements.
    /// This ensures that large values are balanced by small values, preventing any single pair from
    /// having an excessively large sum.
    ///
    /// # Approach
    /// Sort the array, split at the midpoint, then pair elements from the left half with elements from
    /// the reversed right half using iterator combinators for efficient pairing.
    ///
    /// # Complexity
    /// Time: O(n log n) - dominated by sorting
    /// Space: O(1) - only uses a constant amount of extra space
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mid = nums.len() / 2;
        let (left, right) = nums.split_at(mid);

        let mut max = 0;
        for (&a, &b) in left.iter().zip(right.iter().rev()) {
            max = max.max(a + b);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 5, 2, 3];
        assert_eq!(Solution::min_pair_sum(nums), 7);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 5, 4, 2, 4, 6];
        assert_eq!(Solution::min_pair_sum(nums), 8);
    }

    #[test]
    fn test_small_array() {
        let nums = vec![1, 2];
        assert_eq!(Solution::min_pair_sum(nums), 3);
    }

    #[test]
    fn test_four_elements() {
        let nums = vec![1, 4, 3, 2];
        assert_eq!(Solution::min_pair_sum(nums), 5);
    }

    #[test]
    fn test_large_values() {
        let nums = vec![100, 200, 300, 400];
        assert_eq!(Solution::min_pair_sum(nums), 500);
    }

    #[test]
    fn test_duplicate_values() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(Solution::min_pair_sum(nums), 10);
    }

    #[test]
    fn test_unsorted_input() {
        let nums = vec![9, 1, 8, 2, 7, 3];
        assert_eq!(Solution::min_pair_sum(nums), 10);
    }
}
