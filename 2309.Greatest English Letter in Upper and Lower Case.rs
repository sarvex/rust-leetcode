impl Solution {
    /// Finds the greatest letter present in both upper and lower case.
    ///
    /// # Intuition
    /// Track which letters appear in lowercase (bit 0) and uppercase (bit 1).
    /// The greatest letter with both bits set is the answer.
    ///
    /// # Approach
    /// 1. Scan bytes, setting bitmask flags per letter index
    /// 2. Iterate from 'Z' down to 'A', returning the first letter with mask == 3
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed array of 26 elements
    pub fn greatest_letter(s: String) -> String {
        let mut flags = [0u8; 26];
        s.as_bytes().iter().for_each(|b| match b {
            b'a'..=b'z' => flags[(b - b'a') as usize] |= 1,
            b'A'..=b'Z' => flags[(b - b'A') as usize] |= 2,
            _ => {}
        });

        (0..26)
            .rev()
            .find(|&i| flags[i] == 3)
            .map(|i| char::from(b'A' + i as u8).to_string())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::greatest_letter("lEeTcOdE".to_string()),
            "E".to_string()
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::greatest_letter("arRAzFif".to_string()),
            "R".to_string()
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            Solution::greatest_letter("AbCdEfGhIjK".to_string()),
            String::new()
        );
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::greatest_letter("aA".to_string()), "A".to_string());
    }

    #[test]
    fn test_all_lowercase() {
        assert_eq!(
            Solution::greatest_letter("abcdefg".to_string()),
            String::new()
        );
    }
}
