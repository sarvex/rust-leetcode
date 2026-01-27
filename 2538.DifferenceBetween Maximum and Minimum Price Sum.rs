impl Solution {
    /// Finds the maximum cost amongst all possible root choices in a tree.
    ///
    /// # Intuition
    /// For any root r, the minimum path sum is just price[r] (single node path).
    /// The maximum path sum is the longest path from r to any other node.
    /// So cost(r) = max_path_from_r - price[r].
    ///
    /// # Approach
    /// Use rerooting technique:
    /// 1. First DFS: compute down[v] = longest path from v into its subtree
    /// 2. Second DFS: reroot to compute longest path from each node to any other node
    /// 3. Answer = max over all v of (longest[v] - price[v])
    ///
    /// # Complexity
    /// - Time: O(n) — two DFS traversals
    /// - Space: O(n) — adjacency list, down array, and recursion stack
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        if n == 1 {
            return 0;
        }

        let price: Vec<i64> = price.into_iter().map(|p| p as i64).collect();

        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut down = vec![0i64; n];
        Self::dfs_down(0, usize::MAX, &adj, &price, &mut down);

        let mut ans = 0i64;
        Self::dfs_reroot(0, usize::MAX, 0, &adj, &price, &down, &mut ans);
        ans
    }

    fn dfs_down(v: usize, parent: usize, adj: &[Vec<usize>], price: &[i64], down: &mut [i64]) {
        down[v] = price[v];
        for &u in &adj[v] {
            if u != parent {
                Self::dfs_down(u, v, adj, price, down);
                down[v] = down[v].max(price[v] + down[u]);
            }
        }
    }

    fn dfs_reroot(
        v: usize,
        parent: usize,
        up_contrib: i64,
        adj: &[Vec<usize>],
        price: &[i64],
        down: &[i64],
        ans: &mut i64,
    ) {
        let longest_v = down[v].max(price[v] + up_contrib);
        *ans = (*ans).max(longest_v - price[v]);

        let children: Vec<usize> = adj[v].iter().copied().filter(|&u| u != parent).collect();
        if children.is_empty() {
            return;
        }

        let child_downs: Vec<i64> = children.iter().map(|&c| down[c]).collect();
        let k = children.len();

        let mut prefix_max = vec![0i64; k + 1];
        let mut suffix_max = vec![0i64; k + 1];

        for i in 0..k {
            prefix_max[i + 1] = prefix_max[i].max(child_downs[i]);
        }
        for i in (0..k).rev() {
            suffix_max[i] = suffix_max[i + 1].max(child_downs[i]);
        }

        for (i, &c) in children.iter().enumerate() {
            let max_sibling = prefix_max[i].max(suffix_max[i + 1]);
            let child_up = price[v] + up_contrib.max(max_sibling);
            Self::dfs_reroot(c, v, child_up, adj, price, down, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]];
        assert_eq!(Solution::max_output(6, edges, vec![9, 8, 7, 6, 10, 5]), 24);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::max_output(3, edges, vec![1, 1, 1]), 2);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::max_output(1, vec![], vec![100]), 0);
    }

    #[test]
    fn test_two_nodes() {
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::max_output(2, edges, vec![5, 10]), 10);
    }

    #[test]
    fn test_linear_tree() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::max_output(4, edges, vec![1, 2, 3, 4]), 9);
    }

    #[test]
    fn test_star_topology() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        assert_eq!(Solution::max_output(5, edges, vec![1, 5, 3, 4, 2]), 6);
    }

    #[test]
    fn test_complex_tree() {
        let edges = vec![
            vec![1, 7],
            vec![2, 3],
            vec![4, 0],
            vec![5, 7],
            vec![6, 3],
            vec![3, 0],
            vec![0, 7],
        ];
        assert_eq!(
            Solution::max_output(8, edges, vec![4, 5, 6, 2, 2, 7, 7, 8]),
            21
        );
    }
}
