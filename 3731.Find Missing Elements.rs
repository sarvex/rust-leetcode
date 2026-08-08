use std::collections::HashSet;

impl Solution {
    /// Find all integers missing from the range [min, max] of nums.
    ///
    /// # Intuition
    /// The original range spans from the minimum to the maximum value in nums.
    /// Any integer in that range not present in nums is missing. A HashSet gives
    /// O(1) membership checks, so a single pass over the range suffices.
    ///
    /// # Approach
    /// 1. Compute the min and max of nums in one pass.
    /// 2. Insert all elements of nums into a HashSet.
    /// 3. Collect every integer in [min, max] that is absent from the set.
    ///
    /// # Complexity
    /// - Time: O(n + R) where R = max - min (bounded by 100 per constraints)
    /// - Space: O(n) for the HashSet
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let set: HashSet<i32> = nums.into_iter().collect();
        (min..=max).filter(|x| !set.contains(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::find_missing_elements(vec![1, 4, 2, 5]), vec![3]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_missing_elements(vec![7, 8, 6, 9]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::find_missing_elements(vec![5, 1]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_no_missing_two_elements() {
        assert_eq!(
            Solution::find_missing_elements(vec![3, 4]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_all_missing_between_two() {
        assert_eq!(
            Solution::find_missing_elements(vec![1, 100]),
            (2..100).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_unsorted_input() {
        assert_eq!(
            Solution::find_missing_elements(vec![10, 7, 9]),
            vec![8]
        );
    }
}
