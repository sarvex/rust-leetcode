impl Solution {
    /// Determines if a number is a perfect square using binary search.
    ///
    /// # Intuition
    /// Binary search for an integer whose square equals num.
    ///
    /// # Approach
    /// 1. Binary search on [1, num] using i64 to avoid overflow.
    /// 2. If mid * mid >= num, narrow right.
    /// 3. Check if left * left == num.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let (mut left, mut right) = (1i64, num);
        while left < right {
            let mid = (left + right) / 2;
            if mid * mid >= num {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left * left == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_square() {
        assert!(Solution::is_perfect_square(16));
    }

    #[test]
    fn not_perfect_square() {
        assert!(!Solution::is_perfect_square(14));
    }

    #[test]
    fn one_is_perfect() {
        assert!(Solution::is_perfect_square(1));
    }
}
