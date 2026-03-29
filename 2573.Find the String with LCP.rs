impl Solution {
    /// Build the lexicographically smallest string whose LCP matrix matches `lcp`.
    ///
    /// # Intuition
    /// `lcp[i][j] > 0` forces `word[i] == word[j]`. Greedily assigning the smallest unused
    /// letter left-to-right and propagating through every positive `lcp` entry produces the
    /// lex-smallest candidate; a bottom-up rolling-row recomputation confirms or rejects it.
    ///
    /// # Approach
    /// 1. Scan left-to-right; for each unassigned position assign the next letter and propagate
    ///    it to every later position linked by a positive `lcp` value.
    /// 2. Recompute the LCP matrix row-by-row in O(n) space using a single rolling array
    ///    and a saved diagonal value, comparing each entry against the input.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut word = vec![0u8; n];
        let mut c = b'a';

        for i in 0..n {
            if word[i] == 0 {
                if c > b'z' {
                    return String::new();
                }
                word[i] = c;
                c += 1;
            }
            for j in (i + 1)..n {
                if lcp[i][j] > 0 {
                    if word[j] == 0 {
                        word[j] = word[i];
                    } else if word[j] != word[i] {
                        return String::new();
                    }
                }
            }
        }

        let mut dp = vec![0i32; n + 1];
        for i in (0..n).rev() {
            let mut prev = 0i32;
            for j in (0..n).rev() {
                let temp = dp[j];
                dp[j] = if word[i] == word[j] { prev + 1 } else { 0 };
                if dp[j] != lcp[i][j] {
                    return String::new();
                }
                prev = temp;
            }
        }

        String::from_utf8(word).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let lcp = vec![
            vec![4, 0, 2, 0],
            vec![0, 3, 0, 1],
            vec![2, 0, 2, 0],
            vec![0, 1, 0, 1],
        ];
        assert_eq!(Solution::find_the_string(lcp), "abab");
    }

    #[test]
    fn example_two() {
        let lcp = vec![
            vec![4, 3, 2, 1],
            vec![3, 3, 2, 1],
            vec![2, 2, 2, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(Solution::find_the_string(lcp), "aaaa");
    }

    #[test]
    fn example_three() {
        let lcp = vec![
            vec![4, 3, 2, 1],
            vec![3, 3, 2, 1],
            vec![2, 2, 2, 1],
            vec![1, 1, 1, 3],
        ];
        assert_eq!(Solution::find_the_string(lcp), "");
    }

    #[test]
    fn single_character() {
        let lcp = vec![vec![1]];
        assert_eq!(Solution::find_the_string(lcp), "a");
    }

    #[test]
    fn asymmetric_matrix() {
        let lcp = vec![vec![2, 1], vec![0, 1]];
        assert_eq!(Solution::find_the_string(lcp), "");
    }

    #[test]
    fn wrong_diagonal() {
        let lcp = vec![vec![2, 0], vec![0, 1]];
        assert_eq!(Solution::find_the_string(lcp), "");
    }

    #[test]
    fn too_many_classes() {
        let n = 27;
        let mut lcp = vec![vec![0i32; n]; n];
        for i in 0..n {
            lcp[i][i] = (n - i) as i32;
        }
        assert_eq!(Solution::find_the_string(lcp), "");
    }
}
