impl Solution {
    /// Sum of interaction costs using per-group edge contribution
    ///
    /// # Intuition
    /// For each edge, count how many same-group node pairs cross it.
    /// Process each group independently, only visiting nodes in that group.
    ///
    /// # Approach
    /// 1. Build CSR adjacency list for cache efficiency
    /// 2. BFS to establish tree structure and processing order
    /// 3. For each group with 2+ nodes, propagate subtree counts leaf-to-root
    /// 4. Skip nodes not in current group using sparse iteration
    ///
    /// # Complexity
    /// - Time: O(n + sum of group sizes) amortized
    /// - Space: O(n) for tree structure and counts
    pub fn interaction_costs(n: i32, edges: Vec<Vec<i32>>, group: Vec<i32>) -> i64 {
        let n = n as usize;

        if n <= 1 {
            return 0;
        }

        // Build CSR adjacency list
        let mut degree = vec![0u32; n];
        for edge in &edges {
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }

        let mut adj_start = vec![0u32; n + 1];
        for i in 0..n {
            adj_start[i + 1] = adj_start[i] + degree[i];
        }

        let mut adj = vec![0u32; 2 * (n - 1)];
        let mut adj_pos = adj_start[..n].to_vec();

        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[adj_pos[u] as usize] = v as u32;
            adj_pos[u] += 1;
            adj[adj_pos[v] as usize] = u as u32;
            adj_pos[v] += 1;
        }

        // BFS to build parent array and processing order
        let mut parent = vec![u32::MAX; n];
        let mut order = Vec::with_capacity(n);

        parent[0] = 0;
        order.push(0u32);
        let mut head = 0;

        while head < order.len() {
            let u = order[head] as usize;
            head += 1;

            let start = adj_start[u] as usize;
            let end = adj_start[u + 1] as usize;
            for i in start..end {
                let v = adj[i] as usize;
                if parent[v] == u32::MAX {
                    parent[v] = u as u32;
                    order.push(v as u32);
                }
            }
        }

        parent[0] = u32::MAX;
        order.reverse();

        // Count nodes per group and collect node lists
        let mut group_size = [0i32; 21];
        for &g in &group {
            group_size[g as usize] += 1;
        }

        let mut group_nodes: [Vec<u32>; 21] = Default::default();
        for (i, &g) in group.iter().enumerate() {
            let gi = g as usize;
            if group_size[gi] >= 2 {
                group_nodes[gi].push(i as u32);
            }
        }

        let mut total_cost = 0i64;
        let mut cnt = vec![0i32; n];

        // Process each group independently
        for g in 1..21 {
            let tg = group_size[g];
            if tg < 2 {
                continue;
            }

            // Initialize counts for nodes in this group
            for &u in &group_nodes[g] {
                cnt[u as usize] = 1;
            }

            let mut g_cost = 0i64;

            // Propagate counts leaf-to-root
            for &u in &order {
                let u = u as usize;
                let c_u = cnt[u];
                if c_u == 0 {
                    continue;
                }

                let p = parent[u];
                if p != u32::MAX {
                    g_cost += c_u as i64 * (tg - c_u) as i64;
                    cnt[p as usize] += c_u;
                }

                cnt[u] = 0;
            }

            total_cost += g_cost;
        }

        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let group = vec![1, 1, 1];
        assert_eq!(Solution::total_sum_of_interaction_costs(n, edges, group), 4);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let group = vec![3, 2, 3];
        assert_eq!(Solution::total_sum_of_interaction_costs(n, edges, group), 2);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let group = vec![1, 1, 4, 4];
        assert_eq!(Solution::total_sum_of_interaction_costs(n, edges, group), 3);
    }

    #[test]
    fn test_example_4() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let group = vec![9, 8];
        assert_eq!(Solution::total_sum_of_interaction_costs(n, edges, group), 0);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges: Vec<Vec<i32>> = vec![];
        let group = vec![1];
        assert_eq!(Solution::total_sum_of_interaction_costs(n, edges, group), 0);
    }
}
