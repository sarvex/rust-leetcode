use std::collections::HashMap;

impl Solution {
    /// Finds the smallest pair of distinct values with different frequencies.
    ///
    /// # Intuition
    /// Count frequencies, sort unique values, then scan pairs in order.
    /// The first valid pair found is automatically the lexicographically smallest.
    ///
    /// # Approach
    /// 1. Build a frequency map of all values.
    /// 2. Collect and sort unique values.
    /// 3. Iterate pairs (i, j) with i < j in sorted order.
    ///    Return the first pair whose frequencies differ.
    ///
    /// # Complexity
    /// - Time: O(n + u^2) where u is the number of unique values (u <= 100).
    /// - Space: O(u) for the frequency map and sorted unique values.
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = HashMap::with_capacity(nums.len());
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut unique: Vec<i32> = freq.keys().copied().collect();
        unique.sort_unstable();

        for i in 0..unique.len() {
            for j in (i + 1)..unique.len() {
                if freq[&unique[i]] != freq[&unique[j]] {
                    return vec![unique[i], unique[j]];
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![1, 1, 2, 2, 3, 4]),
            vec![1, 3]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![1, 5]),
            vec![-1, -1]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![7]),
            vec![-1, -1]
        );
    }

    #[test]
    fn all_same_frequency() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![1, 2, 3, 4]),
            vec![-1, -1]
        );
    }

    #[test]
    fn first_pair_valid() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![1, 1, 2]),
            vec![1, 2]
        );
    }

    #[test]
    fn all_same_value() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![5, 5, 5]),
            vec![-1, -1]
        );
    }
}
