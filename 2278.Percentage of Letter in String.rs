impl Solution {
    /// Computes the percentage of a given letter in the string (floored).
    ///
    /// # Intuition
    /// Count matching characters and compute integer percentage via integer division.
    ///
    /// # Approach
    /// Use `as_bytes()` for efficient byte comparison, filter matching bytes, and
    /// calculate `100 * count / len` using integer arithmetic.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let target = letter as u8;
        let count = s.as_bytes().iter().filter(|&&b| b == target).count();
        (100 * count / s.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }

    #[test]
    fn test_all_matching() {
        assert_eq!(Solution::percentage_letter("aaa".to_string(), 'a'), 100);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::percentage_letter("z".to_string(), 'z'), 100);
    }
}
