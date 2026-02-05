impl Solution {
    /// Computes the digital root using the mathematical formula.
    ///
    /// # Intuition
    /// The digital root of a positive number follows a pattern based on modulo 9.
    /// For any number > 0, the digital root is `1 + (num - 1) % 9`.
    ///
    /// # Approach
    /// Apply the digital root formula directly.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 { 0 } else { 1 + (num - 1) % 9 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_digit() {
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::add_digits(0), 0);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::add_digits(5), 5);
    }
}
