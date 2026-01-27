impl Solution {
    /// Returns the sum of two integers.
    ///
    /// # Intuition
    /// Direct arithmetic addition satisfies the requirement.
    ///
    /// # Approach
    /// Return the sum of `num1` and `num2`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        assert_eq!(Solution::sum(12, 5), 17);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Solution::sum(-10, 4), -6);
    }

    #[test]
    fn test_zeros() {
        assert_eq!(Solution::sum(0, 0), 0);
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(Solution::sum(-7, -3), -10);
    }
}
