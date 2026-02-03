use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Graph {
    n: usize,
    adj: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    /// Build the directed adjacency list from the initial edges.
    ///
    /// # Intuition
    /// Dijkstra only needs outgoing edges, so store them per node.
    ///
    /// # Approach
    /// Allocate `n` adjacency lists and push each directed edge.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m)
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            let cost = edge[2];
            adj[from].push((to, cost));
        }

        Self { n, adj }
    }

    /// Add a directed edge with the given cost.
    ///
    /// # Intuition
    /// Store the edge immediately; queries will incorporate it.
    ///
    /// # Approach
    /// Push the edge into the adjacency list for `from`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1) extra
    fn add_edge(&mut self, edge: Vec<i32>) {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        let cost = edge[2];
        self.adj[from].push((to, cost));
    }

    /// Compute the minimum path cost via Dijkstra.
    ///
    /// # Intuition
    /// All edge weights are positive, so expanding the smallest frontier is optimal.
    ///
    /// # Approach
    /// Run Dijkstra from `node1` and early-exit once `node2` is finalized.
    ///
    /// # Complexity
    /// - Time: O((n + m) log n)
    /// - Space: O(n + m)
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let start = node1 as usize;
        let target = node2 as usize;
        if start == target {
            return 0;
        }

        let mut distances = vec![i32::MAX; self.n];
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        distances[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((current_cost, node))) = heap.pop() {
            if current_cost != distances[node] {
                continue;
            }
            if node == target {
                return current_cost;
            }
            for &(next_node, edge_cost) in &self.adj[node] {
                let next_cost = current_cost + edge_cost;
                if next_cost < distances[next_node] {
                    distances[next_node] = next_cost;
                    heap.push(Reverse((next_cost, next_node)));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut graph = Graph::new(
            4,
            vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
        );

        assert_eq!(graph.shortest_path(3, 2), 6);
        assert_eq!(graph.shortest_path(0, 3), -1);

        graph.add_edge(vec![1, 3, 4]);
        assert_eq!(graph.shortest_path(0, 3), 6);
    }

    #[test]
    fn self_path_is_zero() {
        let graph = Graph::new(1, vec![]);
        assert_eq!(graph.shortest_path(0, 0), 0);
    }

    #[test]
    fn unreachable_returns_negative_one() {
        let graph = Graph::new(3, vec![vec![0, 1, 7]]);
        assert_eq!(graph.shortest_path(2, 1), -1);
    }
}
