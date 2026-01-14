impl Solution {
    /// Match Substring After Replacement
    ///
    /// # Intuition
    /// Build a lookup table for valid character transformations (including identity mappings),
    /// then slide through all possible starting positions to find a match.
    ///
    /// # Approach
    /// 1. Create a 128x128 boolean lookup table for O(1) transformation validity checks
    /// 2. Initialize identity mappings (each character can match itself)
    /// 3. Add all provided mappings to the lookup table
    /// 4. For each valid starting position in `s`, check if `sub` can transform to match
    /// 5. Return early on first successful match
    ///
    /// # Complexity
    /// - Time: O(n * m) where n = len(s), m = len(sub)
    /// - Space: O(1) - fixed 128x128 lookup table (16KB)
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let s_bytes = s.as_bytes();
        let sub_bytes = sub.as_bytes();
        let s_len = s_bytes.len();
        let sub_len = sub_bytes.len();

        if sub_len > s_len {
            return false;
        }

        // Build transformation lookup table
        // can_transform[original][target] = true means original can become target
        let mut can_transform = [[false; 128]; 128];

        // Identity mappings: each character can match itself
        for i in 0..128 {
            can_transform[i][i] = true;
        }

        // Add provided mappings
        for mapping in &mappings {
            let old_char = mapping[0] as usize;
            let new_char = mapping[1] as usize;
            can_transform[old_char][new_char] = true;
        }

        // Try each starting position in s
        'outer: for start in 0..=(s_len - sub_len) {
            // Check if sub can transform to match s[start..start+sub_len]
            for j in 0..sub_len {
                let sub_char = sub_bytes[j] as usize;
                let s_char = s_bytes[start + j] as usize;

                if !can_transform[sub_char][s_char] {
                    continue 'outer;
                }
            }
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "fool3e7bar".to_string();
        let sub = "leet".to_string();
        let mappings = vec![vec!['e', '3'], vec!['t', '7'], vec!['t', '8']];
        assert!(Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_example_2() {
        let s = "fooleetbar".to_string();
        let sub = "f00l".to_string();
        let mappings = vec![vec!['o', '0']];
        assert!(!Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_example_3() {
        let s = "Fool33tbaR".to_string();
        let sub = "leetd".to_string();
        let mappings = vec![
            vec!['e', '3'],
            vec!['t', '7'],
            vec!['t', '8'],
            vec!['d', 'b'],
            vec!['p', 'b'],
        ];
        assert!(Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_exact_match_no_mappings() {
        let s = "hello".to_string();
        let sub = "ell".to_string();
        let mappings: Vec<Vec<char>> = vec![];
        assert!(Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_sub_longer_than_s() {
        let s = "ab".to_string();
        let sub = "abc".to_string();
        let mappings: Vec<Vec<char>> = vec![];
        assert!(!Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_single_char_match() {
        let s = "a".to_string();
        let sub = "b".to_string();
        let mappings = vec![vec!['b', 'a']];
        assert!(Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_case_sensitive() {
        let s = "ABC".to_string();
        let sub = "abc".to_string();
        let mappings = vec![vec!['a', 'A'], vec!['b', 'B'], vec!['c', 'C']];
        assert!(Solution::match_replacement(s, sub, mappings));
    }

    #[test]
    fn test_digits() {
        let s = "123".to_string();
        let sub = "abc".to_string();
        let mappings = vec![vec!['a', '1'], vec!['b', '2'], vec!['c', '3']];
        assert!(Solution::match_replacement(s, sub, mappings));
    }
}
