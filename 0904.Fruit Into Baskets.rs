use std::collections::HashMap;

impl Solution {
    /// Finds the longest subarray with at most two distinct fruit types.
    ///
    /// # Intuition
    /// This is the classic sliding window problem for at most k distinct
    /// elements (k=2).
    ///
    /// # Approach
    /// Expand the window by adding right elements. When more than 2 types
    /// exist, shrink from the left until the constraint is restored. Track
    /// the maximum window size.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — at most 3 entries in the map
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut freq: HashMap<i32, usize> = HashMap::new();
        let mut max_len = 0;
        let mut left = 0;

        for (right, &fruit) in fruits.iter().enumerate() {
            *freq.entry(fruit).or_default() += 1;
            while freq.len() > 2 {
                let lf = fruits[left];
                let count = freq.get_mut(&lf).unwrap();
                *count -= 1;
                if *count == 0 {
                    freq.remove(&lf);
                }
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }

    #[test]
    fn test_three_types() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn test_longer() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }

    #[test]
    fn test_single_type() {
        assert_eq!(Solution::total_fruit(vec![3, 3, 3, 3]), 4);
    }
}
