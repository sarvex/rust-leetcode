impl Solution {
    /// GCD of odd and even sums via closed-form formula.
    ///
    /// # Intuition
    /// The sum of the first n odd numbers is n², and the sum of the first n
    /// even numbers is n(n+1). Their GCD simplifies algebraically to n itself,
    /// since GCD(n², n(n+1)) = n · GCD(n, n+1) = n · 1 = n.
    ///
    /// # Approach
    /// Return n directly — no iteration required.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // sumOdd = 1+3+5+7 = 16, sumEven = 2+4+6+8 = 20, GCD = 4
        assert_eq!(Solution::gcd_of_odd_even_sums(4), 4);
    }

    #[test]
    fn test_example_2() {
        // sumOdd = 1+3+5+7+9 = 25, sumEven = 2+4+6+8+10 = 30, GCD = 5
        assert_eq!(Solution::gcd_of_odd_even_sums(5), 5);
    }

    #[test]
    fn test_edge_single() {
        // sumOdd = 1, sumEven = 2, GCD = 1
        assert_eq!(Solution::gcd_of_odd_even_sums(1), 1);
    }

    #[test]
    fn test_edge_two() {
        // sumOdd = 1+3 = 4, sumEven = 2+4 = 6, GCD = 2
        assert_eq!(Solution::gcd_of_odd_even_sums(2), 2);
    }

    #[test]
    fn test_edge_three() {
        // sumOdd = 1+3+5 = 9, sumEven = 2+4+6 = 12, GCD = 3
        assert_eq!(Solution::gcd_of_odd_even_sums(3), 3);
    }

    #[test]
    fn test_boundary_max() {
        assert_eq!(Solution::gcd_of_odd_even_sums(1000), 1000);
    }
}
