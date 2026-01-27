use std::collections::HashMap;

impl Solution {
    /// Finds the intersection of two arrays including duplicates using a frequency map.
    ///
    /// # Intuition
    /// Count occurrences in the first array. For each element in the second array,
    /// if it exists in the map with count > 0, include it and decrement.
    ///
    /// # Approach
    /// 1. Build a frequency map from nums1.
    /// 2. Iterate nums2, collecting elements that exist in the map.
    /// 3. Decrement counts to handle duplicates correctly.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(min(n, m)) for the map
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();
        for &x in &nums1 {
            *counts.entry(x).or_insert(0) += 1;
        }
        let mut result = Vec::new();
        for &x in &nums2 {
            if let Some(count) = counts.get_mut(&x) {
                if *count > 0 {
                    result.push(x);
                    *count -= 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut result = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort();
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn partial_overlap() {
        let mut result = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
