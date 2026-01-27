use std::collections::HashSet;

impl Solution {
    /// Finds values that appear in at least two of three arrays.
    ///
    /// # Intuition
    /// Deduplicate each array, then count how many arrays contain each value.
    /// Values with count >= 2 are in the result.
    ///
    /// # Approach
    /// 1. Deduplicate each array into a set.
    /// 2. Increment a frequency counter for each unique value per array.
    /// 3. Collect values with frequency >= 2.
    ///
    /// # Complexity
    /// - Time: O(n1 + n2 + n3)
    /// - Space: O(n1 + n2 + n3)
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut freq = [0u8; 101];

        for set in [nums1, nums2, nums3] {
            let unique: HashSet<i32> = set.into_iter().collect();
            for val in unique {
                freq[val as usize] += 1;
            }
        }

        (1..=100)
            .filter(|&i| freq[i] >= 2)
            .map(|i| i as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut result = Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]);
        result.sort_unstable();
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(
            Solution::two_out_of_three(vec![1], vec![2], vec![3]),
            vec![]
        );
    }
}
