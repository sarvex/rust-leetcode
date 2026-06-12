impl Solution {
    /// Uses Binary Lifting LCA with combinatorial path length analysis.
    ///
    /// # Intuition
    /// For a path with k edges where each edge can have weight 1 or 2, the total
    /// cost is odd iff an odd number of edges have weight 1. The count of ways
    /// to choose an odd subset from k elements is C(k,1) + C(k,3) + ... = 2^(k-1).
    ///
    /// # Approach
    /// 1. Build tree adjacency list and compute depths via iterative DFS from root
    /// 2. Precompute binary lifting table with row-major layout [j][i] for sequential
    ///    inner-loop access (cache-friendly: inner loop walks contiguous memory)
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
        let size = n + 1;

        // Build adjacency list
        let mut adj = vec![Vec::with_capacity(4); size];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        // parent[j][i] = 2^j-th ancestor of node i
        // Row-major by j: inner preprocessing loop over i is sequential (cache-friendly)
        let mut parent = vec![0u32; LOG * size];
        let mut depth = vec![0u32; size];

        // Iterative DFS to fill depth and parent[0][i] (direct parent)
        let mut stack = Vec::with_capacity(n);
        stack.push((1usize, 0usize));
        while let Some((node, par)) = stack.pop() {
            parent[node] = par as u32; // parent[0 * size + node]
            for &next in &adj[node] {
                if next != par {
                    depth[next] = depth[node] + 1;
                    stack.push((next, node));
                }
            }
        }

        // Binary lifting: parent[j * size + i] = parent[(j-1) * size + parent[(j-1) * size + i]]
        for j in 1..LOG {
            let (lo, hi) = parent.split_at_mut(j * size);
            let prev = &lo[(j - 1) * size..j * size];
            let curr = &mut hi[..size];
            for i in 0..size {
                curr[i] = prev[prev[i] as usize];
            }
        }

        // Precompute powers of 2: pow2[k] = 2^k mod MOD
        let pow2: Vec<u64> = (0..=n)
            .scan(1u64, |acc, _| {
                let val = *acc;
                *acc = (*acc << 1) % MOD;
                Some(val)
            })
            .collect();

        // LCA via binary lifting
        let lca = |mut u: usize, mut v: usize| -> usize {
            // Bring u and v to the same depth
            if depth[u] < depth[v] {
                std::mem::swap(&mut u, &mut v);
            }
            // Lift u by depth[u] - depth[v]
            let diff = depth[u] - depth[v];
            for j in 0..LOG {
                if (diff >> j) & 1 == 1 {
                    u = parent[j * size + u] as usize;
                }
            }
            if u == v {
                return u;
            }
            // Lift both until divergence
            for j in (0..LOG).rev() {
                let pu = parent[j * size + u] as usize;
                let pv = parent[j * size + v] as usize;
                if pu != pv {
                    u = pu;
                    v = pv;
                }
            }
            parent[u] as usize // parent[0 * size + u]
        };

        // Answer each query
        queries
            .iter()
            .map(|q| {
                let (u, v) = (q[0] as usize, q[1] as usize);
                let l = lca(u, v);
                let dist = (depth[u] + depth[v] - 2 * depth[l]) as usize;
                if dist == 0 {
                    0
                } else {
                    pow2[dist - 1] as i32
                }
            })
            .collect()
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
