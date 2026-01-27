use std::collections::HashMap;

impl Solution {
    /// Sort by frequency ascending, then by value descending.
    ///
    /// # Intuition
    /// Count frequencies, then sort with a composite key: primary sort by
    /// frequency (ascending), secondary sort by value (descending) for ties.
    ///
    /// # Approach
    /// 1. Count element frequencies
    /// 2. Sort by `(frequency, -value)` using custom comparator
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n) for the frequency map
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq = HashMap::new();
        for &x in &nums {
            *freq.entry(x).or_insert(0) += 1;
        }
        nums.sort_unstable_by(|a, b| freq[a].cmp(&freq[b]).then(b.cmp(a)));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sort() {
        assert_eq!(
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
    }

    #[test]
    fn tiebreak_by_value() {
        assert_eq!(
            Solution::frequency_sort(vec![2, 3, 1, 3, 2]),
            vec![1, 3, 3, 2, 2]
        );
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
