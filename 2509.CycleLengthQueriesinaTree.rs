impl Solution {
    /// Cycle Length Queries in a Tree using O(1) bit manipulation
    ///
    /// # Intuition
    /// The cycle length when adding an edge between nodes `a` and `b` equals
    /// `dist(a, LCA) + dist(b, LCA) + 1`. Using bit operations, we can compute
    /// this in constant time without iterative traversal.
    ///
    /// # Approach
    /// 1. Node level = floor(log2(v)) = 31 - leading_zeros(v) for u32
    /// 2. Bring both nodes to same level via right shift (count edges traversed)
    /// 3. XOR nodes at same level - bit length of XOR gives steps to LCA from each
    /// 4. Total = level_diff + 2 * xor_bit_length + 1
    ///
    /// # Complexity
    /// - Time: O(m) where m is number of queries, O(1) per query
    /// - Space: O(m) for the result vector
    pub fn cycle_length_queries(_n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|q| Self::cycle_len(q[0] as u32, q[1] as u32))
            .collect()
    }

    #[inline(always)]
    fn cycle_len(mut a: u32, mut b: u32) -> i32 {
        // Get levels: floor(log2(v)) via leading zeros
        let level_a = 31 - a.leading_zeros();
        let level_b = 31 - b.leading_zeros();

        // Bring deeper node up to same level, count edges traversed
        let depth_diff = if level_a > level_b {
            a >>= level_a - level_b;
            level_a - level_b
        } else {
            b >>= level_b - level_a;
            level_b - level_a
        };

        // XOR reveals divergence: bit length = steps from each node to LCA
        let xor = a ^ b;
        let to_lca = if xor == 0 {
            0
        } else {
            2 * (32 - xor.leading_zeros())
        };

        // +1 for the added edge
        (depth_diff + to_lca + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let queries = vec![vec![5, 3], vec![4, 7], vec![2, 3]];
        let result = Solution::cycle_length_queries(n, queries);
        assert_eq!(result, vec![4, 5, 3]);
    }

    #[test]
    fn test_example_2() {
        let n = 2;
        let queries = vec![vec![1, 2]];
        let result = Solution::cycle_length_queries(n, queries);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_same_level_nodes() {
        let n = 4;
        let queries = vec![vec![8, 15]];
        let result = Solution::cycle_length_queries(n, queries);
        assert_eq!(result, vec![7]);
    }

    #[test]
    fn test_parent_child_relationship() {
        let n = 3;
        let queries = vec![vec![2, 4]];
        let result = Solution::cycle_length_queries(n, queries);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_sibling_nodes() {
        let n = 3;
        let queries = vec![vec![4, 5]];
        let result = Solution::cycle_length_queries(n, queries);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_root_to_leaf() {
        let n = 4;
        let queries = vec![vec![1, 8]];
        let result = Solution::cycle_length_queries(n, queries);
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn test_large_values() {
        let n = 30;
        let queries = vec![vec![1 << 29, (1 << 29) + 1]];
        let result = Solution::cycle_length_queries(n, queries);
        // Siblings at deepest level: cycle = 1 + 1 + 1 = 3
        assert_eq!(result, vec![3]);
    }
}
