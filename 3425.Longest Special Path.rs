impl Solution {
    /// Iterative DFS with Forward Star graph for longest path with unique values
    ///
    /// # Intuition
    /// A special path requires all node values to be unique. We maintain a sliding
    /// window on the current root-to-node path, tracking where duplicates occur.
    ///
    /// # Approach
    /// 1. Build Forward Star (CSR-like) graph for cache-friendly traversal
    /// 2. Iterative DFS with explicit stack storing backtrack state
    /// 3. Use last occurrence array to detect duplicates; slide start on conflict
    /// 4. Update result at each node: maximize length, minimize nodes on tie
    ///
    /// # Complexity
    /// - Time: O(n) for single DFS traversal
    /// - Space: O(n + max_val) for graph, stack, path, and occurrence tracking
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![0, 1];
        }

        // Build Forward Star representation (flattened adjacency list)
        let m = edges.len();
        let mut head = vec![-1_i32; n];
        let mut next = Vec::with_capacity(m * 2);
        let mut to = Vec::with_capacity(m * 2);
        let mut weight = Vec::with_capacity(m * 2);

        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];

            next.push(head[u]);
            head[u] = to.len() as i32;
            to.push(v as i32);
            weight.push(w);

            next.push(head[v]);
            head[v] = to.len() as i32;
            to.push(u as i32);
            weight.push(w);
        }

        let max_val = nums.iter().fold(0, |a, &x| a.max(x)) as usize + 1;
        let mut last = vec![-1_i32; max_val];

        // Frame: (node, parent, dist, edge_idx, old_pos, old_start)
        let mut stack: Vec<(i32, i32, i32, i32, i32, i32)> = Vec::with_capacity(n);
        stack.push((0, -1, 0, -2, -1, 0));

        let mut path_dist: Vec<i32> = Vec::with_capacity(n);
        let mut start = 0_i32;
        let mut max_len = 0;
        let mut min_nodes = 1;

        while let Some(&(node, parent, dist, edge, old_pos, old_s)) = stack.last() {
            let u = node as usize;

            if edge == -2 {
                // Enter node
                let val = nums[u] as usize;
                let prev = last[val];
                let saved_start = start;

                if prev >= start {
                    start = prev + 1;
                }

                let curr = path_dist.len() as i32;
                path_dist.push(dist);
                last[val] = curr;

                let len = if start > 0 {
                    dist - path_dist[start as usize]
                } else {
                    dist
                };
                let cnt = curr - start + 1;

                if len > max_len || (len == max_len && cnt < min_nodes) {
                    max_len = len;
                    min_nodes = cnt;
                }

                // Update frame with saved state and first edge
                let frame = stack.last_mut().unwrap();
                frame.3 = head[u];
                frame.4 = prev;
                frame.5 = saved_start;
            } else if edge != -1 {
                // Traverse edge
                let idx = edge as usize;
                let next_edge = next[idx];
                stack.last_mut().unwrap().3 = next_edge;

                let v = to[idx];
                if v != parent {
                    let w = weight[idx];
                    stack.push((v, node, dist + w, -2, -1, 0));
                }
            } else {
                // Backtrack
                let val = nums[u] as usize;
                last[val] = old_pos;
                start = old_s;
                path_dist.pop();
                stack.pop();
            }
        }

        vec![max_len, min_nodes]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let edges = vec![
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 5],
            vec![1, 4, 4],
            vec![2, 5, 6],
        ];
        let nums = vec![2, 1, 2, 1, 3, 1];
        assert_eq!(Solution::longest_special_path(edges, nums), vec![6, 2]);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![1, 0, 8]];
        let nums = vec![2, 2];
        assert_eq!(Solution::longest_special_path(edges, nums), vec![0, 1]);
    }

    #[test]
    fn test_single_path_all_unique() {
        let edges = vec![vec![0, 1, 5], vec![1, 2, 3], vec![2, 3, 7]];
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::longest_special_path(edges, nums), vec![15, 4]);
    }

    #[test]
    fn test_all_same_value() {
        let edges = vec![vec![0, 1, 10], vec![1, 2, 20]];
        let nums = vec![5, 5, 5];
        assert_eq!(Solution::longest_special_path(edges, nums), vec![0, 1]);
    }
}
