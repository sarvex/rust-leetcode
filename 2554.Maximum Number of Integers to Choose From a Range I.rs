use std::collections::HashSet;

impl Solution {
    /// Greedily choose smallest non-banned integers under max_sum.
    ///
    /// # Intuition
    /// Choosing the smallest available values first maximizes the count of integers
    /// we can pick without exceeding `max_sum`.
    ///
    /// # Approach
    /// 1. Collect banned values into a HashSet for O(1) lookup
    /// 2. Iterate from 1 to n, skipping banned values
    /// 3. Accumulate the sum, stopping when it would exceed max_sum
    ///
    /// # Complexity
    /// - Time: O(n + b) where b is the length of banned
    /// - Space: O(b)
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned_set: HashSet<i32> = banned.into_iter().collect();
        let mut sum = 0;
        let mut count = 0;

        for i in 1..=n {
            if sum + i > max_sum {
                break;
            }
            if !banned_set.contains(&i) {
                sum += i;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
    }

    #[test]
    fn test_all_banned() {
        assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5], 5, 10), 0);
    }

    #[test]
    fn test_none_banned() {
        assert_eq!(Solution::max_count(vec![], 3, 10), 3);
    }

    #[test]
    fn test_tight_sum() {
        assert_eq!(Solution::max_count(vec![], 5, 1), 1);
    }
}
