impl Solution {
    /// Computes the nth Fibonacci number iteratively.
    ///
    /// # Intuition
    /// Keep two variables for the previous pair and iterate forward.
    ///
    /// # Approach
    /// 1. Initialize a = 0, b = 1.
    /// 2. Iterate n times, advancing (a, b) = (b, a + b).
    /// 3. Return a.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(Solution::fib(0), 0);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::fib(1), 1);
    }

    #[test]
    fn test_four() {
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn test_ten() {
        assert_eq!(Solution::fib(10), 55);
    }
}
