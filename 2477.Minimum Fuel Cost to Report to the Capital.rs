impl Solution {
    /// Calculates minimum fuel to transport all representatives to the capital.
    ///
    /// # Intuition
    /// Each subtree's population must travel through its edge to the parent. The fuel
    /// cost for an edge is ceil(subtree_size / seats), since that many cars are needed.
    ///
    /// # Approach
    /// 1. Build an adjacency list from the road edges
    /// 2. DFS from node 0 (capital), computing subtree sizes bottom-up
    /// 3. For each non-root node, add ceil(subtree_size / seats) to total fuel
    ///
    /// # Complexity
    /// - Time: O(n) — single DFS traversal
    /// - Space: O(n) — adjacency list and recursion stack
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for road in &roads {
            let (a, b) = (road[0] as usize, road[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut ans = 0i64;
        Self::dfs(0, usize::MAX, &graph, &mut ans, seats);
        ans
    }

    fn dfs(node: usize, parent: usize, graph: &[Vec<usize>], ans: &mut i64, seats: i32) -> i32 {
        graph[node]
            .iter()
            .filter(|&&neighbor| neighbor != parent)
            .fold(1, |size, &neighbor| {
                let child_size = Self::dfs(neighbor, node, graph, ans, seats);
                *ans += ((child_size + seats - 1) / seats) as i64;
                size + child_size
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_tree() {
        let roads = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::minimum_fuel_cost(roads, 3), 2);
    }

    #[test]
    fn test_star_topology() {
        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        assert_eq!(Solution::minimum_fuel_cost(roads, 2), 7);
    }

    #[test]
    fn test_single_node() {
        let roads: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::minimum_fuel_cost(roads, 1), 0);
    }

    #[test]
    fn test_single_edge() {
        let roads = vec![vec![0, 1]];
        assert_eq!(Solution::minimum_fuel_cost(roads, 5), 1);
    }
}
