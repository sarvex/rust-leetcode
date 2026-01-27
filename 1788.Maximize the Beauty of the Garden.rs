use std::collections::HashMap;

impl Solution {
    /// Maximizes garden beauty using prefix sums and first-occurrence tracking.
    ///
    /// # Intuition
    /// For each flower value that appears at least twice, the beauty is the sum
    /// of positive values between the first and last occurrence plus twice the
    /// flower value. A prefix sum of positive values enables O(1) range queries.
    ///
    /// # Approach
    /// 1. Build a prefix sum array counting only positive flower values.
    /// 2. Track the first occurrence index of each flower value in a hash map.
    /// 3. On subsequent occurrences, compute beauty as prefix range sum plus
    ///    twice the flower value and update the maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn maximum_beauty(flowers: Vec<i32>) -> i32 {
        let mut prefix = vec![0; flowers.len() + 1];
        let mut first_seen = HashMap::new();
        let mut result = i32::MIN;

        for (i, &val) in flowers.iter().enumerate() {
            if let Some(&j) = first_seen.get(&val) {
                result = result.max(prefix[i] - prefix[j + 1] + val * 2);
            } else {
                first_seen.insert(val, i);
            }
            prefix[i + 1] = prefix[i] + val.max(0);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_positive_negative() {
        assert_eq!(Solution::maximum_beauty(vec![1, 2, 3, 1, 2]), 8);
    }

    #[test]
    fn test_all_same_negative() {
        assert_eq!(Solution::maximum_beauty(vec![-1, -2, 0, -1]), -2);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::maximum_beauty(vec![100, -1, -2, 100]), 200);
    }
}
