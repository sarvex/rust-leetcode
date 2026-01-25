impl Solution {
    /// Subtree Inversion Sum - Tree DP with Distance Constraint
    ///
    /// # Intuition
    /// When we invert a subtree, all values in that subtree get multiplied by -1. The key insight
    /// is that each node's final value depends on how many times it's been "inverted" by ancestor
    /// inversions (parity). The constraint requires inverted ancestor-descendant pairs to be at
    /// least k edges apart.
    ///
    /// # Approach
    /// Use tree DP with memoization tracking two state variables:
    /// - `dist`: Distance from the nearest inverted ancestor (capped at k, since dist >= k means
    ///   we're free to invert)
    /// - `parity`: Current sign multiplier (0 = positive, 1 = negative) based on ancestor inversions
    ///
    /// For each node, we have two choices:
    /// 1. Don't invert: Keep current parity, increment distance for children
    /// 2. Invert (if dist >= k): Flip parity, set distance to 1 for children
    ///
    /// # Complexity
    /// - Time: O(n * k) where n is number of nodes and k is the distance constraint
    /// - Space: O(n * k) for memoization table
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;

        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut memo = vec![vec![vec![None; 2]; k + 1]; n];
        Self::dfs(0, n, k, 0, &adj, &nums, k, &mut memo)
    }

    fn dfs(
        node: usize,
        parent: usize,
        dist: usize,
        parity: usize,
        adj: &[Vec<usize>],
        nums: &[i32],
        k: usize,
        memo: &mut [Vec<Vec<Option<i64>>>],
    ) -> i64 {
        if let Some(val) = memo[node][dist][parity] {
            return val;
        }

        let sign = if parity == 0 { 1i64 } else { -1i64 };
        let next_dist = (dist + 1).min(k);

        let no_invert = nums[node] as i64 * sign
            + adj[node]
                .iter()
                .filter(|&&child| child != parent)
                .map(|&child| Self::dfs(child, node, next_dist, parity, adj, nums, k, memo))
                .sum::<i64>();

        let result = if dist >= k {
            let invert = nums[node] as i64 * (-sign)
                + adj[node]
                    .iter()
                    .filter(|&&child| child != parent)
                    .map(|&child| Self::dfs(child, node, 1, 1 - parity, adj, nums, k, memo))
                    .sum::<i64>();
            no_invert.max(invert)
        } else {
            no_invert
        };

        memo[node][dist][parity] = Some(result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];
        let nums = vec![4, -8, -6, 3, 7, -2, 5];
        let k = 2;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), 27);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let nums = vec![-1, 3, -2, 4, -5];
        let k = 2;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), 9);
    }

    #[test]
    fn test_example_3() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let nums = vec![0, -1, -2];
        let k = 3;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), 3);
    }

    #[test]
    fn test_single_edge() {
        let edges = vec![vec![0, 1]];
        let nums = vec![-5, -10];
        let k = 1;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), 15);
    }

    #[test]
    fn test_all_positive() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let nums = vec![1, 2, 3];
        let k = 1;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), 6);
    }

    #[test]
    fn test_k_equals_one() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let nums = vec![-1, -2, -3];
        let k = 1;
        assert_eq!(Solution::subtree_inversion_sum(edges, nums, k), 6);
    }
}
