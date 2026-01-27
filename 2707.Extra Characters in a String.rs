use std::collections::HashSet;

impl Solution {
    /// Minimum extra characters left over after optimal dictionary segmentation.
    ///
    /// # Intuition
    /// Dynamic programming where `f[i]` tracks the fewest extra characters for the
    /// first `i` characters. For each position, try every substring ending there.
    ///
    /// # Approach
    /// 1. Collect dictionary words into a `HashSet` for O(1) lookup.
    /// 2. Define `f[i]` = minimum extra chars for `s[0..i]`.
    /// 3. For each `i`, set `f[i] = f[i-1] + 1` (skip char), then check all `j < i`
    ///    whether `s[j..i]` is in the dictionary to potentially improve `f[i]`.
    ///
    /// # Complexity
    /// - Time: O(n² · k) where k is average substring comparison cost
    /// - Space: O(n + d) where d is total dictionary size
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let dict: HashSet<String> = dictionary.into_iter().collect();
        let n = s.len();
        let mut f = vec![0; n + 1];
        for i in 1..=n {
            f[i] = f[i - 1] + 1;
            (0..i).for_each(|j| {
                if dict.contains(&s[j..i]) {
                    f[i] = f[i].min(f[j]);
                }
            });
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_covers_full_string() {
        let dict = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(Solution::min_extra_char("leetcode".to_string(), dict), 0);
    }

    #[test]
    fn some_extra_characters_remain() {
        let dict = vec![
            "leet".to_string(),
            "code".to_string(),
            "leetcode".to_string(),
        ];
        assert_eq!(Solution::min_extra_char("leetscode".to_string(), dict), 1);
    }
}
