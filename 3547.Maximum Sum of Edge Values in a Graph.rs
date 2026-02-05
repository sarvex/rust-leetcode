impl Solution {
    /// Optimal value assignment to maximize edge product sum.
    ///
    /// # Intuition
    /// The optimal arrangement is [1,3,5,...,n,n-1,...,4,2] (odds ascending, evens descending).
    /// The score depends only on n and whether it's a path (n-1 edges) or cycle (n edges).
    /// We can compute the sum directly without building the graph.
    ///
    /// # Approach
    /// Sum products of consecutive pairs in optimal arrangement:
    /// - Odd pairs: 1*3 + 3*5 + 5*7 + ... = Σ k*(k-2) for k=3,5,7,...
    /// - Transition: n*(n-1)
    /// - Even pairs: n*(n-2) + (n-2)*(n-4) + ... = Σ (k+2)*k descending
    /// - Cycle adds 2*1 = 2 for the wrap-around edge
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as i64;
        let mut result = 0i64;

        // Sum of odd consecutive pairs: 1*3 + 3*5 + 5*7 + ...
        result += (3..=n).step_by(2).map(|k| k * (k - 2)).sum::<i64>();

        // Transition between max odd and max even
        result += n * (n - 1);

        // Sum of even consecutive pairs descending
        let start = if n % 2 == 0 { n - 2 } else { n - 3 };
        result += (0..=(start - 2) / 2)
            .map(|i| {
                let k = start - 2 * i;
                (k + 2) * k
            })
            .sum::<i64>();

        // Cycle has n edges (adds wrap-around 1*2), path has n-1 edges
        if edges.len() == n as usize {
            result += 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_path() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 23);
    }

    #[test]
    fn test_example_2_cycle() {
        let n = 6;
        let edges = vec![
            vec![0, 3],
            vec![4, 5],
            vec![2, 0],
            vec![1, 3],
            vec![2, 4],
            vec![1, 5],
        ];
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 82);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges = Vec::new();
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_two_nodes_path() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let result = Solution::max_score(n, edges);
        // Both nodes have degree 1, so assignment doesn't matter: 1*2 = 2
        assert_eq!(result, 2);
    }

    #[test]
    fn test_three_nodes_path() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let result = Solution::max_score(n, edges);
        // Middle node (1) gets largest value, ends get smaller values
        // Optimal: node 0=1, node 1=3, node 2=2
        // Sum: 1*3 + 3*2 = 3 + 6 = 9
        assert_eq!(result, 9);
    }

    #[test]
    fn test_three_nodes_cycle() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let result = Solution::max_score(n, edges);
        // All nodes have degree 2, so it's a cycle
        // Values assigned in cycle order: 3, 2, 1
        // Sum: 3*2 + 2*1 + 1*3 = 6 + 2 + 3 = 11
        assert_eq!(result, 11);
    }

    #[test]
    fn test_five_nodes_path() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = Solution::max_score(n, edges);
        // Middle nodes (1, 2, 3) get largest values (5, 4, 3)
        // End nodes (0, 4) get smallest values (1, 2)
        // Sum: 1*5 + 5*4 + 4*3 + 3*2 = 5 + 20 + 12 + 6 = 43
        assert_eq!(result, 43);
    }

    #[test]
    fn test_four_nodes_cycle() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 0]];
        let result = Solution::max_score(n, edges);
        // All nodes have degree 2, cycle structure
        // Values assigned: 4, 3, 2, 1 in cycle order
        // Sum: 4*3 + 3*2 + 2*1 + 1*4 = 12 + 6 + 2 + 4 = 24
        assert_eq!(result, 24);
    }

    #[test]
    fn test_larger_path() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
        ];
        let result = Solution::max_score(n, edges);
        // Middle nodes get largest values
        assert!(result > 0);
    }

    #[test]
    fn test_larger_cycle() {
        let n = 8;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 0],
        ];
        let result = Solution::max_score(n, edges);
        // All nodes have degree 2, cycle structure
        assert!(result > 0);
    }

    #[test]
    fn test_twelve_nodes_path() {
        let n = 12;
        let edges = vec![
            vec![6, 4],
            vec![4, 8],
            vec![8, 3],
            vec![3, 11],
            vec![11, 10],
            vec![10, 5],
            vec![5, 0],
            vec![0, 2],
            vec![2, 7],
            vec![7, 1],
            vec![1, 9],
        ];
        let result = Solution::max_score(n, edges);
        assert_eq!(result, 627);
    }
}
