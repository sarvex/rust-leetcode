impl Solution {
    /// Selects the longest subsequence where adjacent elements have different groups.
    ///
    /// # Intuition
    /// Greedily pick words whenever the group changes from the previous selection,
    /// producing the longest alternating subsequence in one pass.
    ///
    /// # Approach
    /// 1. Iterate over `groups` with `enumerate`.
    /// 2. Include the word at index `i` if it is the first element or its group
    ///    differs from the previous element's group.
    /// 3. Collect the corresponding words.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of words
    /// - Space: O(n) for the result vector
    pub fn get_words_in_longest_subsequence(
        _n: i32,
        words: Vec<String>,
        groups: Vec<i32>,
    ) -> Vec<String> {
        groups
            .iter()
            .enumerate()
            .filter(|(i, g)| *i == 0 || **g != groups[i - 1])
            .map(|(i, _)| words[i].clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternating_groups() {
        let words: Vec<String> = vec!["a", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = Solution::get_words_in_longest_subsequence(4, words, vec![1, 0, 1, 1]);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_single_group() {
        let words: Vec<String> = vec!["a", "b"].into_iter().map(String::from).collect();
        let result = Solution::get_words_in_longest_subsequence(2, words, vec![0, 0]);
        assert_eq!(result, vec!["a"]);
    }
}
