use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Minimum cost path with edge reversals using transformed Dijkstra
    ///
    /// # Intuition
    /// Transform the directed graph into an undirected graph where reversed edges have
    /// double cost. This eliminates the need to track switch states, as Dijkstra naturally
    /// finds the optimal path using reversed edges only when beneficial.
    ///
    /// # Approach
    /// 1. Build bidirectional graph: add normal edges (cost w) and reversed edges (cost 2w)
    /// 2. Run standard Dijkstra from node 0 to node n-1
    /// 3. The algorithm automatically uses reversed edges when they provide shorter paths
    ///
    /// # Complexity
    /// - Time: O(E log V) - standard Dijkstra complexity
    /// - Space: O(V + E) for graph and distance array
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

        edges.iter().for_each(|edge| {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u].push((v, w));
            graph[v].push((u, 2 * w));
        });

        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i32, 0usize)));

        while let Some(Reverse((cost, u))) = heap.pop() {
            if cost > dist[u] {
                continue;
            }
            if u == n - 1 {
                return cost;
            }

            graph[u].iter().for_each(|&(v, w)| {
                let new_cost = cost + w;
                if new_cost < dist[v] {
                    dist[v] = new_cost;
                    heap.push(Reverse((new_cost, v)));
                }
            });
        }

        match dist[n - 1] {
            i32::MAX => -1,
            d => d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_with_reversal_cheaper() {
        let edges = vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]];
        assert_eq!(Solution::min_cost(4, edges), 5);
    }

    #[test]
    fn direct_path_optimal() {
        let edges = vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]];
        assert_eq!(Solution::min_cost(4, edges), 3);
    }
}
