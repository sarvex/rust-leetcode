impl Solution {
    /// Binary search on minimum edge cost with Dijkstra validation.
    ///
    /// # Intuition
    /// The answer (maximum path score) must equal some edge cost on the optimal path.
    /// Binary search on this value: for a candidate min-cost, check if a valid path
    /// exists using only edges with cost >= min-cost and total cost <= k.
    ///
    /// # Approach
    /// 1. Filter edges to only those connecting online nodes
    /// 2. Binary search on sorted unique edge costs
    /// 3. For each candidate, run Dijkstra to find shortest path using valid edges
    /// 4. Valid if shortest path total <= k
    ///
    /// # Complexity
    /// - Time: O(m log m + m log(unique_costs) * (n + m) log n)
    /// - Space: O(n + m)
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = online.len();
        let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
        let mut all_costs: Vec<i64> = vec![];

        edges.iter().for_each(|edge| {
            let (u, v, cost) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            if online[u] && online[v] {
                adj[u].push((v, cost));
                all_costs.push(cost);
            }
        });

        if all_costs.is_empty() {
            return -1;
        }

        all_costs.sort_unstable();
        all_costs.dedup();

        let can_reach = |min_cost: i64| -> bool {
            let mut dist = vec![i64::MAX; n];
            dist[0] = 0;
            let mut heap = BinaryHeap::new();
            heap.push(Reverse((0i64, 0usize)));

            while let Some(Reverse((d, u))) = heap.pop() {
                if d > dist[u] {
                    continue;
                }
                if u == n - 1 {
                    return true;
                }
                adj[u].iter().for_each(|&(v, cost)| {
                    if cost >= min_cost {
                        let new_dist = d + cost;
                        if new_dist < dist[v] && new_dist <= k {
                            dist[v] = new_dist;
                            heap.push(Reverse((new_dist, v)));
                        }
                    }
                });
            }

            dist[n - 1] <= k
        };

        if !can_reach(0) {
            return -1;
        }

        let (mut lo, mut hi) = (0, all_costs.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if can_reach(all_costs[mid]) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        all_costs[lo] as i32
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
