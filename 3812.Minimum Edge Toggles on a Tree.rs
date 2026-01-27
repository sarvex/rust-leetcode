impl Solution {
    /// Leaf-peeling approach to determine minimum edge toggles on a tree
    ///
    /// # Intuition
    /// A leaf node has only one edge. If a leaf needs flipping, we must toggle that edge.
    /// Process leaves iteratively: toggle edge if needed, remove leaf, repeat with new leaves.
    ///
    /// # Approach
    /// 1. Build adjacency list and track node degrees
    /// 2. Initialize stack with all leaves (degree == 1)
    /// 3. For each leaf: if mismatched, toggle its edge (flipping both endpoints)
    /// 4. Remove leaf by decrementing neighbor degrees, push new leaves
    /// 5. Collect toggled edge indices as the result
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn minimum_flips(n: i32, edges: Vec<Vec<i32>>, start: String, target: String) -> Vec<i32> {
        let n = n as usize;
        let m = edges.len();

        if n == 1 {
            return if start == target { vec![] } else { vec![-1] };
        }

        let start_bytes = start.as_bytes();
        let target_bytes = target.as_bytes();
        let mut mismatch: Vec<bool> = (0..n).map(|i| start_bytes[i] != target_bytes[i]).collect();

        let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut degree: Vec<usize> = vec![0; n];

        for (i, edge) in edges.iter().enumerate() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push((i, v));
            graph[v].push((i, u));
            degree[u] += 1;
            degree[v] += 1;
        }

        let mut stack: Vec<usize> = (0..n).filter(|&v| degree[v] == 1).collect();
        let mut toggle = vec![false; m];

        while let Some(v) = stack.pop() {
            if mismatch[v] {
                match graph[v].iter().find(|(_, u)| degree[*u] > 0) {
                    Some(&(edge_idx, u)) => {
                        mismatch[u] = !mismatch[u];
                        toggle[edge_idx] = true;
                    }
                    None => return vec![-1],
                }
            }

            for &(_, u) in &graph[v] {
                if degree[u] > 0 {
                    degree[u] -= 1;
                    if degree[u] == 1 {
                        stack.push(u);
                    }
                }
            }
            degree[v] = 0;
        }

        toggle
            .iter()
            .enumerate()
            .filter_map(|(i, &t)| t.then_some(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_three_node_flip() {
        let result = Solution::minimum_flips(
            3,
            vec![vec![0, 1], vec![1, 2]],
            "010".to_string(),
            "100".to_string(),
        );
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_seven_node_multiple_flips() {
        let result = Solution::minimum_flips(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![3, 5],
                vec![1, 6],
            ],
            "0011000".to_string(),
            "0010001".to_string(),
        );
        assert_eq!(result, vec![1, 2, 5]);
    }

    #[test]
    fn test_impossible_two_nodes() {
        let result =
            Solution::minimum_flips(2, vec![vec![0, 1]], "00".to_string(), "01".to_string());
        assert_eq!(result, vec![-1]);
    }

    #[test]
    fn test_single_node_match() {
        let result = Solution::minimum_flips(1, vec![], "0".to_string(), "0".to_string());
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_single_node_mismatch() {
        let result = Solution::minimum_flips(1, vec![], "0".to_string(), "1".to_string());
        assert_eq!(result, vec![-1]);
    }

    #[test]
    fn test_already_matching() {
        let result = Solution::minimum_flips(
            3,
            vec![vec![0, 1], vec![1, 2]],
            "010".to_string(),
            "010".to_string(),
        );
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_two_nodes_both_flip() {
        let result =
            Solution::minimum_flips(2, vec![vec![0, 1]], "00".to_string(), "11".to_string());
        assert_eq!(result, vec![0]);
    }
}
