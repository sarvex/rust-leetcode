impl Solution {
    /// Recursively decompose special substrings, sort descending, and concatenate.
    ///
    /// # Intuition
    /// A special string starts with `1` and ends with `0`. It can be split at
    /// balance points (where count of 1's equals 0's) into consecutive special
    /// substrings. Swapping such substrings is allowed; to get the lexicographically
    /// largest result, recursively make each part largest and sort the parts in
    /// descending order before concatenating.
    ///
    /// # Approach
    /// 1. Parse `s` into maximal special substrings using a balance counter (+1 for
    ///    `'1'`, -1 for `'0'`); each time balance hits 0, a substring ends.
    /// 2. For each substring of the form `1 + middle + 0`, recursively compute
    ///    `make_largest_special(middle)`.
    /// 3. Sort the list of processed substrings in descending lexicographic order.
    /// 4. Concatenate and return.
    ///
    /// # Complexity
    /// - Time: O(n²) in the worst case due to recursion and sorting at each level.
    /// - Space: O(n) for recursion stack and intermediate strings.
    ///
    /// # Panics
    /// Does not panic; assumes `s` is a valid special binary string.
    pub fn make_largest_special(s: String) -> String {
        if s.len() <= 2 {
            return s;
        }
        let mut parts: Vec<String> = Vec::new();
        let bytes = s.as_bytes();
        let mut balance = 0u32;
        let mut start = 0usize;
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'1' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance == 0 {
                let sub = &s[start..=i];
                let processed = if sub.len() <= 2 {
                    sub.to_string()
                } else {
                    let inner = Self::make_largest_special(sub[1..sub.len() - 1].to_string());
                    format!("1{}0", inner)
                };
                parts.push(processed);
                start = i + 1;
            }
        }
        parts.sort_by(|a, b| b.cmp(a));
        parts.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::make_largest_special("11011000".to_string()),
            "11100100"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::make_largest_special("10".to_string()), "10");
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::make_largest_special("10".to_string()), "10");
    }

    #[test]
    fn test_no_swap_needed() {
        // "111000": inner "1100" is one special substring (balance 1,2,1,0), so no reorder; result unchanged
        assert_eq!(
            Solution::make_largest_special("111000".to_string()),
            "111000"
        );
    }
}
