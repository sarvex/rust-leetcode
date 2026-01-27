impl Solution {
    /// Determines the minimum cuts to divide a circle into n equal slices.
    ///
    /// # Intuition
    /// For n=1, no cuts needed. For even n, each cut creates two slices (diameter),
    /// so n/2 cuts suffice. For odd n>1, each cut only creates one new slice.
    ///
    /// # Approach
    /// - If n == 1: return 0
    /// - If n is even: return n / 2 (diameter cuts)
    /// - If n is odd: return n (chord cuts through center impossible)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn number_of_cuts(n: i32) -> i32 {
        match n {
            1 => 0,
            n if n % 2 == 0 => n >> 1,
            _ => n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_slice() {
        assert_eq!(Solution::number_of_cuts(1), 0);
    }

    #[test]
    fn test_even_slices() {
        assert_eq!(Solution::number_of_cuts(4), 2);
        assert_eq!(Solution::number_of_cuts(8), 4);
    }

    #[test]
    fn test_odd_slices() {
        assert_eq!(Solution::number_of_cuts(3), 3);
        assert_eq!(Solution::number_of_cuts(5), 5);
    }
}
