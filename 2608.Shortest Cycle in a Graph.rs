use std::collections::VecDeque;

impl Solution {
    /// BFS from every node to detect the shortest cycle.
    ///
    /// # Intuition
    /// In an undirected graph, an edge that connects two already discovered BFS nodes (and is not
    /// the parent edge) completes a cycle. Running BFS from each node guarantees we inspect a
    /// shortest cycle because every cycle includes at least one start node.
    ///
    /// # Approach
    /// 1. Build an adjacency list.
    /// 2. For each start node, run BFS to compute distances and parent links.
    /// 3. When an edge connects to a visited node that is not the parent, update the answer with
    ///    `dist[u] + dist[v] + 1`.
    /// 4. Return the minimum length found, or -1 if none exist.
    ///
    /// # Complexity
    /// - Time: O(n * (n + m))
    /// - Space: O(n + m)
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let node_count = n as usize;
        let mut adjacency = vec![Vec::<usize>::new(); node_count];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adjacency[u].push(v);
            adjacency[v].push(u);
        }

        let mut best = i32::MAX;
        let mut dist = vec![-1; node_count];
        let mut parent = vec![usize::MAX; node_count];
        let mut queue = VecDeque::new();

        for start in 0..node_count {
            dist.fill(-1);
            parent.fill(usize::MAX);
            queue.clear();

            dist[start] = 0;
            queue.push_back(start);

            while let Some(node) = queue.pop_front() {
                let next_dist = dist[node] + 1;
                for &next in &adjacency[node] {
                    if dist[next] == -1 {
                        dist[next] = next_dist;
                        parent[next] = node;
                        queue.push_back(next);
                    } else if parent[node] != next {
                        let cycle_len = dist[node] + dist[next] + 1;
                        if cycle_len < best {
                            best = cycle_len;
                        }
                    }
                }
            }
        }

        if best == i32::MAX {
            -1
        } else {
            best
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 3],
        ];
        assert_eq!(Solution::find_shortest_cycle(n, edges), 3);
    }

    #[test]
    fn example_two() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2]];
        assert_eq!(Solution::find_shortest_cycle(n, edges), -1);
    }

    #[test]
    fn square_cycle() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 0]];
        assert_eq!(Solution::find_shortest_cycle(n, edges), 4);
    }

    #[test]
    fn triangle_with_tail() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::find_shortest_cycle(n, edges), 3);
    }

    #[test]
    fn single_edge() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::find_shortest_cycle(n, edges), -1);
    }
}
