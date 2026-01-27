impl Solution {
    /// Iterative BFS-style expansion of phone digit letter combinations.
    ///
    /// # Intuition
    /// Each digit maps to a set of letters. Building combinations iteratively
    /// by appending each digit's letters to all existing partial results
    /// avoids recursion overhead.
    ///
    /// # Approach
    /// Start with a single empty string. For each digit, compute the mapped
    /// letters and produce new combinations by appending each letter to every
    /// existing result. Use `flat_map` for the cartesian product at each step.
    ///
    /// # Complexity
    /// - Time: O(4^n × n) — at most 4 letters per digit, n digits
    /// - Space: O(4^n × n) — storing all combinations
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        const MAPPING: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        digits
            .bytes()
            .fold(vec![String::new()], |combinations, digit| {
                let letters = MAPPING[(digit - b'2') as usize];
                combinations
                    .iter()
                    .flat_map(|prefix| letters.chars().map(move |ch| format!("{prefix}{ch}")))
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_digits() {
        let mut result = Solution::letter_combinations("23".to_string());
        result.sort();
        assert_eq!(
            result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(
            Solution::letter_combinations(String::new()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn single_digit() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn digit_with_four_letters() {
        assert_eq!(
            Solution::letter_combinations("7".to_string()),
            vec!["p", "q", "r", "s"]
        );
    }
}
