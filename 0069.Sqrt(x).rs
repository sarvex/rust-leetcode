impl Solution {
    /// Binary search for integer square root.
    ///
    /// # Intuition
    /// The integer square root is the largest integer `r` such that `r² <= x`.
    /// Binary search on the range `[0, x]` efficiently finds this boundary
    /// using division to avoid overflow.
    ///
    /// # Approach
    /// Maintain `[left, right]` bounds. Compute `mid = (left + right + 1) / 2`
    /// (biased high to avoid infinite loops). If `mid > x / mid`, the answer
    /// is below `mid`; otherwise it is at or above `mid`. Converge until
    /// `left == right`.
    ///
    /// # Complexity
    /// - Time: O(log x) — standard binary search
    /// - Space: O(1) — scalar variables only
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut left, mut right) = (0, x);

        while left < right {
            let mid = left + (right - left + 1) / 2;
            if mid > x / mid {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_square() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn non_perfect_square() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::my_sqrt(0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(Solution::my_sqrt(1), 1);
    }

    #[test]
    fn large_value() {
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
