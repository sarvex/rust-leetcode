impl Solution {
    /// Minimum Operations to Convert String
    ///
    /// # Intuition
    /// Each contiguous substring can be transformed independently using reverse,
    /// swap, and replace operations. We use dynamic programming to find the optimal
    /// partitioning of the string, computing the cost for each possible segment.
    ///
    /// # Approach
    /// 1. Define dp[i] as minimum operations to transform word1[0..i] to word2[0..i]
    /// 2. For each position i, try all possible last partition boundaries j
    /// 3. For each partition [j..i], compute the minimum cost:
    ///    - Without reverse: count mismatches minus beneficial swaps
    ///    - With reverse: 1 + (count mismatches after reverse minus beneficial swaps)
    /// 4. A beneficial swap exists when source[i]=target[j] and source[j]=target[i]
    ///    for positions i,j - one swap fixes two mismatches
    /// 5. Count beneficial swaps by tracking character pairs and taking min of
    ///    forward and reverse transition counts
    ///
    /// # Complexity
    /// - Time: O(n³) where n is the string length
    /// - Space: O(n) for DP array plus O(26²) for character pair counting
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let w1: Vec<char> = word1.chars().collect();
        let w2: Vec<char> = word2.chars().collect();
        let n = w1.len();

        let compute_cost = |s: &[char], t: &[char]| -> i32 {
            let mut count = [[0i32; 26]; 26];
            let mut mismatches = 0;

            for (&sc, &tc) in s.iter().zip(t.iter()) {
                if sc != tc {
                    mismatches += 1;
                    let a = (sc as u8 - b'a') as usize;
                    let b = (tc as u8 - b'a') as usize;
                    count[a][b] += 1;
                }
            }

            let swaps = (0..26)
                .flat_map(|a| ((a + 1)..26).map(move |b| count[a][b].min(count[b][a])))
                .sum::<i32>();

            mismatches - swaps
        };

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for j in 0..i {
                let s1 = &w1[j..i];
                let t = &w2[j..i];

                let cost_no_reverse = compute_cost(s1, t);

                let s_reversed: Vec<char> = s1.iter().rev().copied().collect();
                let cost_with_reverse = 1 + compute_cost(&s_reversed, t);

                dp[i] = dp[i].min(dp[j] + cost_no_reverse.min(cost_with_reverse));
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "ab" -> reverse "ba" -> replace "da"; "c" unchanged; "df" -> "bf" -> "be"
        assert_eq!(
            Solution::min_operations("abcdf".to_string(), "dacbe".to_string()),
            4
        );
    }

    #[test]
    fn test_example_2() {
        // "ab" -> swap "ba"; "ce" -> swap "ec"; "ded" -> replace twice "fef"
        assert_eq!(
            Solution::min_operations("abceded".to_string(), "baecfef".to_string()),
            4
        );
    }

    #[test]
    fn test_example_3() {
        // "abcdef" -> reverse "fedcba" -> swap "fedabc"
        assert_eq!(
            Solution::min_operations("abcdef".to_string(), "fedabc".to_string()),
            2
        );
    }

    #[test]
    fn test_identical_strings() {
        assert_eq!(
            Solution::min_operations("abc".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn test_single_char_match() {
        assert_eq!(
            Solution::min_operations("a".to_string(), "a".to_string()),
            0
        );
    }

    #[test]
    fn test_single_char_mismatch() {
        assert_eq!(
            Solution::min_operations("a".to_string(), "b".to_string()),
            1
        );
    }

    #[test]
    fn test_simple_swap() {
        // One swap suffices
        assert_eq!(
            Solution::min_operations("ab".to_string(), "ba".to_string()),
            1
        );
    }

    #[test]
    fn test_simple_reverse() {
        // Reverse entire string
        assert_eq!(
            Solution::min_operations("abc".to_string(), "cba".to_string()),
            1
        );
    }

    #[test]
    fn test_all_different() {
        // No swaps beneficial, all replaces
        assert_eq!(
            Solution::min_operations("aaa".to_string(), "bbb".to_string()),
            3
        );
    }

    #[test]
    fn test_partial_match() {
        assert_eq!(
            Solution::min_operations("abcd".to_string(), "abce".to_string()),
            1
        );
    }
}
