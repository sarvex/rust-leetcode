impl Solution {
    /// Single Dijkstra pass tracking minimum edge cost on optimal path.
    ///
    /// # Intuition
    /// Instead of binary search + multiple Dijkstra runs, track the minimum edge
    /// on each path during a single Dijkstra. State: (total_cost, min_edge, node).
    /// Optimize for maximum min_edge among all paths with total_cost <= k.
    ///
    /// # Approach
    /// 1. Filter edges to only those connecting online nodes
    /// 2. Run modified Dijkstra with state (cost, -min_edge, node)
    /// 3. For each node, track best (min_edge) for each reachable cost
    /// 4. Return maximum min_edge that reaches destination with cost <= k
    ///
    /// # Complexity
    /// - Time: O(m log m + (n + m) log n)
    /// - Space: O(n + m)
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = online.len();
        let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];

        for edge in edges.iter() {
            let (u, v, cost) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            if online[u] && online[v] {
                adj[u].push((v, cost));
            }
        }

        // State: (cost, -min_edge_on_path, node)
        // Negative min_edge for max-heap behavior on min_edge
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, i64::MIN, 0usize)));

        // best[node] = maximum min_edge we've seen reaching this node with cost <= k
        let mut best = vec![i64::MIN; n];
        best[0] = i64::MAX;

        let mut result = -1;

        while let Some(Reverse((cost, neg_min_edge, u))) = heap.pop() {
            let min_edge = -neg_min_edge;

            // If we've already found a better path to this node, skip
            if min_edge < best[u] {
                continue;
            }

            if u == n - 1 {
                result = result.max(min_edge as i32);
                continue;
            }

            for &(v, edge_cost) in adj[u].iter() {
                let new_cost = cost + edge_cost;
                if new_cost > k {
                    continue;
                }

                let new_min_edge = if u == 0 {
                    edge_cost
                } else {
                    min_edge.min(edge_cost)
                };

                // Only explore if this gives a better min_edge for node v
                if new_min_edge > best[v] {
                    best[v] = new_min_edge;
                    heap.push(Reverse((new_cost, -new_min_edge, v)));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1, 5], vec![1, 3, 10], vec![0, 2, 3], vec![2, 3, 4]];
        let online = vec![true, true, true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 10), 3);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![
            vec![0, 1, 7],
            vec![1, 4, 5],
            vec![0, 2, 6],
            vec![2, 3, 6],
            vec![3, 4, 2],
            vec![2, 4, 6],
        ];
        let online = vec![true, true, true, false, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 12), 6);
    }

    #[test]
    fn test_no_valid_path() {
        let edges = vec![vec![0, 1, 5], vec![1, 2, 10]];
        let online = vec![true, false, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 100), -1);
    }

    #[test]
    fn test_direct_edge_within_budget() {
        let edges = vec![vec![0, 1, 5]];
        let online = vec![true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 5), 5);
    }

    #[test]
    fn test_direct_edge_exceeds_budget() {
        let edges = vec![vec![0, 1, 5]];
        let online = vec![true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 4), -1);
    }

    #[test]
    fn test_cost_exceeds_k() {
        let edges = vec![vec![0, 1, 5], vec![1, 2, 10]];
        let online = vec![true, true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 10), -1);
    }

    #[test]
    fn test_cost_within_k() {
        let edges = vec![vec![0, 1, 5], vec![1, 2, 10]];
        let online = vec![true, true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 15), 5);
    }

    #[test]
    fn test_multiple_paths_large_budget() {
        let edges = vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 2, 5]];
        let online = vec![true, true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 20), 10);
    }

    #[test]
    fn test_multiple_paths_small_budget() {
        let edges = vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 2, 5]];
        let online = vec![true, true, true];
        assert_eq!(Solution::find_max_path_score(edges, online, 5), 5);
    }
}
