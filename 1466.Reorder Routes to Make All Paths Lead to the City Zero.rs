impl Solution {
    /// DFS on undirected graph counting edges that need reversal.
    ///
    /// # Intuition
    /// Build an undirected graph where each edge carries a cost: 1 if the
    /// original direction goes away from city 0 (needs reversal), 0 if it
    /// goes toward. DFS from city 0 sums the cost of all traversed edges.
    ///
    /// # Approach
    /// 1. Build adjacency list with edge costs (1 = original direction, 0 = reverse)
    /// 2. DFS from node 0, summing costs of edges pointing away from root
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the adjacency list and recursion
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n as usize];
        for edge in &connections {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[a].push((b, 1));
            graph[b].push((a, 0));
        }

        fn dfs(node: usize, parent: i32, graph: &[Vec<(usize, i32)>]) -> i32 {
            graph[node]
                .iter()
                .filter(|(next, _)| *next as i32 != parent)
                .map(|(next, cost)| cost + dfs(*next, node as i32, graph))
                .sum()
        }

        dfs(0, -1, &graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_chain() {
        assert_eq!(
            Solution::min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5],]
            ),
            3
        );
    }

    #[test]
    fn star_graph() {
        assert_eq!(
            Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4],]),
            2
        );
    }

    #[test]
    fn all_toward_zero() {
        assert_eq!(Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0);
    }
}
