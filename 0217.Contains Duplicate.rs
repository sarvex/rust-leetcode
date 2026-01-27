use std::collections::HashSet;

impl Solution {
    /// Checks for duplicate elements using a HashSet.
    ///
    /// # Intuition
    /// If the number of unique elements is less than the total count,
    /// duplicates exist.
    ///
    /// # Approach
    /// Collect elements into a HashSet and compare its size with the input length.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<i32> = nums.iter().copied().collect();
        set.len() != nums.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_duplicates() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn no_duplicates() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn all_same() {
        assert!(Solution::contains_duplicate(vec![1, 1, 1, 1]));
    }
}
