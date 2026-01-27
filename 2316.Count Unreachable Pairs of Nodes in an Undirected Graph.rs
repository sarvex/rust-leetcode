impl Solution {
    /// Counts unreachable pairs via connected component sizes.
    ///
    /// # Intuition
    /// Two nodes are unreachable if they belong to different connected components.
    /// The number of unreachable pairs equals the sum of products of each component
    /// size with the cumulative size of previously visited components.
    ///
    /// # Approach
    /// 1. Build adjacency list from edges
    /// 2. DFS to find connected component sizes
    /// 3. Accumulate pairs: for each component of size t, add t * (sum of prior sizes)
    ///
    /// # Complexity
    /// - Time: O(n + e) where e is the number of edges
    /// - Space: O(n + e) for adjacency list and visited array
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut visited = vec![false; n];
        let mut total_seen: i64 = 0;
        let mut pairs: i64 = 0;

        for u in 0..n {
            let component_size = Self::dfs(&graph, &mut visited, u);
            pairs += component_size * total_seen;
            total_seen += component_size;
        }

        pairs
    }

    fn dfs(graph: &[Vec<usize>], visited: &mut [bool], u: usize) -> i64 {
        if visited[u] {
            return 0;
        }
        visited[u] = true;
        graph[u]
            .iter()
            .fold(1, |count, &v| count + Self::dfs(graph, visited, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_components() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        assert_eq!(Solution::count_pairs(7, edges), 14);
    }

    #[test]
    fn test_fully_connected() {
        let edges = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
        assert_eq!(
            Solution::count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            0
        );
    }

    #[test]
    fn test_no_edges() {
        assert_eq!(Solution::count_pairs(4, vec![]), 6);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::count_pairs(1, vec![]), 0);
    }
}
