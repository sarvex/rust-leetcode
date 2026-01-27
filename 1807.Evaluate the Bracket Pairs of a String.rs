use std::collections::HashMap;

impl Solution {
    /// Evaluates bracket pairs by replacing keys with knowledge values.
    ///
    /// # Intuition
    /// Parse the string character by character, extracting keys between
    /// parentheses and looking them up in a pre-built hash map.
    ///
    /// # Approach
    /// 1. Build a hash map from the knowledge pairs.
    /// 2. Iterate through the string, collecting keys inside parentheses.
    /// 3. Replace each key with its mapped value or "?" if unknown.
    ///
    /// # Complexity
    /// - Time: O(n + m) where m is total knowledge size
    /// - Space: O(n + m)
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let map: HashMap<&str, &str> = knowledge
            .iter()
            .map(|pair| (pair[0].as_str(), pair[1].as_str()))
            .collect();

        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut result = String::with_capacity(n);
        let mut i = 0;

        while i < n {
            if bytes[i] == b'(' {
                i += 1;
                let start = i;
                while bytes[i] != b')' {
                    i += 1;
                }
                let key = &s[start..i];
                result.push_str(map.get(key).unwrap_or(&"?"));
            } else {
                result.push(bytes[i] as char);
            }
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        assert_eq!(
            Solution::evaluate(
                "(name)is(age)".to_string(),
                vec![
                    vec!["name".to_string(), "bob".to_string()],
                    vec!["age".to_string(), "two".to_string()],
                ]
            ),
            "bobistwо".replace("о", "o") // "bobistwo"
        );
    }

    #[test]
    fn test_unknown_key() {
        assert_eq!(
            Solution::evaluate(
                "hi(name)".to_string(),
                vec![vec!["a".to_string(), "b".to_string()]]
            ),
            "hi?"
        );
    }
}
