
impl Solution {
    /// Checks if people can be split into two groups where no dislikes are in the same group.
    ///
    /// # Intuition
    /// This is a graph 2-coloring (bipartite check) problem. Build a dislike
    /// graph and verify it is bipartite.
    ///
    /// # Approach
    /// Build an adjacency list from dislikes. DFS color each component with
    /// two colors. If a neighbor has the same color, return false.
    ///
    /// # Complexity
    /// - Time: O(n + e) where e is number of dislikes
    /// - Space: O(n + e)
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        // Pre-compute capacities for each node's adjacency list
        let mut degrees = vec![0usize; n + 1];
        for d in &dislikes {
            degrees[d[0] as usize] += 1;
            degrees[d[1] as usize] += 1;
        }

        let mut graph: Vec<Vec<usize>> = (0..=n).map(|i| Vec::with_capacity(degrees[i])).collect();

        for d in &dislikes {
            let (u, v) = (d[0] as usize, d[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut color = vec![0u8; n + 1];

        fn dfs(node: usize, c: u8, graph: &[Vec<usize>], color: &mut [u8]) -> bool {
            color[node] = c;
            graph[node].iter().all(|&nb| {
                if color[nb] == c {
                    false
                } else {
                    color[nb] != 0 || dfs(nb, 3 - c, graph, color)
                }
            })
        }

        (1..=n).all(|i| color[i] != 0 || dfs(i, 1, &graph, &mut color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible() {
        assert!(Solution::possible_bipartition(
            4,
            vec![vec![1, 2], vec![1, 3], vec![2, 4]],
        ));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::possible_bipartition(
            3,
            vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        ));
    }

    #[test]
    fn test_no_dislikes() {
        assert!(Solution::possible_bipartition(5, vec![]));
    }
}
