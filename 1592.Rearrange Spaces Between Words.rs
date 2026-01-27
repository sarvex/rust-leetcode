impl Solution {
    /// Evenly distribute spaces between words with remainder at end.
    ///
    /// # Intuition
    /// Count total spaces, divide evenly among gaps between words, and append
    /// any remainder spaces at the end.
    ///
    /// # Approach
    /// 1. Count spaces and extract words
    /// 2. If single word, append all spaces after it
    /// 3. Otherwise, join words with `spaces / (words - 1)` gaps
    /// 4. Append `spaces % (words - 1)` trailing spaces
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn reorder_spaces(text: String) -> String {
        let spaces = text.chars().filter(|&c| c == ' ').count();
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.len() == 1 {
            return format!("{}{}", words[0], " ".repeat(spaces));
        }

        let gap = spaces / (words.len() - 1);
        let remainder = spaces % (words.len() - 1);
        let mut result = words.join(&" ".repeat(gap));
        result.push_str(&" ".repeat(remainder));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_distribution() {
        assert_eq!(
            Solution::reorder_spaces("  this   is  a sentence ".to_string()),
            "this   is   a   sentence"
        );
    }

    #[test]
    fn single_word() {
        assert_eq!(Solution::reorder_spaces("  hello".to_string()), "hello  ");
    }

    #[test]
    fn with_remainder() {
        assert_eq!(
            Solution::reorder_spaces(" practice   makes   perfect".to_string()),
            "practice   makes   perfect "
        );
    }
}
