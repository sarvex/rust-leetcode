impl Solution {
    /// Determines if a chessboard square is white based on coordinates.
    ///
    /// # Intuition
    /// A square is white when the sum of its column letter and row digit
    /// is odd (using ASCII values directly).
    ///
    /// # Approach
    /// 1. Sum the two ASCII byte values.
    /// 2. Check if the sum is odd.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn square_is_white(coordinates: String) -> bool {
        let b = coordinates.as_bytes();
        (b[0] + b[1]) & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_square() {
        assert!(Solution::square_is_white("a1".to_string()));
    }

    #[test]
    fn test_black_square() {
        assert!(!Solution::square_is_white("a2".to_string()));
    }

    #[test]
    fn test_h8() {
        assert!(!Solution::square_is_white("h8".to_string()));
    }
}
