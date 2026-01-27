impl Solution {
    /// Finds the maximum subarray sum allowing at most one element deletion.
    ///
    /// # Intuition
    /// Compute max subarray ending at each position (left) and starting at
    /// each position (right). Deleting element `i` combines `left[i-1]` and
    /// `right[i+1]`.
    ///
    /// # Approach
    /// Two Kadane passes: left-to-right for `max_ending_at[i]` and right-to-left
    /// for `max_starting_at[i]`. The answer is the max of all `max_ending_at[i]`
    /// (no deletion) and `max_ending_at[i-1] + max_starting_at[i+1]` (one deletion).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the two arrays
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 1 {
            return arr[0];
        }

        let mut max_ending = vec![0; n];
        let mut max_starting = vec![0; n];

        max_ending[0] = arr[0];
        for i in 1..n {
            max_ending[i] = arr[i] + max_ending[i - 1].max(0);
        }

        max_starting[n - 1] = arr[n - 1];
        for i in (0..n - 1).rev() {
            max_starting[i] = arr[i] + max_starting[i + 1].max(0);
        }

        let mut result = *max_ending.iter().max().unwrap();
        for i in 1..n - 1 {
            result = result.max(max_ending[i - 1] + max_starting[i + 1]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
    }

    #[test]
    fn test_delete_helps() {
        assert_eq!(Solution::maximum_sum(vec![1, -4, -5, -2, 5, 0, -1, 2]), 7);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
    }
}
