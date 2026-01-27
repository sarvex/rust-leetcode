use std::collections::HashMap;

impl Solution {
    /// Hash map DP tracking longest arithmetic subsequence ending at each value.
    ///
    /// # Intuition
    /// For each element, the longest arithmetic subsequence of the given
    /// difference ending at that element equals one plus the length ending at
    /// `element - difference`. A single-pass hash map lookup suffices.
    ///
    /// # Approach
    /// 1. Iterate through the array, maintaining a map from value to best length
    /// 2. For each element, look up `element - difference` and extend by one
    /// 3. Track the global maximum across all entries
    ///
    /// # Complexity
    /// - Time: O(n) single pass with O(1) hash map operations
    /// - Space: O(n) for the hash map
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp = HashMap::with_capacity(arr.len());
        arr.iter().fold(0, |best, &x| {
            let count = dp.get(&(x - difference)).unwrap_or(&0) + 1;
            dp.insert(x, count);
            best.max(count)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_difference() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    }

    #[test]
    fn negative_difference() {
        assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    }

    #[test]
    fn mixed_values() {
        assert_eq!(
            Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
            4
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::longest_subsequence(vec![42], 0), 1);
    }
}
