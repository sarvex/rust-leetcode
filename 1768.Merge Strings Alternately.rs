/// Merges two strings by alternating characters from each string.
/// 
/// # Intuition
/// Iterate through both strings simultaneously, adding characters from each string
/// in alternating order until both strings are exhausted.
///
/// # Approach
/// 1. Use a `String` with a pre-allocated capacity to avoid reallocations
/// 2. Iterate through both strings in parallel using `zip`
/// 3. For each iteration, push characters from both strings
/// 4. Extend the result with any remaining characters from either string
///
/// # Complexity
/// - Time complexity: O(n + m), where n and m are the lengths of the input strings
/// - Space complexity: O(n + m) for the result string
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        for (c1, c2) in word1.chars().zip(word2.chars()) {
            result.push(c1);
            result.push(c2);
        }
        result.extend(word1.chars().skip(word2.len()));
        result.extend(word2.chars().skip(word1.len()));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        let solution = Solution {};
        
        assert_eq!(
            solution.merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
        
        assert_eq!(
            solution.merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
        
        assert_eq!(
            solution.merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
