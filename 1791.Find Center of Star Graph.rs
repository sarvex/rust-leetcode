impl Solution {
    /// Finds the center node of a star graph from two edges.
    ///
    /// # Intuition
    /// The center node appears in every edge. Comparing just the first two
    /// edges reveals the shared node immediately.
    ///
    /// # Approach
    /// 1. Check if the first node of edge 0 appears in edge 1.
    /// 2. If so, it is the center; otherwise, the second node of edge 0 is.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_is_first_node() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![1, 3], vec![1, 4]]),
            1
        );
    }

    #[test]
    fn test_center_is_second_node() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![2, 4]]),
            2
        );
    }
}
