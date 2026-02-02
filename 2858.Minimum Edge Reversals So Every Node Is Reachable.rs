impl Solution {
    /// Computes the minimum reversals needed for every possible starting node.
    ///
    /// # Intuition
    /// Rooting the underlying tree fixes a parent-to-child direction for each edge. An edge needs
    /// a reversal exactly when its original direction points toward the root instead of away from
    /// it.
    ///
    /// # Approach
    /// - Build an undirected adjacency list with a directional cost per traversal
    ///   (`0` if the edge already points away from the current node, `1` otherwise).
    /// - DFS from node `0` to sum those costs and obtain the answer for root `0`.
    /// - Reroot with another DFS: moving the root across one edge flips that edge's contribution,
    ///   changing the total by `+1` or `-1`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let node_count = n as usize;
        if node_count == 0 {
            return Vec::new();
        }

        let mut graph: Vec<Vec<(usize, i32)>> =
            (0..node_count).map(|_| Vec::with_capacity(2)).collect();
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            graph[from].push((to, 0));
            graph[to].push((from, 1));
        }

        let mut answers = vec![0_i32; node_count];
        let mut total_reversals = 0_i32;
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(node_count);
        stack.push((0, usize::MAX));
        while let Some((node, parent)) = stack.pop() {
            for &(next, cost) in &graph[node] {
                if next == parent {
                    continue;
                }
                total_reversals += cost;
                stack.push((next, node));
            }
        }
        answers[0] = total_reversals;

        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(node_count);
        stack.push((0, usize::MAX));
        while let Some((node, parent)) = stack.pop() {
            for &(next, cost) in &graph[node] {
                if next == parent {
                    continue;
                }
                let delta = if cost == 0 { 1 } else { -1 };
                answers[next] = answers[node] + delta;
                stack.push((next, node));
            }
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let n = 4;
        let edges = vec![vec![2, 0], vec![2, 1], vec![1, 3]];
        assert_eq!(Solution::min_edge_reversals(n, edges), vec![1, 1, 0, 2]);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![1, 2], vec![2, 0]];
        assert_eq!(Solution::min_edge_reversals(n, edges), vec![2, 0, 1]);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::min_edge_reversals(n, edges), vec![0]);
    }

    #[test]
    fn test_star_outward() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        assert_eq!(Solution::min_edge_reversals(n, edges), vec![0, 1, 1, 1]);
    }
}
