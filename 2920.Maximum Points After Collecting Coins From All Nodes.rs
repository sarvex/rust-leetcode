impl Solution {
    /// Maximizes points with a tree DP keyed by the number of halvings on the path.
    ///
    /// # Intuition
    /// Using the second method halves every coin in the subtree, so a node's effective coins depend
    /// only on how many ancestors already halved the path. Since `coins[i] <= 10^4`, after 14
    /// halvings every value becomes zero, giving a small fixed number of states.
    ///
    /// # Approach
    /// - Build a capacity-aware adjacency list and strip parent links during iterative DFS.
    /// - Let `dp[node][h]` be the best score in `node`'s subtree when coins were halved `h` times.
    /// - Transition for each `h`:
    ///   - Take full coins: `(coin >> h) - k + sum(child dp[child][h])`
    ///   - Halve subtree: `(coin >> (h + 1)) + sum(child dp[child][min(h + 1, H)])`
    /// - The answer is `dp[0][0]`.
    ///
    /// # Complexity
    /// - Time: O(n * H), with `H = 15`
    /// - Space: O(n * H)
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        const MAX_HALVES: usize = 14;
        const STATES: usize = MAX_HALVES + 1;

        let n = coins.len();
        let mut degree = vec![0_usize; n];
        for edge in &edges {
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }
        let mut graph: Vec<Vec<usize>> = degree.iter().map(|&d| Vec::with_capacity(d)).collect();
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }

        let mut order = Vec::with_capacity(n);
        let mut stack: Vec<(usize, usize, bool)> = Vec::with_capacity(n);
        stack.push((0, usize::MAX, false));
        while let Some((node, parent, expanded)) = stack.pop() {
            if expanded {
                order.push(node);
                continue;
            }
            if parent != usize::MAX {
                if let Some(pos) = graph[node].iter().position(|&next| next == parent) {
                    graph[node].swap_remove(pos);
                }
            }
            stack.push((node, parent, true));
            for &child in &graph[node] {
                stack.push((child, node, false));
            }
        }

        let mut dp = vec![[0_i32; STATES]; n];

        for &node in &order {
            let mut sum_same = [0_i32; STATES];
            let mut sum_next = [0_i32; STATES];
            for &child in &graph[node] {
                let child_dp = &dp[child];
                for h in 0..MAX_HALVES {
                    sum_same[h] += child_dp[h];
                    sum_next[h] += child_dp[h + 1];
                }
                sum_same[MAX_HALVES] += child_dp[MAX_HALVES];
                sum_next[MAX_HALVES] += child_dp[MAX_HALVES];
            }

            let mut shifted = coins[node];
            for h in 0..STATES {
                let next_shifted = shifted >> 1;
                let take_full = shifted - k + sum_same[h];
                let take_half = next_shifted + sum_next[h];
                dp[node][h] = take_full.max(take_half);
                shifted = next_shifted;
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let coins = vec![10, 10, 3, 3];
        assert_eq!(Solution::maximum_points(edges, coins, 5), 11);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let coins = vec![8, 4, 4];
        assert_eq!(Solution::maximum_points(edges, coins, 0), 16);
    }

    #[test]
    fn test_high_k_prefers_halving() {
        let edges = vec![vec![0, 1], vec![0, 2]];
        let coins = vec![10, 1, 1];
        assert_eq!(Solution::maximum_points(edges, coins, 9), 5);
    }

    #[test]
    fn test_all_zero_coins() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let coins = vec![0, 0, 0];
        assert_eq!(Solution::maximum_points(edges, coins, 7), 0);
    }

    #[test]
    fn test_two_nodes() {
        let edges = vec![vec![0, 1]];
        let coins = vec![5, 1];
        assert_eq!(Solution::maximum_points(edges, coins, 2), 3);
    }
}
