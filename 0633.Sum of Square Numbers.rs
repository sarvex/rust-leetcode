impl Solution {
    /// Checks if c can be expressed as a² + b² using two pointers.
    ///
    /// # Intuition
    /// Start with a = 0 and b = √c. Narrow the range based on whether
    /// a² + b² is less than, equal to, or greater than c.
    ///
    /// # Approach
    /// 1. Set a = 0, b = floor(√c).
    /// 2. If a² + b² == c, return true.
    /// 3. If less, increment a; if greater, decrement b.
    ///
    /// # Complexity
    /// - Time: O(√c)
    /// - Space: O(1)
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a: i64 = 0;
        let mut b: i64 = (c as f64).sqrt() as i64;
        let target = c as i64;
        while a <= b {
            let sum = a * a + b * b;
            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => a += 1,
                std::cmp::Ordering::Greater => b -= 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five() {
        assert!(Solution::judge_square_sum(5));
    }

    #[test]
    fn test_three() {
        assert!(!Solution::judge_square_sum(3));
    }

    #[test]
    fn test_zero() {
        assert!(Solution::judge_square_sum(0));
    }

    #[test]
    fn test_one() {
        assert!(Solution::judge_square_sum(1));
    }
}
