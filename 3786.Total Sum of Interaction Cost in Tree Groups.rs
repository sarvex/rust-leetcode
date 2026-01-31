impl Solution {
    /// Sum of interaction costs using per-group edge contribution.
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
    pub fn total_sum_of_interaction_costs(n: i32, edges: Vec<Vec<i32>>, group: Vec<i32>) -> i64 {
        let n = n as usize;

        if n <= 1 {
            return 0;
        }

        let mut degree = vec![0u32; n];
        edges.iter().for_each(|e| {
            degree[e[0] as usize] += 1;
            degree[e[1] as usize] += 1;
        });

        let adj_start: Vec<u32> = std::iter::once(0u32)
            .chain(degree.iter().scan(0u32, |acc, &d| {
                *acc += d;
                Some(*acc)
            }))
            .collect();

        let mut adj = vec![0u32; 2 * (n - 1)];
        let mut adj_pos = adj_start[..n].to_vec();

        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[adj_pos[u] as usize] = v as u32;
            adj_pos[u] += 1;
            adj[adj_pos[v] as usize] = u as u32;
            adj_pos[v] += 1;
        }

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

        let mut group_size = [0i32; 21];
        for &g in &group {
            group_size[g as usize] += 1;
        }

        let mut group_nodes: [Vec<u32>; 21] = std::array::from_fn(|i| {
            let sz = group_size[i];
            if sz >= 2 {
                Vec::with_capacity(sz as usize)
            } else {
                Vec::new()
            }
        });
        for (i, &g) in group.iter().enumerate() {
            let gi = g as usize;
            if group_size[gi] >= 2 {
                group_nodes[gi].push(i as u32);
            }
        }

        let mut total_cost = 0i64;
        let mut cnt = vec![0i32; n];

        for g in 1..21 {
            let tg = group_size[g];
            if tg < 2 {
                continue;
            }

            for &u in &group_nodes[g] {
                cnt[u as usize] = 1;
            }

            let mut g_cost = 0i64;

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
    fn test_all_same_group_linear_tree() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let group = vec![1, 1, 1];
        assert_eq!(Solution::total_sum_of_interaction_costs(3, edges, group), 4);
    }

    #[test]
    fn test_two_groups_linear_tree() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let group = vec![3, 2, 3];
        assert_eq!(Solution::total_sum_of_interaction_costs(3, edges, group), 2);
    }

    #[test]
    fn test_star_tree_mixed_groups() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let group = vec![1, 1, 4, 4];
        assert_eq!(Solution::total_sum_of_interaction_costs(4, edges, group), 3);
    }

    #[test]
    fn test_no_same_group_pair() {
        let edges = vec![vec![0, 1]];
        let group = vec![9, 8];
        assert_eq!(Solution::total_sum_of_interaction_costs(2, edges, group), 0);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(
            Solution::total_sum_of_interaction_costs(1, vec![], vec![1]),
            0
        );
    }
}
