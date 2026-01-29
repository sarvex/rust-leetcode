use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Finds shortest path visiting all nodes using BFS with bitmask state.
    ///
    /// # Intuition
    /// The state is `(current_node, visited_set)` where the visited set is a
    /// bitmask. BFS guarantees the shortest path in an unweighted graph.
    ///
    /// # Approach
    /// Initialize BFS from every node with its single-bit mask. Expand states
    /// by visiting neighbors and updating the bitmask. Return the step count
    /// when a state reaches the full mask `(1 << n) - 1`.
    ///
    /// # Complexity
    /// - Time: O(n * 2^n)
    /// - Space: O(n * 2^n) for visited states
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let full_mask = (1 << n) - 1;
        let mut visited = vec![vec![false; 1 << n]; n];
        let mut queue = VecDeque::new();

        for i in 0..n {
            let mask = 1 << i;
            queue.push_back((i, mask, 0));
            visited[i][mask] = true;
        }

        while let Some((node, mask, steps)) = queue.pop_front() {
            if mask == full_mask {
                return steps;
            }
            for &neighbor in &graph[node] {
                let nb = neighbor as usize;
                let new_mask = mask | (1 << nb);
                if !visited[nb][new_mask] {
                    visited[nb][new_mask] = true;
                    queue.push_back((nb, new_mask, steps + 1));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_graph() {
        assert_eq!(
            Solution::shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]),
            4
        );
    }

    #[test]
    fn test_cycle() {
        assert_eq!(
            Solution::shortest_path_length(vec![
                vec![1],
                vec![0, 2, 4],
                vec![1, 3, 4],
                vec![2],
                vec![1, 2]
            ]),
            4
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::shortest_path_length(vec![vec![]]), 0);
    }
}
