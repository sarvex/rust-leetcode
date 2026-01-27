impl Solution {
    /// Checks if two strings are anagrams using character frequency counting.
    ///
    /// # Intuition
    /// Two strings are anagrams if they have identical character frequencies.
    /// A fixed-size array for 26 lowercase letters provides O(1) lookup.
    ///
    /// # Approach
    /// 1. Return false if lengths differ.
    /// 2. Count character frequencies using a 26-element array.
    /// 3. Increment for s, decrement for t.
    /// 4. All counts must be zero for an anagram.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 26-element array
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0i32; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            counts[(b - b'a') as usize] -= 1;
        }
        counts.iter().all(|&c| c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_anagram() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn not_anagram() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn different_lengths() {
        assert!(!Solution::is_anagram("a".to_string(), "ab".to_string()));
    }
}
