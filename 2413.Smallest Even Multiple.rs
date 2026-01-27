impl Solution {
    /// Returns the smallest positive integer that is a multiple of both 2 and n.
    ///
    /// # Intuition
    /// The LCM of 2 and n is n itself when n is even, and 2n when n is odd.
    /// A bitwise trick: if the lowest bit is set, n is odd so we double it.
    ///
    /// # Approach
    /// 1. Check if n is even using bitwise AND with 1
    /// 2. Return n if even, otherwise return n * 2
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n % 2 {
            0 => n,
            _ => n * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_input() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }

    #[test]
    fn test_odd_input() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::smallest_even_multiple(1), 2);
    }

    #[test]
    fn test_two() {
        assert_eq!(Solution::smallest_even_multiple(2), 2);
    }
}
