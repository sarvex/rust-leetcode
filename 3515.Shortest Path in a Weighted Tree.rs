impl Solution {
    /// Uses Euler tour with Binary Indexed Tree for efficient path queries and updates
    ///
    /// # Intuition
    /// In a tree rooted at node 1, the distance from root to any node is the sum of edge
    /// weights along the unique path. When an edge weight changes, it affects the distance
    /// to all nodes in the subtree below that edge.
    ///
    /// # Approach
    /// 1. Build adjacency list and perform DFS to compute Euler tour (in/out times)
    /// 2. Store initial distances from root to each node during DFS
    /// 3. Use a Binary Indexed Tree (BIT) to handle range updates efficiently
    /// 4. For edge weight updates: the child node (with larger in_time) determines the
    ///    subtree to update - use BIT range update
    /// 5. For distance queries: return initial distance + BIT prefix sum at node's in-time
    ///
    /// # Complexity
    /// - Time: O((n + q) * log(n)) for building tree and processing queries
    /// - Space: O(n) for adjacency list, BIT, and Euler tour arrays
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        // Build adjacency list: (neighbor, edge_index)
        let mut adj: Vec<Vec<(u32, u32)>> = vec![vec![]; n + 1];
        let mut edge_weights: Vec<i32> = vec![0; n];

        for (idx, edge) in edges.iter().enumerate() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj[u].push((v as u32, idx as u32));
            adj[v].push((u as u32, idx as u32));
            edge_weights[idx] = edge[2];
        }

        // Euler tour arrays
        let mut in_time: Vec<u32> = vec![0; n + 1];
        let mut out_time: Vec<u32> = vec![0; n + 1];
        let mut dist: Vec<i64> = vec![0; n + 1];
        let mut parent: Vec<u32> = vec![0; n + 1];
        let mut parent_edge: Vec<u32> = vec![u32::MAX; n + 1];

        // Iterative DFS
        let mut timer = 0u32;
        let mut stack: Vec<(u32, u32, i64, usize)> = Vec::with_capacity(n);
        stack.push((1, 0, 0, 0));

        while let Some(&(node, par, d, idx)) = stack.last() {
            let node_usize = node as usize;

            if idx == 0 {
                in_time[node_usize] = timer;
                timer += 1;
                dist[node_usize] = d;
                parent[node_usize] = par;
            }

            let adj_node = &adj[node_usize];
            let mut found = false;

            for i in idx..adj_node.len() {
                let (neighbor, edge_idx) = adj_node[i];
                if neighbor != par {
                    stack.last_mut().unwrap().3 = i + 1;
                    parent_edge[neighbor as usize] = edge_idx;
                    let weight = edge_weights[edge_idx as usize] as i64;
                    stack.push((neighbor, node, d + weight, 0));
                    found = true;
                    break;
                }
            }

            if !found {
                out_time[node_usize] = timer - 1;
                stack.pop();
            }
        }

        // Binary Indexed Tree
        let mut bit: Vec<i64> = vec![0; n + 2];

        #[inline(always)]
        fn update(bit: &mut [i64], mut i: usize, delta: i64) {
            i += 1;
            while i < bit.len() {
                bit[i] += delta;
                i += i & i.wrapping_neg();
            }
        }

        #[inline(always)]
        fn query(bit: &[i64], mut i: usize) -> i64 {
            i += 1;
            let mut sum = 0i64;
            while i > 0 {
                sum += bit[i];
                i &= i - 1;
            }
            sum
        }

        let mut result = Vec::with_capacity(queries.len());

        for q in &queries {
            if q[0] == 1 {
                let (u, v, new_w) = (q[1] as usize, q[2] as usize, q[3]);

                // Child is the one with larger in_time (deeper in tree)
                let child = if in_time[u] > in_time[v] { u } else { v };
                let edge_idx = parent_edge[child] as usize;

                let delta = (new_w - edge_weights[edge_idx]) as i64;
                edge_weights[edge_idx] = new_w;

                update(&mut bit, in_time[child] as usize, delta);
                update(&mut bit, out_time[child] as usize + 1, -delta);
            } else {
                let x = q[1] as usize;
                let distance = dist[x] + query(&bit, in_time[x] as usize);
                result.push(distance as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let n = 2;
        let edges = vec![vec![1, 2, 7]];
        let queries = vec![vec![2, 2], vec![1, 1, 2, 4], vec![2, 2]];
        assert_eq!(Solution::tree_queries(n, edges, queries), vec![7, 4]);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![1, 2, 2], vec![1, 3, 4]];
        let queries = vec![
            vec![2, 1],
            vec![2, 3],
            vec![1, 1, 3, 7],
            vec![2, 2],
            vec![2, 3],
        ];
        assert_eq!(Solution::tree_queries(n, edges, queries), vec![0, 4, 2, 7]);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let edges = vec![vec![1, 2, 2], vec![2, 3, 1], vec![3, 4, 5]];
        let queries = vec![
            vec![2, 4],
            vec![2, 3],
            vec![1, 2, 3, 3],
            vec![2, 2],
            vec![2, 3],
        ];
        assert_eq!(Solution::tree_queries(n, edges, queries), vec![8, 3, 2, 5]);
    }
}
