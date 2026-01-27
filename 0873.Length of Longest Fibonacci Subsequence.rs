use std::collections::HashMap;

impl Solution {
    /// Finds the longest Fibonacci-like subsequence using DP with index lookup.
    ///
    /// # Intuition
    /// For each pair `(arr[j], arr[i])`, the previous element in the Fibonacci
    /// sequence would be `arr[i] - arr[j]`. Use a hash map to find its index.
    ///
    /// # Approach
    /// Build a value-to-index map. Define `dp[i][j]` as the length of the
    /// longest Fibonacci subsequence ending with `(arr[j], arr[i])`. For each
    /// pair, look up `arr[i] - arr[j]` in the map. If found at index `k < j`,
    /// extend that subsequence.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n^2) for the DP table
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let index: HashMap<i32, usize> = arr.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut dp = vec![vec![2; n]; n];
        let mut max_len = 0;

        for i in 2..n {
            for j in 1..i {
                let target = arr[i] - arr[j];
                if let Some(&k) = index.get(&target) {
                    if k < j {
                        dp[i][j] = dp[i][j].max(dp[j][k] + 1);
                        max_len = max_len.max(dp[i][j]);
                    }
                }
            }
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            5
        );
    }

    #[test]
    fn test_longer_subsequence() {
        assert_eq!(
            Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]),
            3
        );
    }

    #[test]
    fn test_no_fibonacci() {
        assert_eq!(Solution::len_longest_fib_subseq(vec![1, 2, 4, 8, 16]), 0);
    }
}
