impl Solution {
    /// Group digits with dashes in groups of 3, ending with 2-2 if needed.
    ///
    /// # Intuition
    /// Strip non-digit characters, then group digits into blocks of 3.
    /// If the remainder is 1, the last group of 4 splits into 2-2 instead
    /// of 3-1 for valid formatting.
    ///
    /// # Approach
    /// 1. Filter to digits only
    /// 2. Track position and insert dashes at group boundaries
    /// 3. Handle the last-4-digit split for remainder 1 case
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn reformat_number(number: String) -> String {
        let digits: Vec<char> = number.chars().filter(|c| c.is_ascii_digit()).collect();
        let n = digits.len();

        digits
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let needs_dash = ((i + 1) % 3 == 0 && i < n - 2) || (n % 3 == 1 && i == n - 3);
                if needs_dash {
                    format!("{}-", *c)
                } else {
                    (*c).to_string()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_format() {
        assert_eq!(
            Solution::reformat_number("1-23-45 6".to_string()),
            "123-456"
        );
    }

    #[test]
    fn remainder_one() {
        assert_eq!(
            Solution::reformat_number("123 4-567".to_string()),
            "123-45-67"
        );
    }

    #[test]
    fn short_number() {
        assert_eq!(Solution::reformat_number("12".to_string()), "12");
    }
}
