impl Solution {
    /// Uses Binary Lifting LCA with combinatorial path length analysis
    ///
    /// # Intuition
    /// For a path with k edges where each edge can have weight 1 or 2, the total
    /// cost is odd iff an odd number of edges have weight 1. The count of ways
    /// to choose an odd subset from k elements is C(k,1) + C(k,3) + ... = 2^(k-1).
    ///
    /// # Approach
    /// 1. Build tree adjacency list and compute depths via DFS from root
    /// 2. Precompute binary lifting table for O(log n) LCA queries
    /// 3. For each query, compute path length as depth[u] + depth[v] - 2*depth[LCA]
    /// 4. Return 2^(path_length - 1) mod (10^9 + 7), or 0 if path_length is 0
    ///
    /// # Complexity
    /// - Time: O((n + q) * log n)
    /// - Space: O(n * log n)
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: u64 = 1_000_000_007;
        const LOG: usize = 17;
        let n = edges.len() + 1;

        // Build adjacency list with preallocated capacity
        let mut adj = vec![Vec::with_capacity(4); n + 1];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        // Flat parent table for cache locality: parent[i * LOG + j] = 2^j-th ancestor of i
        let mut parent = vec![0u32; (n + 1) * LOG];
        let mut depth = vec![0u32; n + 1];

        // Iterative DFS
        let mut stack = Vec::with_capacity(n);
        stack.push((1usize, 0u32));

        while let Some((node, par)) = stack.pop() {
            parent[node * LOG] = par;
            for &next in &adj[node] {
                if next != par as usize {
                    depth[next] = depth[node] + 1;
                    stack.push((next, node as u32));
                }
            }
        }

        // Binary lifting preprocessing
        for j in 1..LOG {
            for i in 1..=n {
                let p = parent[i * LOG + j - 1] as usize;
                parent[i * LOG + j] = parent[p * LOG + j - 1];
            }
        }

        // Precompute powers of 2
        let mut pow2 = vec![1u64; n + 1];
        for i in 1..=n {
            pow2[i] = (pow2[i - 1] << 1) % MOD;
        }

        // Inline LCA computation
        let lca = |mut u: usize, mut v: usize| -> usize {
            if depth[u] < depth[v] {
                std::mem::swap(&mut u, &mut v);
            }

            let mut diff = depth[u] - depth[v];
            let mut j = 0;
            while diff > 0 {
                if diff & 1 == 1 {
                    u = parent[u * LOG + j] as usize;
                }
                diff >>= 1;
                j += 1;
            }

            if u == v {
                return u;
            }

            for j in (0..LOG).rev() {
                if parent[u * LOG + j] != parent[v * LOG + j] {
                    u = parent[u * LOG + j] as usize;
                    v = parent[v * LOG + j] as usize;
                }
            }

            parent[u * LOG] as usize
        };

        // Process queries
        let mut result = Vec::with_capacity(queries.len());
        for q in &queries {
            let (u, v) = (q[0] as usize, q[1] as usize);
            let l = lca(u, v);
            let dist = (depth[u] + depth[v] - 2 * depth[l]) as usize;
            result.push(if dist == 0 { 0 } else { pow2[dist - 1] as i32 });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![1, 2]];
        let queries = vec![vec![1, 1], vec![1, 2]];
        assert_eq!(Solution::assign_edge_weights(edges, queries), vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]];
        let queries = vec![vec![1, 4], vec![3, 4], vec![2, 5]];
        assert_eq!(Solution::assign_edge_weights(edges, queries), vec![2, 1, 4]);
    }

    #[test]
    fn test_single_path() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        let queries = vec![vec![1, 4]];
        assert_eq!(Solution::assign_edge_weights(edges, queries), vec![4]);
    }

    #[test]
    fn test_same_node() {
        let edges = vec![vec![1, 2], vec![2, 3]];
        let queries = vec![vec![2, 2]];
        assert_eq!(Solution::assign_edge_weights(edges, queries), vec![0]);
    }
}
