impl Solution {
    /// Sum all integers from 1 to n divisible by 3, 5, or 7.
    ///
    /// # Intuition
    /// Filter and sum using a simple divisibility check on each integer.
    ///
    /// # Approach
    /// Iterate 1..=n, filter values divisible by 3, 5, or 7, and sum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|&x| x % 3 == 0 || x % 5 == 0 || x % 7 == 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
    }

    #[test]
    fn test_ten() {
        assert_eq!(Solution::sum_of_multiples(10), 40);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::sum_of_multiples(1), 0);
    }
}
