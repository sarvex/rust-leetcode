impl Solution {
    /// Finds the longest subsequence with unequal adjacent groups and Hamming distance one.
    ///
    /// # Intuition
    /// This is a longest increasing subsequence variant where transitions require
    /// different groups and exactly one character difference (Hamming distance = 1).
    ///
    /// # Approach
    /// 1. Define a helper to check if two words differ by exactly one character.
    /// 2. Use DP arrays `f` (lengths) and `g` (predecessors) to track the LIS.
    /// 3. For each pair `(i, j)` where `j < i`, extend the subsequence if groups
    ///    differ and the words have Hamming distance one.
    /// 4. Reconstruct the path from the optimal endpoint.
    ///
    /// # Complexity
    /// - Time: O(n² * L) where n is the number of words and L is max word length
    /// - Space: O(n) for DP arrays and result
    pub fn get_words_in_longest_subsequence(
        n: i32,
        words: Vec<String>,
        groups: Vec<i32>,
    ) -> Vec<String> {
        let check = |s: &str, t: &str| -> bool {
            s.len() == t.len()
                && s.as_bytes()
                    .iter()
                    .zip(t.as_bytes().iter())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1
        };

        let n = n as usize;
        let mut f = vec![1i32; n];
        let mut g = vec![-1i32; n];
        let mut mx = 1;

        for i in 0..n {
            for j in 0..i {
                if groups[i] != groups[j] && f[i] < f[j] + 1 && check(&words[i], &words[j]) {
                    f[i] = f[j] + 1;
                    g[i] = j as i32;
                    mx = mx.max(f[i]);
                }
            }
        }

        let mut i = (0..n).rev().find(|&i| f[i] == mx).unwrap_or(0);
        let mut ans = Vec::new();
        let mut j = i as i32;
        while j >= 0 {
            ans.push(words[j as usize].clone());
            j = g[j as usize];
        }

        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_subsequence() {
        let words: Vec<String> = vec!["bab", "dab", "cab"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = Solution::get_words_in_longest_subsequence(3, words, vec![1, 2, 2]);
        assert_eq!(result, vec!["bab", "dab"]);
    }

    #[test]
    fn test_single_word() {
        let words = vec!["a".to_string()];
        let result = Solution::get_words_in_longest_subsequence(1, words, vec![1]);
        assert_eq!(result, vec!["a"]);
    }
}
