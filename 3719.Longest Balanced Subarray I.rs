use std::collections::HashSet;

impl Solution {
    /// Longest contiguous subarray where #distinct evens equals #distinct odds.
    ///
    /// # Intuition
    /// Balance means distinct evens count == distinct odds count. Enumerate subarrays
    /// by start; for each start extend the end and maintain two sets. When sizes match, update max length.
    ///
    /// # Approach
    /// For each starting index `i`, maintain `evens` and `odds` HashSets. For each ending index `j >= i`,
    /// add `nums[j]` to the appropriate set (even → evens, else odds). When `evens.len() == odds.len()`,
    /// candidate length is `j - i + 1`; take the maximum over all such pairs.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        (0..n)
            .map(|i| {
                let mut evens = HashSet::with_capacity(n);
                let mut odds = HashSet::with_capacity(n);
                let mut best = 0;
                for j in i..n {
                    if nums[j] % 2 == 0 {
                        evens.insert(nums[j]);
                    } else {
                        odds.insert(nums[j]);
                    }
                    if evens.len() == odds.len() {
                        best = best.max((j - i + 1) as i32);
                    }
                }
                best
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::longest_balanced(vec![1]), 0);
    }

    #[test]
    fn test_two_same_parity() {
        assert_eq!(Solution::longest_balanced(vec![2, 4]), 0);
    }

    #[test]
    fn test_two_balanced() {
        assert_eq!(Solution::longest_balanced(vec![1, 2]), 2);
    }
}
