use std::collections::HashMap;

impl Solution {
    /// Checks if two strings are isomorphic using bidirectional character mapping.
    ///
    /// # Intuition
    /// Two strings are isomorphic if a consistent one-to-one mapping exists in
    /// both directions between their characters.
    ///
    /// # Approach
    /// 1. Verify mapping from s to t: each character in s maps to exactly one in t.
    /// 2. Verify mapping from t to s: each character in t maps to exactly one in s.
    /// 3. Both directions must hold for isomorphism.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the string length
    /// - Space: O(k) where k is the character set size
    pub fn is_isomorphic(s: String, t: String) -> bool {
        Self::maps_consistently(s.as_bytes(), t.as_bytes())
            && Self::maps_consistently(t.as_bytes(), s.as_bytes())
    }

    fn maps_consistently(a: &[u8], b: &[u8]) -> bool {
        let mut mapping = HashMap::new();
        a.iter()
            .zip(b.iter())
            .all(|(&ac, &bc)| match mapping.get(&ac) {
                Some(&mapped) => mapped == bc,
                None => {
                    mapping.insert(ac, bc);
                    true
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isomorphic_strings() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
    }

    #[test]
    fn not_isomorphic() {
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
    }

    #[test]
    fn isomorphic_with_mapping() {
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
    }
}
