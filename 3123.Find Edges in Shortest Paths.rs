use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    weight: i64,
}

fn dijkstra(start: usize, adjacency: &[Vec<(usize, i64)>]) -> Vec<i64> {
    let node_count = adjacency.len();
    let inf = i64::MAX / 4;
    let mut dist = vec![inf; node_count];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push((Reverse(0i64), start));

    while let Some((Reverse(current), node)) = heap.pop() {
        if current != dist[node] {
            continue;
        }
        for &(next_node, weight) in &adjacency[node] {
            let next_cost = current + weight;
            if next_cost < dist[next_node] {
                dist[next_node] = next_cost;
                heap.push((Reverse(next_cost), next_node));
            }
        }
    }

    dist
}

impl Solution {
    /// Marks edges that appear in any shortest path from 0 to n - 1.
    ///
    /// # Intuition
    /// An edge is on a shortest path iff it can bridge a shortest prefix from 0
    /// with a shortest suffix to n - 1.
    ///
    /// # Approach
    /// - Run Dijkstra from 0 and from n - 1 to get two distance arrays.
    /// - For each edge (u, v, w), check if dist0[u] + w + distN[v] equals the
    ///   shortest distance, or the symmetric orientation.
    ///
    /// # Complexity
    /// - Time: O((n + m) log n)
    /// - Space: O(n + m)
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let node_count = n as usize;
        let mut adjacency = vec![Vec::<(usize, i64)>::new(); node_count];
        let mut edge_list = Vec::with_capacity(edges.len());

        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            let weight = edge[2] as i64;
            edge_list.push(Edge { from, to, weight });
            adjacency[from].push((to, weight));
            adjacency[to].push((from, weight));
        }

        if node_count == 0 {
            return Vec::new();
        }

        let dist_from_start = dijkstra(0, &adjacency);
        let dist_from_end = dijkstra(node_count - 1, &adjacency);
        let shortest = dist_from_start[node_count - 1];
        let inf = i64::MAX / 4;

        if shortest >= inf {
            return vec![false; edge_list.len()];
        }

        edge_list
            .iter()
            .map(|edge| {
                let forward = dist_from_start[edge.from] + edge.weight + dist_from_end[edge.to];
                let backward = dist_from_start[edge.to] + edge.weight + dist_from_end[edge.from];
                forward == shortest || backward == shortest
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 6;
        let edges = vec![
            vec![0, 1, 4],
            vec![0, 2, 1],
            vec![1, 3, 2],
            vec![1, 4, 3],
            vec![1, 5, 1],
            vec![2, 3, 1],
            vec![3, 5, 3],
            vec![4, 5, 2],
        ];
        let expected = vec![true, true, true, false, true, true, true, false];
        assert_eq!(Solution::find_answer(n, edges), expected);
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let edges = vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]];
        let expected = vec![true, false, false, true];
        assert_eq!(Solution::find_answer(n, edges), expected);
    }

    #[test]
    fn test_disconnected() {
        let n = 3;
        let edges = vec![vec![0, 1, 5]];
        let expected = vec![false];
        assert_eq!(Solution::find_answer(n, edges), expected);
    }

    #[test]
    fn test_direct_edge() {
        let n = 2;
        let edges = vec![vec![0, 1, 7]];
        let expected = vec![true];
        assert_eq!(Solution::find_answer(n, edges), expected);
    }
}
