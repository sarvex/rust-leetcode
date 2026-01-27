impl Solution {
    /// Maximum subgraph score for each node using tree DP with rerooting.
    ///
    /// # Intuition
    /// For each node, find the maximum score of any connected subgraph containing it.
    /// Use tree DP with rerooting: bottom-up accumulates scores, top-down propagates.
    ///
    /// # Approach
    /// 1. Build CSR-style adjacency list for cache efficiency
    /// 2. BFS to establish parent-child relationships and traversal order
    /// 3. Bottom-up: accumulate positive child contributions
    /// 4. Top-down: add parent's contribution excluding current subtree
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn max_subgraph_score(n: i32, edges: Vec<Vec<i32>>, good: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![if good[0] == 1 { 1 } else { -1 }];
        }

        let mut degree = vec![0u32; n];
        for e in &edges {
            degree[e[0] as usize] += 1;
            degree[e[1] as usize] += 1;
        }

        let mut head = vec![0u32; n + 1];
        for i in 0..n {
            head[i + 1] = head[i] + degree[i];
        }

        let mut adj = vec![0u32; 2 * (n - 1)];
        let mut pos = head[..n].to_vec();
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[pos[u] as usize] = v as u32;
            pos[u] += 1;
            adj[pos[v] as usize] = u as u32;
            pos[v] += 1;
        }

        let mut parent = vec![u32::MAX; n];
        let mut order = Vec::with_capacity(n);
        parent[0] = 0;
        order.push(0u32);
        let mut idx = 0;

        while idx < order.len() {
            let u = order[idx] as usize;
            idx += 1;
            for i in head[u]..head[u + 1] {
                let v = adj[i as usize] as usize;
                if parent[v] == u32::MAX {
                    parent[v] = u as u32;
                    order.push(v as u32);
                }
            }
        }

        let mut dp: Vec<i32> = good.iter().map(|&g| (g << 1) - 1).collect();

        for i in (1..n).rev() {
            let u = order[i] as usize;
            let p = parent[u] as usize;
            if dp[u] > 0 {
                dp[p] += dp[u];
            }
        }

        for i in 1..n {
            let u = order[i] as usize;
            let p = parent[u] as usize;
            let contrib = dp[p] - dp[u].max(0);
            if contrib > 0 {
                dp[u] += contrib;
            }
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_tree_mixed_good() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let good = vec![1, 0, 1];
        assert_eq!(Solution::max_subgraph_score(3, edges, good), vec![1, 1, 1]);
    }

    #[test]
    fn test_star_tree_with_subtree() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3], vec![3, 4]];
        let good = vec![0, 1, 0, 1, 1];
        assert_eq!(
            Solution::max_subgraph_score(5, edges, good),
            vec![2, 3, 2, 3, 3]
        );
    }

    #[test]
    fn test_all_bad_nodes() {
        let edges = vec![vec![0, 1]];
        let good = vec![0, 0];
        assert_eq!(Solution::max_subgraph_score(2, edges, good), vec![-1, -1]);
    }

    #[test]
    fn test_all_good_nodes() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let good = vec![1, 1, 1];
        assert_eq!(Solution::max_subgraph_score(3, edges, good), vec![3, 3, 3]);
    }

    #[test]
    fn test_single_good_center() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let good = vec![0, 1, 0];
        assert_eq!(Solution::max_subgraph_score(3, edges, good), vec![0, 1, 0]);
    }

    #[test]
    fn test_single_node_good() {
        assert_eq!(Solution::max_subgraph_score(1, vec![], vec![1]), vec![1]);
    }
}
