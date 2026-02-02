use std::collections::VecDeque;

impl Solution {
    /// Counts distinct nodes visited in each walk.
    ///
    /// # Intuition
    /// Each node has exactly one outgoing edge, so every walk eventually enters a directed cycle.
    /// Cycle nodes visit exactly the cycle length, while tree nodes visit their distance to the
    /// cycle plus that length.
    ///
    /// # Approach
    /// - Compute indegrees and remove nodes with indegree 0 via queue (topological pruning).
    ///   The remaining nodes are exactly the cycle nodes.
    /// - For each remaining cycle, traverse it once to obtain its size and assign that size to
    ///   its nodes.
    /// - Process the removed nodes in reverse order and set `answer[node] = answer[next] + 1`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn count_visited_nodes(edges: Vec<i32>) -> Vec<i32> {
        let n = edges.len();
        let next: Vec<usize> = edges.into_iter().map(|v| v as usize).collect();

        let mut indegree = vec![0_usize; n];
        for &v in &next {
            indegree[v] += 1;
        }

        let mut queue = VecDeque::new();
        for (node, &deg) in indegree.iter().enumerate() {
            if deg == 0 {
                queue.push_back(node);
            }
        }

        let mut removed_order = Vec::with_capacity(n);
        while let Some(node) = queue.pop_front() {
            removed_order.push(node);
            let to = next[node];
            indegree[to] -= 1;
            if indegree[to] == 0 {
                queue.push_back(to);
            }
        }

        let mut answer = vec![0_i32; n];
        for node in 0..n {
            if indegree[node] == 0 || answer[node] != 0 {
                continue;
            }
            let mut cycle_nodes = Vec::new();
            let mut cur = node;
            loop {
                cycle_nodes.push(cur);
                cur = next[cur];
                if cur == node {
                    break;
                }
            }
            let cycle_len = cycle_nodes.len() as i32;
            for v in cycle_nodes {
                answer[v] = cycle_len;
            }
        }

        for &node in removed_order.iter().rev() {
            let to = next[node];
            answer[node] = answer[to] + 1;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::count_visited_nodes(vec![1, 2, 0, 0]),
            vec![3, 3, 3, 4],
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::count_visited_nodes(vec![1, 2, 3, 4, 0]),
            vec![5, 5, 5, 5, 5],
        );
    }

    #[test]
    fn test_multiple_cycles() {
        assert_eq!(
            Solution::count_visited_nodes(vec![1, 0, 3, 2]),
            vec![2, 2, 2, 2],
        );
    }

    #[test]
    fn test_chain_into_cycle() {
        assert_eq!(
            Solution::count_visited_nodes(vec![1, 0, 1, 2]),
            vec![2, 2, 3, 4],
        );
    }
}
