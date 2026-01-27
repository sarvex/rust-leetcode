use std::collections::HashSet;

impl Solution {
    /// Determines if a number is happy using digit-square-sum cycle detection.
    ///
    /// # Intuition
    /// Repeatedly sum the squares of digits. If the result reaches 1, the number
    /// is happy. If a cycle is detected (repeated sum), it is not.
    ///
    /// # Approach
    /// 1. Compute the sum of squared digits.
    /// 2. Track seen sums in a HashSet.
    /// 3. Stop when reaching 1 (happy) or detecting a cycle (not happy).
    ///
    /// # Complexity
    /// - Time: O(log n) per step, bounded cycle length
    /// - Space: O(log n) for the seen set
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();
        while n != 1 {
            n = Self::digit_square_sum(n);
            if !seen.insert(n) {
                return false;
            }
        }
        true
    }

    fn digit_square_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n != 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_number() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn not_happy() {
        assert!(!Solution::is_happy(2));
    }

    #[test]
    fn one_is_happy() {
        assert!(Solution::is_happy(1));
    }
}
