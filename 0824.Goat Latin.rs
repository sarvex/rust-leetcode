pub struct Solution;

impl Solution {
    /// Converts a sentence to Goat Latin with vowel/consonant rules.
    ///
    /// # Intuition
    /// Each word is transformed based on whether it starts with a vowel or
    /// consonant, then suffixed with "ma" and an increasing number of "a"s.
    ///
    /// # Approach
    /// Split the sentence. For vowel-starting words, append "ma". For
    /// consonant-starting words, move the first letter to the end then
    /// append "ma". Add `i + 1` trailing "a" characters.
    ///
    /// # Complexity
    /// - Time: O(n * k) where n is word count and k is average word length
    /// - Space: O(n * k) for the output
    pub fn to_goat_latin(sentence: String) -> String {
        const VOWELS: &[u8] = b"aeiouAEIOU";
        let words: Vec<_> = sentence
            .split_whitespace()
            .enumerate()
            .map(|(i, word)| {
                let mut result = String::with_capacity(word.len() + 3 + i);
                if VOWELS.contains(&word.as_bytes()[0]) {
                    result.push_str(word);
                } else {
                    result.push_str(&word[1..]);
                    result.push(word.as_bytes()[0] as char);
                }
                result.push_str("ma");
                result.extend(std::iter::repeat('a').take(i + 1));
                result
            })
            .collect();
        words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa teleakspaa oatGmaaa atinLmaaaa"
        );
    }

    #[test]
    fn test_vowel_start() {
        assert_eq!(
            Solution::to_goat_latin("Each word".to_string()),
            "Eachma ordwmaa"
        );
    }
}
