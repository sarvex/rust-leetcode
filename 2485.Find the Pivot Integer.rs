impl Solution {
    /// Finds the pivot integer x where sum(1..=x) equals sum(x..=n).
    ///
    /// # Intuition
    /// Sum 1..=x = x(x+1)/2. Sum x..=n = n(n+1)/2 - x(x-1)/2.
    /// Setting equal: x² = n(n+1)/2. So x exists iff n(n+1)/2 is a perfect square.
    ///
    /// # Approach
    /// Compute total = n*(n+1)/2, take its integer square root, and verify.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn pivot_integer(n: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let x = Self::isqrt(total as u32) as i32;
        if x * x == total {
            x
        } else {
            -1
        }
    }

    #[inline]
    fn isqrt(n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let mut x = n as u64;
        let n = n as u64;
        let mut y = (x + n / x) / 2;
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_exists() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::pivot_integer(1), 1);
    }

    #[test]
    fn test_no_pivot() {
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
