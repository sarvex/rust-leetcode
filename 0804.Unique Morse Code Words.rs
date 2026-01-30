use std::collections::HashSet;


impl Solution {
    /// Counts unique Morse code representations of words.
    ///
    /// # Intuition
    /// Map each word to its Morse encoding and collect into a set to
    /// count distinct transformations.
    ///
    /// # Approach
    /// Use a constant Morse table. Transform each word's bytes to their
    /// Morse equivalents, concatenate, and insert into a `HashSet`.
    ///
    /// # Complexity
    /// - Time: O(n * L) where n is word count and L is max word length
    /// - Space: O(n * L) for the set
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        const CODES: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut morse_codes = HashSet::with_capacity(words.len());
        for word in &words {
            let morse: String = word.bytes().map(|b| CODES[(b - b'a') as usize]).collect();
            morse_codes.insert(morse);
        }
        morse_codes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let words: Vec<String> = vec!["gin", "zen", "gig", "msg"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::unique_morse_representations(words), 2);
    }

    #[test]
    fn test_single_word() {
        assert_eq!(
            Solution::unique_morse_representations(vec!["a".to_string()]),
            1
        );
    }
}
