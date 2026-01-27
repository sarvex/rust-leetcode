impl Solution {
    /// Sort and verify constant difference for arithmetic progression.
    ///
    /// # Intuition
    /// An arithmetic progression has a constant difference between consecutive
    /// terms. Sorting the array and checking all consecutive differences suffices.
    ///
    /// # Approach
    /// 1. Sort the array
    /// 2. Check that all consecutive differences are equal using `windows(3)`
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(1) in-place sort
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        arr.windows(3).all(|w| w[0] - w[1] == w[1] - w[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_progression() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
    }

    #[test]
    fn invalid_progression() {
        assert!(!Solution::can_make_arithmetic_progression(vec![1, 2, 4]));
    }

    #[test]
    fn two_elements() {
        assert!(Solution::can_make_arithmetic_progression(vec![7, 3]));
    }
}
