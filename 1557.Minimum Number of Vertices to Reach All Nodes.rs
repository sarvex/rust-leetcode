impl Solution {
    /// Collect nodes with zero in-degree as required starting vertices.
    ///
    /// # Intuition
    /// In a DAG, a node with no incoming edges cannot be reached from any
    /// other node, so it must be a starting vertex. Nodes with incoming edges
    /// can be reached transitively.
    ///
    /// # Approach
    /// 1. Mark all destination nodes as reachable
    /// 2. Return nodes that are never a destination (in-degree = 0)
    ///
    /// # Complexity
    /// - Time: O(n + e)
    /// - Space: O(n) for the reachability array
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut has_incoming = vec![false; n as usize];
        for edge in &edges {
            has_incoming[edge[1] as usize] = true;
        }
        (0..n).filter(|&i| !has_incoming[i as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_dag() {
        let mut result = Solution::find_smallest_set_of_vertices(
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]],
        );
        result.sort_unstable();
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn chain() {
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                5,
                vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]],
            ),
            vec![0, 2, 3]
        );
    }
}
