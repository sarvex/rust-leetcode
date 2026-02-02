#[derive(Clone)]
struct TrieNode {
    next: [Option<usize>; 26],
    best_index: usize,
}

impl TrieNode {
    fn new(best_index: usize) -> Self {
        Self {
            next: [None; 26],
            best_index,
        }
    }
}

fn is_better_index(current: usize, candidate: usize, lengths: &[usize]) -> bool {
    let current_len = lengths[current];
    let candidate_len = lengths[candidate];
    candidate_len < current_len || (candidate_len == current_len && candidate < current)
}

fn update_best_index(current: &mut usize, candidate: usize, lengths: &[usize]) {
    if is_better_index(*current, candidate, lengths) {
        *current = candidate;
    }
}

impl Solution {
    /// Build a reversed trie and pick the shortest matching suffix.
    ///
    /// # Intuition
    /// A longest common suffix becomes a longest common prefix after reversing strings, so a
    /// trie over reversed container words can answer each query by walking as far as possible.
    ///
    /// # Approach
    /// - Insert each container word reversed into a trie.
    /// - At every node, store the best index among words passing through it, ranked by
    ///   shortest length then earliest index.
    /// - For each query, traverse the trie along its reversed characters and keep the last
    ///   reachable node's best index as the answer.
    ///
    /// # Complexity
    /// - Time: O(total_container_len + total_query_len)
    /// - Space: O(total_container_len)
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let lengths: Vec<usize> = words_container.iter().map(|word| word.len()).collect();
        let mut nodes = vec![TrieNode::new(0)];

        for (idx, word) in words_container.iter().enumerate() {
            update_best_index(&mut nodes[0].best_index, idx, &lengths);
            let mut node_idx = 0;
            for &byte in word.as_bytes().iter().rev() {
                let child = (byte - b'a') as usize;
                let next_idx = match nodes[node_idx].next[child] {
                    Some(existing) => existing,
                    None => {
                        nodes.push(TrieNode::new(idx));
                        let new_idx = nodes.len() - 1;
                        nodes[node_idx].next[child] = Some(new_idx);
                        new_idx
                    }
                };
                node_idx = next_idx;
                update_best_index(&mut nodes[node_idx].best_index, idx, &lengths);
            }
        }

        words_query
            .iter()
            .map(|query| {
                let mut node_idx = 0;
                let mut best = nodes[0].best_index;
                for &byte in query.as_bytes().iter().rev() {
                    let child = (byte - b'a') as usize;
                    match nodes[node_idx].next[child] {
                        Some(next_idx) => {
                            node_idx = next_idx;
                            best = nodes[node_idx].best_index;
                        }
                        None => break,
                    }
                }
                best as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words_container = vec!["abcd", "bcd", "xbcd"]
            .into_iter()
            .map(String::from)
            .collect();
        let words_query = vec!["cd", "bcd", "xyz"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn test_example_2() {
        let words_container = vec!["abcdefgh", "poiuygh", "ghghgh"]
            .into_iter()
            .map(String::from)
            .collect();
        let words_query = vec!["gh", "acbfgh", "acbfegh"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            vec![2, 0, 2]
        );
    }

    #[test]
    fn test_tie_by_length_then_index() {
        let words_container = vec!["aa", "ba", "ca"]
            .into_iter()
            .map(String::from)
            .collect();
        let words_query = vec!["a", "za"].into_iter().map(String::from).collect();
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            vec![0, 0]
        );
    }

    #[test]
    fn test_no_matching_suffix() {
        let words_container = vec!["abc", "de", "f"]
            .into_iter()
            .map(String::from)
            .collect();
        let words_query = vec!["x", "yz"].into_iter().map(String::from).collect();
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            vec![2, 2]
        );
    }
}
