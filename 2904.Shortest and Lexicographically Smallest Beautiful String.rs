impl Solution {
    /// Finds the shortest and lexicographically smallest beautiful substring with k ones.
    ///
    /// # Intuition
    /// A "beautiful" substring contains exactly k '1' bits. Among all such substrings,
    /// we want the shortest one, breaking ties lexicographically.
    ///
    /// # Approach
    /// 1. Enumerate all substrings starting from each index, with minimum length k.
    /// 2. Count '1' bits using `as_bytes()` filtering.
    /// 3. Track the best candidate by length then lexicographic order.
    ///
    /// # Complexity
    /// - Time: O(n³) for all substring enumeration and comparison
    /// - Space: O(n) for the result string
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;
        let mut ans = "";

        for i in 0..n {
            for j in (i + k)..=n {
                let t = &s[i..j];
                let ones = bytes[i..j].iter().filter(|&&b| b == b'1').count();
                if ones == k
                    && (ans.is_empty() || j - i < ans.len() || (j - i == ans.len() && t < ans))
                {
                    ans = t;
                }
            }
        }

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_beautiful() {
        assert_eq!(
            Solution::shortest_beautiful_substring("100011001".to_string(), 3),
            "11001"
        );
    }

    #[test]
    fn test_single_one() {
        assert_eq!(
            Solution::shortest_beautiful_substring("1011".to_string(), 1),
            "1"
        );
    }

    #[test]
    fn test_no_beautiful_substring() {
        assert_eq!(
            Solution::shortest_beautiful_substring("000".to_string(), 1),
            ""
        );
    }
}
