impl Solution {
    /// Find a binary string of length n absent from the given array using Cantor's diagonal.
    ///
    /// # Intuition
    /// Cantor's diagonalisation guarantees a string that differs from every
    /// `nums[i]` at position `i`. By flipping the i-th character of the i-th
    /// string we construct a result that cannot equal any element of `nums`.
    ///
    /// # Approach
    /// Iterate over each index `i` from `0` to `n - 1`. Inspect `nums[i][i]`:
    /// if it is `'0'`, append `'1'` to the result; otherwise append `'0'`.
    /// The resulting string differs from `nums[i]` at index `i` for every `i`,
    /// so it is guaranteed to be absent from `nums`.
    ///
    /// # Complexity
    /// - Time: O(n) — one pass over the diagonal.
    /// - Space: O(n) — the output string.
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        nums.iter()
            .enumerate()
            .map(|(i, s)| if s.as_bytes()[i] == b'0' { '1' } else { '0' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec!["01".to_string(), "10".to_string()];
        let result = Solution::find_different_binary_string(nums.clone());
        assert_eq!(result.len(), 2);
        assert!(!nums.contains(&result));
    }

    #[test]
    fn test_example_2() {
        let nums = vec!["00".to_string(), "01".to_string()];
        let result = Solution::find_different_binary_string(nums.clone());
        assert_eq!(result.len(), 2);
        assert!(!nums.contains(&result));
    }

    #[test]
    fn test_example_3() {
        let nums = vec!["111".to_string(), "011".to_string(), "001".to_string()];
        let result = Solution::find_different_binary_string(nums.clone());
        assert_eq!(result.len(), 3);
        assert!(!nums.contains(&result));
    }

    #[test]
    fn test_single_element() {
        let nums = vec!["0".to_string()];
        let result = Solution::find_different_binary_string(nums.clone());
        assert_eq!(result.len(), 1);
        assert!(!nums.contains(&result));
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![
            "0000".to_string(),
            "0000".to_string(),
            "0000".to_string(),
            "0000".to_string(),
        ];
        let result = Solution::find_different_binary_string(nums.clone());
        assert_eq!(result.len(), 4);
        assert!(!nums.contains(&result));
    }

    #[test]
    fn test_binary_only_chars() {
        let nums = vec!["10".to_string(), "01".to_string()];
        let result = Solution::find_different_binary_string(nums);
        assert!(result.chars().all(|c| c == '0' || c == '1'));
    }
}
