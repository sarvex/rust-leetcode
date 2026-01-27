impl Solution {
    /// Returns the array concatenated with itself.
    ///
    /// # Intuition
    /// The repeat method efficiently duplicates the vector contents.
    ///
    /// # Approach
    /// 1. Use `Vec::repeat(2)` to double the array.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::get_concatenation(vec![1]), vec![1, 1]);
    }
}
