use std::collections::{HashSet, VecDeque};

impl Solution {
    /// Finds the minimum number of stickers to spell the target using BFS on bitmask states.
    ///
    /// # Intuition
    /// Represent which target characters are covered as a bitmask. BFS explores
    /// states level by level, where each level adds one sticker.
    ///
    /// # Approach
    /// 1. BFS from state 0 (no characters covered).
    /// 2. For each state, try every sticker: greedily match characters to advance the bitmask.
    /// 3. Return the BFS depth when the full bitmask is reached.
    ///
    /// # Complexity
    /// - Time: O(2^n × S × n) where n = target length, S = sticker count
    /// - Space: O(2^n)
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let target_bytes = target.as_bytes();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(0u32);
        visited.insert(0u32);
        let full = (1u32 << n) - 1;
        let mut steps = 0;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let state = queue.pop_front().unwrap();
                if state == full {
                    return steps;
                }
                for sticker in &stickers {
                    let mut next = state;
                    let mut cnt = [0u8; 26];
                    for &b in sticker.as_bytes() {
                        cnt[(b - b'a') as usize] += 1;
                    }
                    for (i, &c) in target_bytes.iter().enumerate() {
                        let idx = (c - b'a') as usize;
                        if (next & (1 << i)) == 0 && cnt[idx] > 0 {
                            next |= 1 << i;
                            cnt[idx] -= 1;
                        }
                    }
                    if visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::min_stickers(
                vec![
                    "with".to_string(),
                    "example".to_string(),
                    "science".to_string()
                ],
                "thehat".to_string(),
            ),
            3
        );
    }

    #[test]
    fn test_impossible() {
        assert_eq!(
            Solution::min_stickers(vec!["a".to_string()], "b".to_string()),
            -1
        );
    }
}
