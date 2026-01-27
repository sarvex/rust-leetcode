impl Solution {
    /// Right-to-left scan with subtraction rule for Roman to integer conversion.
    ///
    /// # Intuition
    /// Roman numerals are additive except when a smaller value precedes a larger
    /// one (subtractive notation). Scanning right-to-left, if the current value
    /// is less than the previous value it should be subtracted, otherwise added.
    ///
    /// # Approach
    /// Map each character to its integer value. Fold from right to left,
    /// tracking the previous value. Subtract when the current value is smaller
    /// than the previous; otherwise add.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the string
    /// - Space: O(1) — scalar accumulators only
    pub fn roman_to_int(s: String) -> i32 {
        let value_of = |ch: u8| -> i32 {
            match ch {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            }
        };

        s.bytes()
            .rev()
            .fold((0, 0), |(total, prev), ch| {
                let current = value_of(ch);
                if current < prev {
                    (total - current, current)
                } else {
                    (total + current, current)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_additive() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn subtractive_notation() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn mixed() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn complex_subtractive() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
