impl Solution {
    /// Minimum increasing subsequences to remove to empty the array.
    ///
    /// # Intuition
    /// This is equivalent to finding the longest non-increasing subsequence
    /// (patience sorting variant). We maintain a list of "tails" where each
    /// entry represents the end of an active decreasing subsequence. For each
    /// element, we binary-search for the leftmost tail that is strictly less
    /// than the current value; if none exists, we start a new subsequence.
    ///
    /// # Approach
    /// 1. Maintain a sorted list of subsequence tails in decreasing order.
    /// 2. For each element, binary search for the position to replace.
    /// 3. If no valid position, extend the list.
    /// 4. Return the list length.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n)
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut tails = Vec::new();

        for &x in &nums {
            let pos = tails.partition_point(|&t| t >= x);
            match pos == tails.len() {
                true => tails.push(x),
                false => tails[pos] = x,
            }
        }

        tails.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strictly_increasing() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 4]), 1);
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(Solution::min_operations(vec![4, 3, 2, 1]), 4);
    }

    #[test]
    fn mixed_sequence() {
        assert_eq!(Solution::min_operations(vec![5, 2, 4, 1]), 3);
    }
}
