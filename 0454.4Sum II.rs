use std::collections::HashMap;

impl Solution {
    /// Counts four-element tuples summing to zero using hash map partitioning.
    ///
    /// # Intuition
    /// Split the four arrays into two pairs. Store all sums of the first pair
    /// in a hash map, then for each sum of the second pair, look up its negation.
    ///
    /// # Approach
    /// 1. Compute all pairwise sums of nums1 and nums2, counting frequencies.
    /// 2. For each pairwise sum of nums3 and nums4, add the count of its negation.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums1.len() * nums2.len());
        for &a in &nums1 {
            for &b in &nums2 {
                *map.entry(a + b).or_insert(0) += 1;
            }
        }
        nums3
            .iter()
            .flat_map(|&c| nums4.iter().map(move |&d| -(c + d)))
            .filter_map(|target| map.get(&target))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(
            Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]),
            1
        );
    }
}
