impl Solution {
    /// Find the maximum cost amongst all possible root choices in a tree.
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
    /// - Time: O(n) - two DFS traversals
    /// - Space: O(n) - adjacency list, down array, and recursion stack
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        if n == 1 {
            return 0;
        }

        let price: Vec<i64> = price.into_iter().map(|p| p as i64).collect();

        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        // down[v] = longest path sum from v into its subtree (includes price[v])
        let mut down = vec![0i64; n];
        Self::dfs_down(0, usize::MAX, &adj, &price, &mut down);

        // Reroot to find max(longest_path_from_v - price[v])
        let mut ans = 0i64;
        Self::dfs_reroot(0, usize::MAX, 0, &adj, &price, &down, &mut ans);

        ans
    }

    /// Compute down[v] = longest path from v into subtree (when rooted at 0)
    fn dfs_down(v: usize, parent: usize, adj: &[Vec<usize>], price: &[i64], down: &mut [i64]) {
        down[v] = price[v];
        for &u in &adj[v] {
            if u == parent {
                continue;
            }
            Self::dfs_down(u, v, adj, price, down);
            down[v] = down[v].max(price[v] + down[u]);
        }
    }

    /// Reroot to compute answer for each node
    /// up_contrib = max path sum going "up" from v (through parent), NOT including v
    fn dfs_reroot(
        v: usize,
        parent: usize,
        up_contrib: i64,
        adj: &[Vec<usize>],
        price: &[i64],
        down: &[i64],
        ans: &mut i64,
    ) {
        // longest[v] = max(down[v], price[v] + up_contrib)
        let longest_v = down[v].max(price[v] + up_contrib);

        // Update answer: cost = longest[v] - price[v]
        *ans = (*ans).max(longest_v - price[v]);

        // Collect children
        let children: Vec<usize> = adj[v].iter().copied().filter(|&u| u != parent).collect();
        if children.is_empty() {
            return;
        }

        // Precompute prefix and suffix max of children's down values
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

        // Recurse to children
        for (i, &c) in children.iter().enumerate() {
            // For child c, its up_contrib is:
            // price[v] + max(up_contrib, best_sibling_down)
            let max_sibling = prefix_max[i].max(suffix_max[i + 1]);
            let child_up_contrib = price[v] + up_contrib.max(max_sibling);
            Self::dfs_reroot(c, v, child_up_contrib, adj, price, down, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]];
        let price = vec![9, 8, 7, 6, 10, 5];
        assert_eq!(Solution::max_output(n, edges, price), 24);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let price = vec![1, 1, 1];
        assert_eq!(Solution::max_output(n, edges, price), 2);
    }

    #[test]
    fn test_failing_case() {
        let n = 8;
        let edges = vec![
            vec![1, 7],
            vec![2, 3],
            vec![4, 0],
            vec![5, 7],
            vec![6, 3],
            vec![3, 0],
            vec![0, 7],
        ];
        let price = vec![4, 5, 6, 2, 2, 7, 7, 8];
        assert_eq!(Solution::max_output(n, edges, price), 21);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges: Vec<Vec<i32>> = vec![];
        let price = vec![100];
        assert_eq!(Solution::max_output(n, edges, price), 0);
    }

    #[test]
    fn test_two_nodes() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let price = vec![5, 10];
        // Root at 0: max path = 0->1 = 15, cost = 15 - 5 = 10
        // Root at 1: max path = 1->0 = 15, cost = 15 - 10 = 5
        assert_eq!(Solution::max_output(n, edges, price), 10);
    }

    #[test]
    fn test_linear_tree() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let price = vec![1, 2, 3, 4];
        // Longest path = 1+2+3+4 = 10
        // Best root is 0 (min price at endpoint), cost = 10 - 1 = 9
        assert_eq!(Solution::max_output(n, edges, price), 9);
    }

    #[test]
    fn test_star_topology() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let price = vec![1, 5, 3, 4, 2];
        // From node 0: longest = 0->1 = 1+5 = 6, cost = 6-1 = 5
        // From node 1: longest = 1->0->3 = 5+1+4 = 10, cost = 10-5 = 5
        // From node 2: longest = 2->0->1 = 3+1+5 = 9, cost = 9-3 = 6
        // From node 3: longest = 3->0->1 = 4+1+5 = 10, cost = 10-4 = 6
        // From node 4: longest = 4->0->1 = 2+1+5 = 8, cost = 8-2 = 6
        // Max cost = 6
        assert_eq!(Solution::max_output(n, edges, price), 6);
    }
}
