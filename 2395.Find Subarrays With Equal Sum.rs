impl Solution {
    /// Checks if two distinct subarrays of length 2 have the same sum.
    ///
    /// # Intuition
    /// With only length-2 subarrays, track seen sums in a HashSet. A duplicate
    /// sum means we found two such subarrays.
    ///
    /// # Approach
    /// Iterate over consecutive pairs, inserting sums into a set. If insertion
    /// fails (duplicate), return true immediately.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::with_capacity(nums.len());
        nums.windows(2).any(|w| !seen.insert(w[0] + w[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::find_subarrays(vec![4, 2, 4]));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::find_subarrays(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_all_same() {
        assert!(Solution::find_subarrays(vec![0, 0, 0]));
    }

    #[test]
    fn test_two_elements() {
        assert!(!Solution::find_subarrays(vec![1, 2]));
    }
}
