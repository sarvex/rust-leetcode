impl Solution {
    /// Checks if all numbers in a sentence are strictly ascending.
    ///
    /// # Intuition
    /// Extract numeric tokens and verify each is greater than the previous.
    ///
    /// # Approach
    /// 1. Split by spaces and parse numeric tokens.
    /// 2. Compare each number with the previous one.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = -1;
        for token in s.split(' ') {
            if token.as_bytes()[0].is_ascii_digit() {
                let num = token.parse::<i32>().unwrap();
                if num <= prev {
                    return false;
                }
                prev = num;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending() {
        assert!(Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
        ));
    }

    #[test]
    fn test_not_ascending() {
        assert!(!Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ));
    }
}
