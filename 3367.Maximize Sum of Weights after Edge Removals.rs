impl Solution {
    /// Tree DP with greedy edge selection based on marginal gain
    ///
    /// # Intuition
    /// For each node, we must decide which edges to keep while respecting the degree
    /// constraint k. The key insight is that keeping an edge has a "gain" compared to
    /// not keeping it, and we should greedily select edges with highest positive gains.
    ///
    /// # Approach
    /// Root the tree at node 0. For each node u, compute two DP values:
    /// - dp0: max weight sum in subtree when edge to parent is NOT kept (can use k child edges)
    /// - dp1: max weight sum in subtree when edge to parent IS kept (can use k-1 child edges)
    ///
    /// For each child v with edge weight w:
    /// - If we keep edge (u,v): we gain w + dp1[v] (child contributes its "parent kept" value)
    /// - If we skip edge (u,v): we gain dp0[v] (child contributes its "parent not kept" value)
    /// - Marginal gain = w + dp1[v] - dp0[v]
    ///
    /// Sort gains descending and take top-k (or k-1) positive gains to maximize the sum.
    ///
    /// # Complexity
    /// - Time: O(n log n) - DFS visits each node once, sorting gains at each node
    /// - Space: O(n) - adjacency list and recursion stack
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let n = edges.len() + 1;
        let k = k as usize;

        let adj = edges.iter().fold(vec![vec![]; n], |mut adj, e| {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
            adj[u].push((v, w));
            adj[v].push((u, w));
            adj
        });

        Self::dfs(0, usize::MAX, &adj, k).0
    }

    fn dfs(u: usize, parent: usize, adj: &[Vec<(usize, i64)>], k: usize) -> (i64, i64) {
        let (base_sum, mut gains): (i64, Vec<i64>) = adj[u]
            .iter()
            .filter(|&&(v, _)| v != parent)
            .map(|&(v, w)| {
                let (child_dp0, child_dp1) = Self::dfs(v, u, adj, k);
                (child_dp0, w + child_dp1 - child_dp0)
            })
            .fold((0, Vec::new()), |(sum, mut gains), (dp0, gain)| {
                if gain > 0 {
                    gains.push(gain);
                }
                (sum + dp0, gains)
            });

        gains.sort_unstable_by(|a, b| b.cmp(a));

        let dp0 = base_sum + gains.iter().take(k).sum::<i64>();
        let dp1 = base_sum + gains.iter().take(k.saturating_sub(1)).sum::<i64>();

        (dp0, dp1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximize_sum_of_weights(
                vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 12], vec![2, 4, 6]],
                2,
            ),
            22,
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::maximize_sum_of_weights(
                vec![
                    vec![0, 1, 5],
                    vec![1, 2, 10],
                    vec![0, 3, 15],
                    vec![3, 4, 20],
                    vec![3, 5, 5],
                    vec![0, 6, 10],
                ],
                3,
            ),
            65,
        );
    }
}
