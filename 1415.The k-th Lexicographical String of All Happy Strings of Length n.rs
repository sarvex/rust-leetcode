impl Solution {
    /// Combinatorial indexing to find the k-th happy string of length n.
    ///
    /// # Intuition
    /// Happy strings of length n form a complete ternary-then-binary tree: the first character
    /// has 3 choices (`a`, `b`, `c`) and every subsequent character has exactly 2 choices (anything
    /// except the previous character). The total count is `3 × 2^(n−1)`, and lexicographic order
    /// maps directly to a mixed-radix numbering system, so we can decode the 0-indexed rank
    /// digit by digit without enumerating all strings.
    ///
    /// # Approach
    /// 1. Compute `total = 3 × 2^(n−1)`. If `k > total`, return `""`.
    /// 2. Let `idx = k − 1` (0-indexed rank).
    /// 3. First character: divide `idx` by `2^(n−1)` to get an index into `['a','b','c']`.
    /// 4. Each subsequent character: divide the remainder by the next power of 2, indexing into
    ///    the two sorted characters that differ from the previous one.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the output string
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as u32;
        let total = 3 * (1_i32 << (n - 1));
        if k > total {
            return String::new();
        }

        let mut result = Vec::with_capacity(n as usize);
        let mut idx = k - 1;

        // First character: 3-way split
        let group = 1 << (n - 1);
        let first = (idx / group) as u8;
        result.push(b'a' + first);
        idx %= group;

        // Subsequent characters: 2-way split among sorted valid successors
        for pos in 1..n {
            let group = 1 << (n - 1 - pos);
            let prev = *result.last().unwrap();
            let choices = match prev {
                b'a' => [b'b', b'c'],
                b'b' => [b'a', b'c'],
                _ => [b'a', b'b'],
            };
            let pick = (idx / group) as usize;
            result.push(choices[pick]);
            idx %= group;
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n1_k3() {
        assert_eq!(Solution::get_happy_string(1, 3), "c");
    }

    #[test]
    fn test_n1_k4() {
        assert_eq!(Solution::get_happy_string(1, 4), "");
    }

    #[test]
    fn test_n3_k9() {
        assert_eq!(Solution::get_happy_string(3, 9), "cab");
    }

    #[test]
    fn test_n2_all() {
        let expected = ["ab", "ac", "ba", "bc", "ca", "cb"];
        for (i, &exp) in expected.iter().enumerate() {
            assert_eq!(Solution::get_happy_string(2, (i + 1) as i32), exp);
        }
        assert_eq!(Solution::get_happy_string(2, 7), "");
    }

    #[test]
    fn test_n3_all() {
        let expected = [
            "aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc",
        ];
        for (i, &exp) in expected.iter().enumerate() {
            assert_eq!(Solution::get_happy_string(3, (i + 1) as i32), exp);
        }
        assert_eq!(Solution::get_happy_string(3, 13), "");
    }

    #[test]
    fn test_n10_k1() {
        assert_eq!(Solution::get_happy_string(10, 1), "ababababab");
    }

    #[test]
    fn test_n10_k100() {
        let result = Solution::get_happy_string(10, 100);
        assert_eq!(result.len(), 10);
        assert!(result.as_bytes().windows(2).all(|w| w[0] != w[1]));
        assert!(result.bytes().all(|b| b == b'a' || b == b'b' || b == b'c'));
    }
}
