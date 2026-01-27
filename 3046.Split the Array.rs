use std::collections::HashMap;

impl Solution {
    /// Determine whether the array can be split into two arrays with all distinct elements.
    ///
    /// # Intuition
    /// If any value appears three or more times, it is impossible to distribute those
    /// occurrences across only two arrays while keeping each array distinct.
    ///
    /// # Approach
    /// 1. Count the frequency of each element using a hash map.
    /// 2. Return true if the maximum frequency is at most 2.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut cnt = HashMap::new();
        nums.iter().for_each(|&x| *cnt.entry(x).or_insert(0) += 1);
        cnt.values().all(|&v| v < 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splittable_array() {
        assert!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
    }

    #[test]
    fn unsplittable_due_to_triple() {
        assert!(!Solution::is_possible_to_split(vec![1, 1, 1, 1]));
    }
}
