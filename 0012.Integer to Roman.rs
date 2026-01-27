impl Solution {
    /// Greedy subtraction with value-symbol table for integer to Roman conversion.
    ///
    /// # Intuition
    /// Roman numerals greedily use the largest possible symbols. A descending
    /// table of values (including subtractive forms like CM, CD) lets us
    /// repeatedly subtract the largest fitting value and append its symbol.
    ///
    /// # Approach
    /// Define parallel arrays of values and symbols in descending order.
    /// Iterate through them, appending each symbol as many times as the
    /// value fits into the remaining number.
    ///
    /// # Complexity
    /// - Time: O(1) — bounded by the fixed 13-entry table (max ~15 iterations for 3999)
    /// - Space: O(1) — output string bounded by the input range
    pub fn int_to_roman(mut num: i32) -> String {
        const SYMBOLS: [&str; 13] = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        const VALUES: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        let mut result = String::new();
        for (&value, &symbol) in VALUES.iter().zip(SYMBOLS.iter()) {
            while num >= value {
                num -= value;
                result.push_str(symbol);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three() {
        assert_eq!(Solution::int_to_roman(3), "III");
    }

    #[test]
    fn fifty_eight() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }

    #[test]
    fn nineteen_ninety_four() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn max_value() {
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    }
}
