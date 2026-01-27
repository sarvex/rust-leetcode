use std::collections::HashMap;

impl Solution {
    /// Finds the k-th distinct string in the array.
    ///
    /// # Intuition
    /// Count occurrences first, then scan for the k-th string appearing exactly once.
    ///
    /// # Approach
    /// 1. Build a frequency map of all strings.
    /// 2. Iterate in order, counting distinct strings.
    /// 3. Return the k-th one found.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let mut freq = HashMap::new();
        for s in &arr {
            *freq.entry(s.as_str()).or_insert(0) += 1;
        }

        for s in &arr {
            if freq[s.as_str()] == 1 {
                k -= 1;
                if k == 0 {
                    return s.clone();
                }
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            ),
            "a"
        );
    }

    #[test]
    fn test_not_enough_distinct() {
        assert_eq!(
            Solution::kth_distinct(vec!["a".to_string(), "a".to_string()], 1),
            ""
        );
    }
}
