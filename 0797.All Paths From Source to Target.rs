impl Solution {
    /// Finds all paths from node 0 to node n-1 in a DAG via DFS.
    ///
    /// # Intuition
    /// Since the graph is a DAG, every DFS path from source to target is
    /// valid without cycle detection.
    ///
    /// # Approach
    /// Backtracking DFS: push the current node onto the path, recurse into
    /// neighbors, and pop when backtracking. Collect complete paths when
    /// the target node is reached.
    ///
    /// # Complexity
    /// - Time: O(2^n * n) in worst case for all possible paths
    /// - Space: O(n) recursion depth
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(node: usize, graph: &[Vec<i32>], path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            path.push(node as i32);
            if node == graph.len() - 1 {
                result.push(path.clone());
            } else {
                for &next in &graph[node] {
                    dfs(next as usize, graph, path, result);
                }
            }
            path.pop();
        }

        let mut result = Vec::new();
        dfs(0, &graph, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(
            Solution::all_paths_source_target(graph),
            vec![vec![0, 1, 3], vec![0, 2, 3]]
        );
    }

    #[test]
    fn test_multiple_paths() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let result = Solution::all_paths_source_target(graph);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![]]),
            vec![vec![0]]
        );
    }
}
