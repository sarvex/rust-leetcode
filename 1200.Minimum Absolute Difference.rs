impl Solution {
    /// Sorting with Two-Pass Iterator Chain
    ///
    /// # Intuition
    /// The minimum absolute difference between any two elements in an array must
    /// occur between adjacent elements when the array is sorted. Two simple passes
    /// with predictable branches outperform a single complex pass due to better
    /// branch prediction and iterator fusion optimization.
    ///
    /// # Approach
    /// 1. Sort the array using unstable sort for better performance
    /// 2. First pass: find minimum difference between adjacent elements
    /// 3. Second pass: collect all pairs matching the minimum difference
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting, O(n) for two passes
    /// - Space: O(1) auxiliary (excluding output and sort space)
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();

        let min_diff = arr
            .windows(2)
            .map(|w| w[1] - w[0])
            .min()
            .unwrap_or(i32::MAX);

        arr.windows(2)
            .filter(|w| w[1] - w[0] == min_diff)
            .map(|w| vec![w[0], w[1]])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_elements() {
        let arr = vec![4, 2, 1, 3];
        let expected = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::minimum_abs_difference(arr), expected);
    }

    #[test]
    fn single_pair() {
        let arr = vec![1, 3, 6, 10, 15];
        let expected = vec![vec![1, 3]];
        assert_eq!(Solution::minimum_abs_difference(arr), expected);
    }

    #[test]
    fn negative_numbers() {
        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let expected = vec![vec![-14, -10], vec![19, 23], vec![23, 27]];
        assert_eq!(Solution::minimum_abs_difference(arr), expected);
    }

    #[test]
    fn two_elements() {
        let arr = vec![5, 10];
        let expected = vec![vec![5, 10]];
        assert_eq!(Solution::minimum_abs_difference(arr), expected);
    }
}
