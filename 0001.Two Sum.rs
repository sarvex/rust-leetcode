use std::collections::HashMap;

impl Solution {
    /// Hash map complement lookup for two-sum pair finding.
    ///
    /// # Intuition
    /// For each element, the complement needed to reach the target is uniquely
    /// determined. A hash map provides O(1) lookup to check whether a previously
    /// seen element equals the current complement.
    ///
    /// # Approach
    /// Iterate through the array once, maintaining a map from value to index.
    /// For each element compute `target - num` and probe the map. If found,
    /// return both indices immediately; otherwise insert the current element.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass with constant-time hash operations
    /// - Space: O(n) — hash map stores at most n entries
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&j) = num_to_index.get(&complement) {
                return vec![j as i32, i as i32];
            }

            num_to_index.insert(num, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_at_start() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn pair_in_middle() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }

    #[test]
    fn no_solution() {
        assert_eq!(Solution::two_sum(vec![1, 2, 3], 10), vec![]);
    }
}
