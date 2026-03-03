impl Solution {
    /// Triangular number formula for splitting cost.
    ///
    /// # Intuition
    /// When splitting x into a and b (where a + b = x) the cost is a * b,
    /// then we recursively split a and b into ones. If we define f(n) as the
    /// minimum total cost, then f(n) = min over valid (a,b) of
    /// a*b + f(a) + f(b). Hypothesizing f(n) = n*(n-1)/2 and expanding:
    /// a*(a-1)/2 + b*(b-1)/2 + a*b = ((a+b)² - (a+b)) / 2 = n*(n-1)/2.
    /// The cost is invariant regardless of the splitting strategy.
    ///
    /// # Approach
    /// 1. Recognize that every possible sequence of splits yields the same
    ///    total cost, equal to the (n-1)-th triangular number.
    /// 2. Return n * (n - 1) / 2 directly.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn min_cost(n: i32) -> i32 {
        n * (n - 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_cost(3), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_cost(4), 6);
    }

    #[test]
    fn test_base_case() {
        assert_eq!(Solution::min_cost(1), 0);
    }

    #[test]
    fn test_two() {
        assert_eq!(Solution::min_cost(2), 1);
    }

    #[test]
    fn test_upper_bound() {
        assert_eq!(Solution::min_cost(500), 500 * 499 / 2);
    }
}
