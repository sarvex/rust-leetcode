impl Solution {
    /// Count assignments of edge weights (1 or 2) on the root-to-deepest-node path with odd total cost.
    ///
    /// # Intuition
    /// For a path of length `d` (number of edges), there are `2^d` total assignments. Exactly half
    /// produce an odd sum, giving `2^(d-1)` valid assignments. This holds because fixing any single
    /// edge and toggling it changes parity, creating a bijection between odd and even assignments.
    ///
    /// # Approach
    /// 1. Build a CSR-style adjacency list (two flat arrays, zero heap fragmentation).
    /// 2. Iterative DFS to find maximum depth from node 1.
    /// 3. Return `2^(depth-1) mod (10^9 + 7)`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        const MOD: u64 = 1_000_000_007;

        let n = edges.len() + 1; // nodes labeled 1..=n

        // CSR adjacency: count degrees, then fill
        let mut deg = vec![0u32; n + 1];
        for e in &edges {
            deg[e[0] as usize] += 1;
            deg[e[1] as usize] += 1;
        }
        // prefix-sum to get start offsets (1-indexed nodes → offset array size n+2)
        let mut start = vec![0u32; n + 2];
        for i in 1..=n {
            start[i + 1] = start[i] + deg[i];
        }
        let total_edges = start[n + 1] as usize;
        let mut neighbors = vec![0u32; total_edges];
        let mut pos = start[1..=n].to_vec(); // current fill pointer per node
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            neighbors[pos[u - 1] as usize] = v as u32;
            pos[u - 1] += 1;
            neighbors[pos[v - 1] as usize] = u as u32;
            pos[v - 1] += 1;
        }

        // Iterative DFS from root 1, tracking (node, parent, depth)
        let mut max_depth = 0u32;
        let mut stack = Vec::with_capacity(n);
        stack.push((1u32, 0u32, 0u32)); // (node, parent, depth)

        while let Some((node, parent, depth)) = stack.pop() {
            if depth > max_depth {
                max_depth = depth;
            }
            let idx = node as usize;
            let lo = start[idx] as usize;
            let hi = start[idx + 1] as usize;
            for &nb in &neighbors[lo..hi] {
                if nb != parent {
                    stack.push((nb, node, depth + 1));
                }
            }
        }

        if max_depth == 0 {
            return 0;
        }

        // 2^(max_depth - 1) mod MOD via fast exponentiation
        let mut result = 1u64;
        let mut base = 2u64;
        let mut exp = (max_depth - 1) as u64;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_edge() {
        // Path length 1: only weight=1 gives odd cost → 2^0 = 1
        assert_eq!(Solution::assign_edge_weights(vec![vec![1, 2]]), 1);
    }

    #[test]
    fn test_two_levels() {
        // Max depth 2: path has 2 edges → 2^1 = 2 valid assignments
        assert_eq!(
            Solution::assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]]),
            2
        );
    }

    #[test]
    fn test_linear_chain() {
        // 1-2-3-4: depth=3 → 2^2 = 4
        assert_eq!(
            Solution::assign_edge_weights(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            4
        );
    }

    #[test]
    fn test_star_graph() {
        // All nodes at depth 1 → 2^0 = 1
        assert_eq!(
            Solution::assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5]]),
            1
        );
    }

    #[test]
    fn test_deep_chain() {
        // Chain of length 10: depth=10 → 2^9 = 512
        let edges: Vec<Vec<i32>> = (1..=10).map(|i| vec![i, i + 1]).collect();
        assert_eq!(Solution::assign_edge_weights(edges), 512);
    }

    #[test]
    fn test_large_depth() {
        // Chain of 100 nodes: depth=99 → 2^98 mod 10^9+7
        let edges: Vec<Vec<i32>> = (1..=99).map(|i| vec![i, i + 1]).collect();
        let expected = {
            const MOD: u64 = 1_000_000_007;
            let (mut r, mut b, mut e) = (1u64, 2u64, 98u64);
            while e > 0 {
                if e & 1 == 1 {
                    r = r * b % MOD;
                }
                b = b * b % MOD;
                e >>= 1;
            }
            r as i32
        };
        assert_eq!(Solution::assign_edge_weights(edges), expected);
    }
}
