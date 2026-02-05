impl Solution {
    /// Counts the number of connected components (provinces) using DFS.
    ///
    /// # Intuition
    /// Each unvisited city starts a new DFS that marks all reachable cities,
    /// forming one province.
    ///
    /// # Approach
    /// 1. Iterate over cities; for each unvisited city, increment count and DFS.
    /// 2. DFS marks all directly and transitively connected cities as visited.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n) for the visited array + recursion
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];

        fn dfs(graph: &[Vec<i32>], visited: &mut [bool], i: usize) {
            visited[i] = true;
            for j in 0..graph.len() {
                if !visited[j] && graph[i][j] == 1 {
                    dfs(graph, visited, j);
                }
            }
        }

        let mut provinces = 0;
        for i in 0..n {
            if !visited[i] {
                provinces += 1;
                dfs(&is_connected, &mut visited, i);
            }
        }
        provinces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_provinces() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
    }

    #[test]
    fn test_three_provinces() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }

    #[test]
    fn test_single_city() {
        assert_eq!(Solution::find_circle_num(vec![vec![1]]), 1);
    }

    #[test]
    fn test_all_connected() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]),
            1
        );
    }

    #[test]
    fn test_transitive_connection() {
        // Cities 0-1 connected, 1-2 connected, so all in one province
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]]),
            1
        );
    }

    #[test]
    fn test_two_pairs() {
        // Cities 0-1 connected, 2-3 connected, forming two provinces
        assert_eq!(
            Solution::find_circle_num(vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1]
            ]),
            2
        );
    }
}
