impl Solution {
    /// Computes the most expensive item that cannot be bought using two coprime denominations.
    ///
    /// # Intuition
    /// By the Chicken McNugget theorem (Frobenius), for two coprime integers `a` and `b`,
    /// the largest number not representable as `a*x + b*y` (x,y ≥ 0) is `a*b - a - b`.
    ///
    /// # Approach
    /// 1. Apply the Frobenius formula directly: `p1 * p2 - p1 - p2`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn most_expensive_item(prime_one: i32, prime_two: i32) -> i32 {
        prime_one * prime_two - prime_one - prime_two
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_primes() {
        assert_eq!(Solution::most_expensive_item(2, 5), 3);
    }

    #[test]
    fn test_consecutive_primes() {
        assert_eq!(Solution::most_expensive_item(5, 7), 23);
    }
}
