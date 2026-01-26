impl Solution {
    /// Match-and-extend with palindrome precomputation
    ///
    /// # Intuition
    /// A palindrome formed by s_sub + t_sub requires matching characters from the start
    /// of s_sub with the end of t_sub (in reverse). Any unmatched middle portion must
    /// itself be a palindrome.
    ///
    /// # Approach
    /// 1. Precompute longest palindrome starting at each position in s
    /// 2. Precompute longest palindrome ending at each position in t
    /// 3. For each pair (i,j), find the longest match where s[i+k] = t[j-k]
    /// 4. Extend with a palindrome from either s or t in the middle
    ///
    /// # Complexity
    /// - Time: O(n^2 + m^2 + n*m) for DP precomputation and main loop
    /// - Space: O(n^2 + m^2) for the palindrome DP tables
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let n = s.len();
        let m = t.len();

        let lps_s = Self::longest_palindrome_starting(&s);
        let lpe_t = Self::longest_palindrome_ending(&t);

        let mut result = lps_s.iter().copied().max().unwrap_or(0);
        result = result.max(lpe_t.iter().copied().max().unwrap_or(0));

        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                let k = (0..)
                    .take_while(|&k| i + k < n && k <= j && s[i + k] == t[j - k])
                    .count();

                if k > 0 {
                    let ext_s = if i + k < n { lps_s[i + k] } else { 0 };
                    let ext_t = if j >= k { lpe_t[j - k] } else { 0 };
                    result = result.max(2 * k + ext_s.max(ext_t));
                }
            });
        });

        result as i32
    }

    fn longest_palindrome_starting(s: &[char]) -> Vec<usize> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }

        let mut result = vec![1; n];
        let mut is_pal = vec![vec![false; n]; n];

        (0..n).for_each(|i| is_pal[i][i] = true);
        (0..n.saturating_sub(1)).for_each(|i| is_pal[i][i + 1] = s[i] == s[i + 1]);

        (3..=n).for_each(|len| {
            (0..=n - len).for_each(|i| {
                let j = i + len - 1;
                is_pal[i][j] = s[i] == s[j] && is_pal[i + 1][j - 1];
            });
        });

        (0..n).for_each(|i| {
            result[i] = (i..n)
                .filter(|&j| is_pal[i][j])
                .map(|j| j - i + 1)
                .max()
                .unwrap_or(1);
        });

        result
    }

    fn longest_palindrome_ending(s: &[char]) -> Vec<usize> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }

        let mut result = vec![1; n];
        let mut is_pal = vec![vec![false; n]; n];

        (0..n).for_each(|i| is_pal[i][i] = true);
        (0..n.saturating_sub(1)).for_each(|i| is_pal[i][i + 1] = s[i] == s[i + 1]);

        (3..=n).for_each(|len| {
            (0..=n - len).for_each(|i| {
                let j = i + len - 1;
                is_pal[i][j] = s[i] == s[j] && is_pal[i + 1][j - 1];
            });
        });

        (0..n).for_each(|j| {
            result[j] = (0..=j)
                .filter(|&i| is_pal[i][j])
                .map(|i| j - i + 1)
                .max()
                .unwrap_or(1);
        });

        result
    }
}
