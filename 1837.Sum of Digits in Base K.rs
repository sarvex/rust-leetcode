impl Solution {
    /// Computes the digit sum of n in base k.
    ///
    /// # Intuition
    /// Repeatedly extract the last digit via modulo and divide by the base
    /// until the number is zero.
    ///
    /// # Approach
    /// 1. While n > 0, add n % k to the sum.
    /// 2. Divide n by k each iteration.
    ///
    /// # Complexity
    /// - Time: O(log_k(n))
    /// - Space: O(1)
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % k;
            n /= k;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_10() {
        assert_eq!(Solution::sum_base(34, 6), 9);
    }

    #[test]
    fn test_base_2() {
        assert_eq!(Solution::sum_base(10, 10), 1);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::sum_base(5, 10), 5);
    }
}
