impl Solution {
    /// Sorts a string according to a custom character ordering.
    ///
    /// # Intuition
    /// Assign each character a priority based on its position in the order
    /// string. Characters not in the order get the highest priority (appear last).
    ///
    /// # Approach
    /// Build a priority map from the order string. Sort the string's bytes
    /// by their mapped priority.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n) for the sorted output
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut priority = [order.len(); 26];
        for (i, b) in order.bytes().enumerate() {
            priority[(b - b'a') as usize] = i;
        }
        let mut bytes = s.into_bytes();
        bytes.sort_unstable_by_key(|&b| priority[(b - b'a') as usize]);
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
            "cbad"
        );
    }

    #[test]
    fn test_partial_order() {
        let result = Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string());
        assert_eq!(result, "bcad");
    }

    #[test]
    fn test_no_reorder_needed() {
        assert_eq!(
            Solution::custom_sort_string("abc".to_string(), "abc".to_string()),
            "abc"
        );
    }
}
