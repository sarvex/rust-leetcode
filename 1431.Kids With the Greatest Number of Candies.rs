impl Solution {
    /// Compare each child's total against the current maximum.
    ///
    /// # Intuition
    /// A child can have the greatest number if their candies plus extra
    /// candies meets or exceeds the current maximum across all children.
    ///
    /// # Approach
    /// 1. Find the maximum candy count
    /// 2. Map each child to whether `candies[i] + extra >= max`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap_or(&0);
        candies.iter().map(|&c| c + extra_candies >= max).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_result() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn all_false_except_max() {
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn ties() {
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
