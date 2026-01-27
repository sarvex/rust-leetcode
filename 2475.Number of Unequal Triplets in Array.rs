use std::collections::HashMap;

impl Solution {
    /// Counts triplets (i, j, k) where all three elements are distinct.
    ///
    /// # Intuition
    /// Sort and group by value. For each group, count triplets where one element
    /// comes from elements before the group, one from the group, and one after.
    /// This avoids the O(n^3) brute force.
    ///
    /// # Approach
    /// 1. Count frequency of each value
    /// 2. For each frequency group, the number of valid triplets involving that
    ///    group as the middle value is: left_count * freq * right_count
    /// 3. Accumulate left_count as we iterate through groups
    ///
    /// # Complexity
    /// - Time: O(n) — one pass to count, one pass over distinct values
    /// - Space: O(n) — for frequency map
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let freq: HashMap<i32, usize> = nums.iter().fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

        let mut left = 0usize;
        let mut result = 0;

        for &count in freq.values() {
            let right = n - left - count;
            result += left * count * right;
            left += count;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_all_distinct() {
        assert_eq!(Solution::unequal_triplets(vec![1, 2, 3, 4]), 4);
    }

    #[test]
    fn test_minimum_triplet() {
        assert_eq!(Solution::unequal_triplets(vec![1, 2, 3]), 1);
    }

    #[test]
    fn test_two_values_only() {
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 2, 2]), 0);
    }
}
