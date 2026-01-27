impl Solution {
    /// Cycle Length Queries in a Tree using O(1) bit manipulation.
    ///
    /// # Intuition
    /// The cycle length when adding an edge between nodes `a` and `b` equals
    /// `dist(a, LCA) + dist(b, LCA) + 1`. Using bit operations, we can compute
    /// this in constant time without iterative traversal.
    ///
    /// # Approach
    /// 1. Node level = floor(log2(v)) = 31 - leading_zeros(v) for u32
    /// 2. Bring both nodes to same level via right shift (count edges traversed)
    /// 3. XOR nodes at same level — bit length of XOR gives steps to LCA from each
    /// 4. Total = level_diff + 2 × xor_bit_length + 1
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

    #[inline]
    fn cycle_len(mut a: u32, mut b: u32) -> i32 {
        let level_a = 31 - a.leading_zeros();
        let level_b = 31 - b.leading_zeros();

        let depth_diff = if level_a > level_b {
            a >>= level_a - level_b;
            level_a - level_b
        } else {
            b >>= level_b - level_a;
            level_b - level_a
        };

        let xor = a ^ b;
        let to_lca = if xor == 0 {
            0
        } else {
            2 * (32 - xor.leading_zeros())
        };

        (depth_diff + to_lca + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let queries = vec![vec![5, 3], vec![4, 7], vec![2, 3]];
        assert_eq!(Solution::cycle_length_queries(3, queries), vec![4, 5, 3]);
    }

    #[test]
    fn test_example_2() {
        let queries = vec![vec![1, 2]];
        assert_eq!(Solution::cycle_length_queries(2, queries), vec![2]);
    }

    #[test]
    fn test_same_level_nodes() {
        let queries = vec![vec![8, 15]];
        assert_eq!(Solution::cycle_length_queries(4, queries), vec![7]);
    }

    #[test]
    fn test_parent_child() {
        let queries = vec![vec![2, 4]];
        assert_eq!(Solution::cycle_length_queries(3, queries), vec![2]);
    }

    #[test]
    fn test_siblings() {
        let queries = vec![vec![4, 5]];
        assert_eq!(Solution::cycle_length_queries(3, queries), vec![3]);
    }

    #[test]
    fn test_root_to_leaf() {
        let queries = vec![vec![1, 8]];
        assert_eq!(Solution::cycle_length_queries(4, queries), vec![4]);
    }

    #[test]
    fn test_large_values() {
        let queries = vec![vec![1 << 29, (1 << 29) + 1]];
        assert_eq!(Solution::cycle_length_queries(30, queries), vec![3]);
    }
}
