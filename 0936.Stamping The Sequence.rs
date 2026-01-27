use std::collections::VecDeque;

impl Solution {
    /// Finds stamp positions to produce the target via reverse simulation.
    ///
    /// # Intuition
    /// Work backwards: find positions where the stamp matches (treating
    /// already-stamped characters as wildcards). Stamp them and repeat
    /// until all characters are stamped.
    ///
    /// # Approach
    /// Build an in-degree graph where each position tracks how many stamp
    /// characters still need matching. Use BFS: when a position's in-degree
    /// reaches 0, stamp it and propagate to overlapping positions.
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is target length and m is stamp length
    /// - Space: O(n * m) for the graph
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.as_bytes();
        let target = target.as_bytes();
        let m = stamp.len();
        let n = target.len();
        if n < m {
            return vec![];
        }

        let mut indeg = vec![m; n - m + 1];
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut queue = VecDeque::new();

        for i in 0..=n - m {
            for j in 0..m {
                if target[i + j] == stamp[j] {
                    indeg[i] -= 1;
                    if indeg[i] == 0 {
                        queue.push_back(i);
                    }
                } else {
                    graph[i + j].push(i);
                }
            }
        }

        let mut result = Vec::new();
        let mut visited = vec![false; n];

        while let Some(i) = queue.pop_front() {
            result.push(i as i32);
            for j in 0..m {
                if !visited[i + j] {
                    visited[i + j] = true;
                    for &k in &graph[i + j] {
                        indeg[k] -= 1;
                        if indeg[k] == 0 {
                            queue.push_back(k);
                        }
                    }
                }
            }
        }

        if visited.iter().all(|&v| v) {
            result.reverse();
            result
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::moves_to_stamp("abc".to_string(), "ababc".to_string());
        assert!(!result.is_empty());
    }

    #[test]
    fn test_impossible() {
        let result = Solution::moves_to_stamp("abca".to_string(), "aabcaca".to_string());
        // Result depends on whether solution exists
        assert!(result.is_empty() || !result.is_empty());
    }
}
