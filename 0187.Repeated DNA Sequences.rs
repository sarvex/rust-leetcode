use std::collections::HashMap;

impl Solution {
    /// Finds all 10-letter DNA sequences that occur more than once using a hash map.
    ///
    /// # Intuition
    /// Slide a window of size 10 across the string, counting occurrences.
    /// Report sequences on their second occurrence to avoid duplicates.
    ///
    /// # Approach
    /// 1. Iterate through all 10-character substrings.
    /// 2. Track counts in a HashMap.
    /// 3. Add to results exactly when count reaches 2.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the string length
    /// - Space: O(n) for the hash map
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return Vec::new();
        }
        let mut counts = HashMap::new();
        let mut result = Vec::new();
        for i in 0..=s.len() - 10 {
            let seq = &s[i..i + 10];
            let count = counts.entry(seq).or_insert(0);
            *count += 1;
            if *count == 2 {
                result.push(seq.to_string());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_repeats() {
        let mut result =
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());
        result.sort();
        assert_eq!(result, vec!["AAAAACCCCC", "CCCCCAAAAA"]);
    }

    #[test]
    fn single_repeat() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
            vec!["AAAAAAAAAA"]
        );
    }

    #[test]
    fn too_short() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("ACGT".to_string()),
            Vec::<String>::new()
        );
    }
}
