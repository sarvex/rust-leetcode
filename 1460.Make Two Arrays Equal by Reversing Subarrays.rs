impl Solution {
    /// Sort-and-compare to check if arrays are permutations.
    ///
    /// # Intuition
    /// Any permutation can be achieved through subarray reversals. Two arrays
    /// can be made equal if and only if they contain the same elements with
    /// the same frequencies — i.e., they are permutations of each other.
    ///
    /// # Approach
    /// 1. Sort both arrays
    /// 2. Compare for equality
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_arrays() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    }

    #[test]
    fn not_equal() {
        assert!(!Solution::can_be_equal(vec![7], vec![7, 7]));
    }

    #[test]
    fn already_equal() {
        assert!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 9]));
    }
}
