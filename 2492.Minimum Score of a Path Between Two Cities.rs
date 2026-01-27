impl Solution {
    /// Finds the minimum edge weight on any path between cities 1 and n.
    ///
    /// # Intuition
    /// Since we can revisit edges, the answer is the minimum weight of any edge
    /// in the connected component containing nodes 1 and n.
    ///
    /// # Approach
    /// 1. Build adjacency list from roads
    /// 2. DFS from node 1, tracking the minimum edge weight encountered
    ///
    /// # Complexity
    /// - Time: O(n + e) where e is the number of roads
    /// - Space: O(n + e) — adjacency list and visited array
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n + 1];
        for road in &roads {
            let (a, b, w) = (road[0] as usize, road[1] as usize, road[2]);
            graph[a].push((b, w));
            graph[b].push((a, w));
        }

        let mut visited = vec![false; n + 1];
        Self::dfs(1, &graph, &mut visited)
    }

    fn dfs(node: usize, graph: &[Vec<(usize, i32)>], visited: &mut [bool]) -> i32 {
        if visited[node] {
            return i32::MAX;
        }
        visited[node] = true;

        graph[node]
            .iter()
            .fold(i32::MAX, |min_score, &(neighbor, weight)| {
                min_score
                    .min(weight)
                    .min(Self::dfs(neighbor, graph, visited))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
        assert_eq!(Solution::min_score(4, roads), 5);
    }

    #[test]
    fn test_example_2() {
        let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
        assert_eq!(Solution::min_score(4, roads), 2);
    }

    #[test]
    fn test_single_road() {
        let roads = vec![vec![1, 2, 10]];
        assert_eq!(Solution::min_score(2, roads), 10);
    }
}
