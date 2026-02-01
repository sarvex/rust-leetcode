impl Solution {
    /// Computes the latest marking time for each possible start node.
    ///
    /// # Intuition
    /// Marking time from a start node to any other node equals the sum of per-node delays
    /// along the unique path, excluding the start node itself. Since the graph is a tree,
    /// the path is unique and the time is deterministic.
    ///
    /// # Approach
    /// - Build an adjacency list and root the tree at node `0`.
    /// - Post-order: compute the top two child contributions for each node.
    /// - Pre-order: combine the parent contribution with the top two child values to
    ///   reroot in one scan per node.
    /// - The answer for each node is the best contribution reaching it.
    /// - Use iterative traversals to avoid recursion depth limits.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut adjacency = vec![Vec::new(); n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency[u].push(v);
            adjacency[v].push(u);
        }

        let delay = |node: usize| if node % 2 == 0 { 2 } else { 1 };

        let mut parent = vec![usize::MAX; n];
        let mut order = Vec::with_capacity(n);
        let mut stack = Vec::with_capacity(n);
        parent[0] = 0;
        stack.push(0);
        while let Some(node) = stack.pop() {
            order.push(node);
            for &next in &adjacency[node] {
                if next == parent[node] {
                    continue;
                }
                parent[next] = node;
                stack.push(next);
            }
        }

        let mut maxes = vec![(0i32, 0i32); n];
        let mut subtree_result = vec![0i32; n];
        for &node in order.iter().rev() {
            let mut best1 = 0i32;
            let mut best2 = 0i32;
            for &next in &adjacency[node] {
                if parent[next] != node {
                    continue;
                }
                let candidate = delay(next) + maxes[next].0;
                subtree_result[next] = candidate;
                if candidate > best1 {
                    best2 = best1;
                    best1 = candidate;
                } else if candidate > best2 {
                    best2 = candidate;
                }
            }
            maxes[node] = (best1, best2);
        }

        let mut parent_time = vec![0i32; n];
        let mut result = vec![0i32; n];
        for &node in &order {
            let (mut max1, mut max2) = maxes[node];
            let from_parent = parent_time[node];
            if from_parent > max1 {
                max2 = max1;
                max1 = from_parent;
            } else if from_parent > max2 {
                max2 = from_parent;
            }

            result[node] = max1;
            for &next in &adjacency[node] {
                if parent[next] != node {
                    continue;
                }
                let best_excluding = if subtree_result[next] == max1 {
                    max2
                } else {
                    max1
                };
                parent_time[next] = best_excluding + delay(node);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let result = Solution::time_taken(edges);
        assert_eq!(result, vec![2, 4, 3]);
    }

    #[test]
    fn test_example_two() {
        let edges = vec![vec![0, 1]];
        let result = Solution::time_taken(edges);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_example_three() {
        let edges = vec![vec![2, 4], vec![0, 1], vec![2, 3], vec![0, 2]];
        let result = Solution::time_taken(edges);
        assert_eq!(result, vec![4, 6, 3, 5, 5]);
    }

    #[test]
    fn test_chain_three_nodes() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let result = Solution::time_taken(edges);
        assert_eq!(result, vec![3, 2, 3]);
    }
}
