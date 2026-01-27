impl Solution {
    /// Finds the shortest word containing all letters from the license plate.
    ///
    /// # Intuition
    /// Count letter frequencies in the license plate, then find the shortest
    /// word whose letter counts meet or exceed every required frequency.
    ///
    /// # Approach
    /// Build a frequency array from the plate's alphabetic characters. For each
    /// candidate word, build its frequency array and check it covers the plate.
    /// Track the shortest valid word.
    ///
    /// # Complexity
    /// - Time: O(n * L) where n is word count and L is max word length
    /// - Space: O(1) — fixed 26-element arrays
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut required = [0u8; 26];
        for b in license_plate.bytes() {
            if b.is_ascii_alphabetic() {
                required[(b.to_ascii_lowercase() - b'a') as usize] += 1;
            }
        }

        let completes = |word: &str| -> bool {
            let mut counts = [0u8; 26];
            for b in word.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
            required.iter().zip(counts.iter()).all(|(r, c)| c >= r)
        };

        words
            .into_iter()
            .filter(|w| completes(w))
            .min_by_key(|w| w.len())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 PSt".to_string(),
                to_vec(&["step", "steps", "stripe", "stepple"]),
            ),
            "steps"
        );
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 456".to_string(),
                to_vec(&["looks", "pest", "stew", "show"]),
            ),
            "pest"
        );
    }
}
