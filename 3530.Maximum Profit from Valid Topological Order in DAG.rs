impl Solution {
    /// Bitmask DP with submask enumeration.
    ///
    /// # Intuition
    /// Since n ≤ 22, we can use bitmask DP where each bit represents whether a node
    /// has been processed. For maximum profit, we want higher-scoring nodes to appear
    /// later in the ordering (higher multiplier positions).
    ///
    /// # Approach
    /// 1. Build dependency mask for each node (which nodes must be processed before it)
    /// 2. Precompute for each mask which nodes become available (dependencies satisfied)
    /// 3. Use DP where dp[mask] = maximum profit achievable when nodes in mask are processed
    /// 4. Only iterate over available nodes using submask enumeration
    ///
    /// # Complexity
    /// - Time: O(2^n * n) preprocessing + O(3^n) DP via submask enumeration
    /// - Space: O(2^n)
    pub fn max_profit(n: i32, edges: Vec<Vec<i32>>, score: Vec<i32>) -> i64 {
        let n = n as usize;
        let m = 1usize << n;
        let full = (1u32 << n) - 1;

        // dep[v] = bitmask of nodes that must come before v
        let mut dep = vec![0u32; n];
        for e in &edges {
            dep[e[1] as usize] |= 1 << e[0];
        }

        // avail[mask] = bitmask of nodes available to pick when `mask` is processed
        let mut avail = vec![0u32; m];
        for mask in 0..m {
            let mask_u32 = mask as u32;
            let remaining = full ^ mask_u32;
            let mut bits = remaining;
            while bits != 0 {
                let i = bits.trailing_zeros() as usize;
                if dep[i] & mask_u32 == dep[i] {
                    avail[mask] |= 1 << i;
                }
                bits &= bits - 1;
            }
        }

        let mut dp = vec![i64::MIN; m];
        dp[0] = 0;

        for mask in 0..m - 1 {
            if dp[mask] == i64::MIN {
                continue;
            }
            let pos = (mask as u32).count_ones() as i64 + 1;
            let mut bits = avail[mask];
            while bits != 0 {
                let i = bits.trailing_zeros() as usize;
                let next = mask | (1 << i);
                dp[next] = dp[next].max(dp[mask] + score[i] as i64 * pos);
                bits &= bits - 1;
            }
        }

        dp[m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_linear_chain() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let score = vec![1, 2, 3];
        assert_eq!(Solution::max_profit(3, edges, score), 14);
    }

    #[test]
    fn test_no_edges() {
        let edges: Vec<Vec<i32>> = vec![];
        let score = vec![3, 1, 2];
        assert_eq!(Solution::max_profit(3, edges, score), 14);
    }

    #[test]
    fn test_single_node() {
        let edges: Vec<Vec<i32>> = vec![];
        let score = vec![5];
        assert_eq!(Solution::max_profit(1, edges, score), 5);
    }

    #[test]
    fn test_diamond_dag() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        let score = vec![1, 2, 3, 4];
        assert_eq!(Solution::max_profit(4, edges, score), 26);
    }
}
