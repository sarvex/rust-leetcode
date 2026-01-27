impl Solution {
    /// Determines if a path exists between source and destination via DFS.
    ///
    /// # Intuition
    /// Build an adjacency list and perform depth-first search from the
    /// source node. If we reach the destination, a path exists.
    ///
    /// # Approach
    /// 1. Build an undirected adjacency list.
    /// 2. DFS from source, marking visited nodes.
    /// 3. Return true if destination is reached.
    ///
    /// # Complexity
    /// - Time: O(V + E)
    /// - Space: O(V + E)
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let (source, destination) = (source as usize, destination as usize);

        let mut graph = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut visited = vec![false; n];

        fn dfs(graph: &[Vec<usize>], visited: &mut [bool], node: usize, target: usize) -> bool {
            if node == target {
                return true;
            }
            visited[node] = true;
            graph[node]
                .iter()
                .any(|&next| !visited[next] && dfs(graph, visited, next, target))
        }

        dfs(&graph, &mut visited, source, destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_exists() {
        assert!(Solution::valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn test_no_path() {
        assert!(!Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }
}
