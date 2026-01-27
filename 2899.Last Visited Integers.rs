impl Solution {
    /// Returns the last visited integers for each "prev" command.
    ///
    /// # Intuition
    /// Maintain a stack of seen integers. On each consecutive "prev", look back
    /// further into the stack. Any non-"prev" word resets the counter.
    ///
    /// # Approach
    /// 1. Iterate over words, tracking a counter `k` for consecutive "prev" occurrences.
    /// 2. On "prev", increment `k` and index backwards into the collected numbers.
    /// 3. On a numeric word, reset `k` and push the parsed integer.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of words
    /// - Space: O(n) for the nums and result vectors
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        let mut nums: Vec<i32> = Vec::new();
        let mut ans: Vec<i32> = Vec::new();
        let mut k = 0i32;

        for w in &words {
            match w.as_str() {
                "prev" => {
                    k += 1;
                    let i = nums.len() as i32 - k;
                    ans.push(if i < 0 { -1 } else { nums[i as usize] });
                }
                _ => {
                    k = 0;
                    nums.push(w.parse::<i32>().unwrap_or(0));
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
    fn test_basic_prev() {
        let words: Vec<String> = vec!["1", "2", "prev", "prev", "prev"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::last_visited_integers(words), vec![2, 1, -1]);
    }

    #[test]
    fn test_interleaved() {
        let words: Vec<String> = vec!["1", "prev", "2", "prev", "prev"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::last_visited_integers(words), vec![1, 2, 1]);
    }
}
