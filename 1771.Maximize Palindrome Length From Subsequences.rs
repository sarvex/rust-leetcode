impl Solution {
    /// Longest palindromic subsequence spanning both word1 and word2.
    ///
    /// # Intuition
    /// Concatenate both words and apply longest palindromic subsequence DP.
    /// The constraint is that the palindrome must use at least one character
    /// from each word, so only update the answer when the match spans the boundary.
    ///
    /// # Approach
    /// 1. Concatenate `word1 + word2` into a single character array.
    /// 2. Use interval DP: `f[i][j]` = longest palindromic subsequence in `s[i..=j]`.
    /// 3. When `s[i] == s[j]` and `i < len(word1)` and `j >= len(word1)`, update answer.
    ///
    /// # Complexity
    /// - Time: O(n²) where n = len(word1) + len(word2)
    /// - Space: O(n²)
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let boundary = word1.len();
        let s: Vec<u8> = word1.bytes().chain(word2.bytes()).collect();
        let n = s.len();
        let mut f = vec![vec![0i32; n]; n];
        for i in 0..n {
            f[i][i] = 1;
        }
        let mut ans = 0;
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if s[i] == s[j] {
                    f[i][j] = f[i + 1][j - 1] + 2;
                    if i < boundary && j >= boundary {
                        ans = ans.max(f[i][j]);
                    }
                } else {
                    f[i][j] = f[i + 1][j].max(f[i][j - 1]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::longest_palindrome("cacb".to_string(), "cbba".to_string()),
            5
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::longest_palindrome("ab".to_string(), "ab".to_string()),
            3
        );
    }

    #[test]
    fn test_no_common() {
        assert_eq!(
            Solution::longest_palindrome("aa".to_string(), "bb".to_string()),
            0
        );
    }
}
