use std::collections::VecDeque;

impl Solution {
    /// Minimum-cost segmentation with an Aho-Corasick automaton.
    ///
    /// # Intuition
    /// We need the cheapest way to split `target` into dictionary words. Efficiently finding all
    /// word matches ending at each position enables a dynamic programming solution.
    ///
    /// # Approach
    /// Build an Aho-Corasick automaton over the words (keeping the minimum cost for duplicates).
    /// Scan the target once, and for each position enumerate all matched words ending there via
    /// dictionary suffix links. Let `dp[i]` be the minimum cost to build the prefix of length `i`.
    /// Update `dp[i]` using matches that end at `i`.
    ///
    /// # Complexity
    /// - Time: O(n + total_word_length + matches)
    /// - Space: O(n + total_word_length)
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        #[derive(Debug)]
        struct Node {
            next: [i32; 26],
            link: usize,
            dict_link: Option<usize>,
            word_cost: Option<i32>,
            depth: usize,
        }

        impl Node {
            fn new(depth: usize) -> Self {
                Self {
                    next: [-1; 26],
                    link: 0,
                    dict_link: None,
                    word_cost: None,
                    depth,
                }
            }
        }

        let mut nodes = Vec::new();
        nodes.push(Node::new(0));

        for (word, cost) in words.into_iter().zip(costs.into_iter()) {
            let mut state = 0usize;
            for &byte in word.as_bytes() {
                let idx = (byte - b'a') as usize;
                let next = nodes[state].next[idx];
                if next == -1 {
                    let next_index = nodes.len();
                    nodes.push(Node::new(nodes[state].depth + 1));
                    nodes[state].next[idx] = next_index as i32;
                    state = next_index;
                } else {
                    state = next as usize;
                }
            }
            match nodes[state].word_cost {
                Some(existing) => {
                    if cost < existing {
                        nodes[state].word_cost = Some(cost);
                    }
                }
                None => {
                    nodes[state].word_cost = Some(cost);
                }
            }
        }

        let mut queue = VecDeque::new();
        for idx in 0..26 {
            let next = nodes[0].next[idx];
            if next == -1 {
                nodes[0].next[idx] = 0;
            } else {
                let child = next as usize;
                nodes[child].link = 0;
                nodes[child].dict_link = None;
                queue.push_back(child);
            }
        }

        while let Some(state) = queue.pop_front() {
            for idx in 0..26 {
                let next = nodes[state].next[idx];
                if next == -1 {
                    let link = nodes[state].link;
                    nodes[state].next[idx] = nodes[link].next[idx];
                    continue;
                }
                let child = next as usize;
                let link = nodes[state].link;
                let link_next = nodes[link].next[idx] as usize;
                nodes[child].link = link_next;
                nodes[child].dict_link = if nodes[link_next].word_cost.is_some() {
                    Some(link_next)
                } else {
                    nodes[link_next].dict_link
                };
                queue.push_back(child);
            }
        }

        let n = target.len();
        let bytes = target.as_bytes();
        let inf = i64::MAX / 4;
        let mut dp = vec![inf; n + 1];
        dp[0] = 0;
        let mut state = 0usize;

        for (i, &byte) in bytes.iter().enumerate() {
            let idx = (byte - b'a') as usize;
            state = nodes[state].next[idx] as usize;

            if let Some(cost) = nodes[state].word_cost {
                let len = nodes[state].depth;
                let prev = dp[i + 1 - len];
                if prev != inf {
                    dp[i + 1] = dp[i + 1].min(prev + cost as i64);
                }
            }

            let mut link = nodes[state].dict_link;
            while let Some(node_idx) = link {
                let node = &nodes[node_idx];
                let len = node.depth;
                if let Some(cost) = node.word_cost {
                    let prev = dp[i + 1 - len];
                    if prev != inf {
                        dp[i + 1] = dp[i + 1].min(prev + cost as i64);
                    }
                }
                link = node.dict_link;
            }
        }

        if dp[n] >= inf { -1 } else { dp[n] as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let target = "abcdef".to_string();
        let words = vec![
            "abdef".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "def".to_string(),
            "ef".to_string(),
        ];
        let costs = vec![100, 1, 1, 10, 5];
        assert_eq!(Solution::minimum_cost(target, words, costs), 7);
    }

    #[test]
    fn test_example_2() {
        let target = "aaaa".to_string();
        let words = vec!["z".to_string(), "zz".to_string(), "zzz".to_string()];
        let costs = vec![1, 10, 100];
        assert_eq!(Solution::minimum_cost(target, words, costs), -1);
    }

    #[test]
    fn test_duplicate_words() {
        let target = "aa".to_string();
        let words = vec!["a".to_string(), "a".to_string(), "aa".to_string()];
        let costs = vec![5, 1, 10];
        assert_eq!(Solution::minimum_cost(target, words, costs), 2);
    }

    #[test]
    fn test_prefer_longer_when_cheaper() {
        let target = "aaaa".to_string();
        let words = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
        let costs = vec![3, 4, 10];
        assert_eq!(Solution::minimum_cost(target, words, costs), 8);
    }
}
