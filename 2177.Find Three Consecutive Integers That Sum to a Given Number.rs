impl Solution {
    /// Find three consecutive integers summing to the given number.
    ///
    /// # Intuition
    /// Three consecutive integers x-1, x, x+1 sum to 3x, so the number must
    /// be divisible by 3 with x = num / 3.
    ///
    /// # Approach
    /// Check divisibility by 3; if valid, return [x-1, x, x+1].
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        match num % 3 {
            0 => {
                let x = num / 3;
                vec![x - 1, x, x + 1]
            }
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisible_by_three() {
        assert_eq!(Solution::sum_of_three(33), vec![10, 11, 12]);
    }

    #[test]
    fn not_divisible_by_three() {
        assert!(Solution::sum_of_three(4).is_empty());
    }

    #[test]
    fn zero_input() {
        assert_eq!(Solution::sum_of_three(0), vec![-1, 0, 1]);
    }
}
