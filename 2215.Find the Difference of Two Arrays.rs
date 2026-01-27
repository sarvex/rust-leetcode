use std::collections::HashSet;

impl Solution {
    /// Find elements unique to each of two arrays.
    ///
    /// # Intuition
    /// Convert both arrays to sets, then compute set differences.
    ///
    /// # Approach
    /// 1. Build HashSets from both arrays.
    /// 2. Filter each set for elements not present in the other.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m)
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.iter().copied().collect();
        let set2: HashSet<i32> = nums2.iter().copied().collect();

        vec![
            set1.difference(&set2).copied().collect(),
            set2.difference(&set1).copied().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_difference() {
        let result = Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]);
        let mut r0 = result[0].clone();
        let mut r1 = result[1].clone();
        r0.sort_unstable();
        r1.sort_unstable();
        assert_eq!(r0, vec![1, 3]);
        assert_eq!(r1, vec![4, 6]);
    }

    #[test]
    fn identical_arrays() {
        let result = Solution::find_difference(vec![1, 2, 3], vec![1, 2, 3]);
        assert!(result[0].is_empty());
        assert!(result[1].is_empty());
    }
}
