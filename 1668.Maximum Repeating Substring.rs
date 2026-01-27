impl Solution {
    /// DP for maximum consecutive word repetitions in sequence.
    ///
    /// # Intuition
    /// At each position where `word` matches, the repetition count extends
    /// from the count at `position - word.len()`. A DP array tracks the
    /// longest chain ending at each position.
    ///
    /// # Approach
    /// 1. For each valid starting position, check if `word` matches
    /// 2. If match: `dp[i] = dp[i - m] + 1` (or 1 if no prior match)
    /// 3. Return the maximum dp value
    ///
    /// # Complexity
    /// - Time: O(n · m) for substring comparisons
    /// - Space: O(n) for the DP array
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let n = sequence.len();
        let m = word.len();
        if n < m {
            return 0;
        }

        let mut dp = vec![0i32; n - m + 1];
        for i in 0..=n - m {
            if &sequence[i..i + m] == word {
                dp[i] = if i >= m { dp[i - m] } else { 0 } + 1;
            }
        }

        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_twice() {
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ab".to_string()),
            2
        );
    }

    #[test]
    fn repeated_once() {
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ba".to_string()),
            1
        );
    }

    #[test]
    fn not_found() {
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ac".to_string()),
            0
        );
    }
}
