use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    /// Minimum cost path with edge reversals using Dijkstra with state tracking
    ///
    /// # Intuition
    /// Model as shortest path where state = (node, switches_used). At each node,
    /// we can traverse normal outgoing edges or reverse incoming edges (if switch
    /// not used) and traverse them at double cost.
    ///
    /// # Approach
    /// 1. Build adjacency list for outgoing edges (normal traversal)
    /// 2. Build reverse adjacency list for incoming edges (potential reversals)
    /// 3. Use Dijkstra with state (node, switches_used_set as sorted Vec)
    /// 4. From state (u, switches_used):
    ///    - Traverse normal outgoing edges with same switches_used
    ///    - If u's switch not used, reverse incoming edges and traverse at 2*cost
    ///
    /// # Complexity
    /// - Time: O(E * 2^S * log(V * 2^S)) where S is max switches used (worst case exponential)
    /// - Space: O(V * 2^S) for distance map
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        let mut reverse_graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

        edges.iter().for_each(|edge| {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u].push((v, w));
            reverse_graph[v].push((u, w));
        });

        let mut dist: HashMap<(usize, Vec<usize>), i32> = HashMap::new();
        let mut pq = BinaryHeap::new();

        let initial_switches = Vec::new();
        dist.insert((0, initial_switches.clone()), 0);
        pq.push(Reverse((0i32, 0usize, initial_switches)));

        while let Some(Reverse((cost, u, switches_used))) = pq.pop() {
            if u == n - 1 {
                return cost;
            }

            let switches_key = switches_used.clone();
            if let Some(&best_cost) = dist.get(&(u, switches_key)) {
                if cost > best_cost {
                    continue;
                }
            }

            let switch_used_at_u = switches_used.binary_search(&u).is_ok();

            // Traverse normal outgoing edges
            graph[u].iter().for_each(|&(v, w)| {
                let new_cost = cost + w;
                let state = (v, switches_used.clone());

                if new_cost < *dist.get(&state).unwrap_or(&i32::MAX) {
                    dist.insert(state.clone(), new_cost);
                    pq.push(Reverse((new_cost, v, state.1)));
                }
            });

            // Reverse incoming edges if switch not used at current node
            if !switch_used_at_u {
                let mut new_switches = switches_used.clone();
                new_switches.push(u);
                new_switches.sort_unstable();

                reverse_graph[u].iter().for_each(|&(v, w)| {
                    let new_cost = cost + 2 * w;
                    let state = (v, new_switches.clone());

                    if new_cost < *dist.get(&state).unwrap_or(&i32::MAX) {
                        dist.insert(state.clone(), new_cost);
                        pq.push(Reverse((new_cost, v, state.1)));
                    }
                });
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let edges = vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]];
        assert_eq!(Solution::min_cost(4, edges), 5);
    }

    #[test]
    fn example_2() {
        let edges = vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]];
        assert_eq!(Solution::min_cost(4, edges), 3);
    }
}
