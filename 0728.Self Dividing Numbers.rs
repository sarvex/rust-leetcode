impl Solution {
    /// Collects self-dividing numbers in a range using digit checking.
    ///
    /// # Intuition
    /// A self-dividing number is divisible by each of its digits and contains
    /// no zero digit.
    ///
    /// # Approach
    /// Filter the range, checking each number by extracting digits and verifying
    /// divisibility.
    ///
    /// # Complexity
    /// - Time: O((right - left) * d) where d is max digit count
    /// - Space: O(1) auxiliary
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let is_self_dividing = |x: &i32| -> bool {
            let mut y = *x;
            while y > 0 {
                let d = y % 10;
                if d == 0 || x % d != 0 {
                    return false;
                }
                y /= 10;
            }
            true
        };
        (left..=right).filter(is_self_dividing).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_1_to_22() {
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
    }

    #[test]
    fn test_single_value() {
        assert_eq!(
            Solution::self_dividing_numbers(47, 85),
            vec![48, 55, 66, 77]
        );
    }
}
