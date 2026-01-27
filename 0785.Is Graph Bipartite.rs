impl Solution {
    /// Checks if an undirected graph is bipartite using DFS coloring.
    ///
    /// # Intuition
    /// A graph is bipartite if and only if it is 2-colorable — no edge
    /// connects two nodes of the same color.
    ///
    /// # Approach
    /// DFS from each unvisited node, assigning alternating colors. If a
    /// neighbor already has the same color, the graph is not bipartite.
    ///
    /// # Complexity
    /// - Time: O(V + E)
    /// - Space: O(V) for the color array and recursion stack
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut color = vec![0i8; n];

        fn dfs(node: usize, c: i8, graph: &[Vec<i32>], color: &mut [i8]) -> bool {
            color[node] = c;
            graph[node].iter().all(|&neighbor| {
                let nb = neighbor as usize;
                match color[nb] {
                    x if x == c => false,
                    0 => dfs(nb, -c, graph, color),
                    _ => true,
                }
            })
        }

        (0..n).all(|i| color[i] != 0 || dfs(i, 1, &graph, &mut color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bipartite() {
        assert!(Solution::is_bipartite(vec![
            vec![1, 3],
            vec![0, 2],
            vec![1, 3],
            vec![0, 2],
        ]));
    }

    #[test]
    fn test_not_bipartite() {
        assert!(!Solution::is_bipartite(vec![
            vec![1, 2, 3],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 2],
        ]));
    }

    #[test]
    fn test_disconnected() {
        assert!(Solution::is_bipartite(vec![vec![], vec![2], vec![1],]));
    }
}
