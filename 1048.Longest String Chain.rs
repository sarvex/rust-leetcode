use std::collections::HashMap;


impl Solution {
    /// Finds the longest word chain where each word is a predecessor of the next.
    ///
    /// # Intuition
    /// Sort words by length. For each word, try removing each character to
    /// form a predecessor. If the predecessor exists in the DP map, extend
    /// its chain length.
    ///
    /// # Approach
    /// Sort by length. For each word, compute the best chain by checking all
    /// single-character deletions against the DP map. Store the result.
    ///
    /// # Complexity
    /// - Time: O(n * L^2) where L is max word length
    /// - Space: O(n * L) for the hash map
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by_key(|w| w.len());
        let mut dp: HashMap<String, i32> = HashMap::with_capacity(words.len());
        let mut max_chain = 1;

        for word in &words {
            let n = word.len();
            let mut best = 1;
            for i in 0..n {
                let pred = format!("{}{}", &word[..i], &word[i + 1..]);
                if let Some(&chain) = dp.get(&pred) {
                    best = best.max(chain + 1);
                }
            }
            dp.insert(word.clone(), best);
            max_chain = max_chain.max(best);
        }

        max_chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::longest_str_chain(to_vec(&["a", "b", "ba", "bca", "bda", "bdca"])),
            4
        );
    }

    #[test]
    fn test_single_chain() {
        assert_eq!(
            Solution::longest_str_chain(to_vec(&["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"])),
            5
        );
    }

    #[test]
    fn test_no_chain() {
        assert_eq!(Solution::longest_str_chain(to_vec(&["abcd", "dbqca"])), 1);
    }
}
