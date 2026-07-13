impl Solution {
    /// Union-Find with edge counting to detect complete components.
    ///
    /// # Intuition
    /// A complete component with k vertices must have exactly k*(k-1)/2 edges.
    /// Group vertices into connected components via Union-Find, then verify
    /// each component's edge count matches the complete-graph formula.
    ///
    /// # Approach
    /// 1. Build Union-Find over all vertices.
    /// 2. Union each edge and track edge count per root.
    /// 3. After processing, count vertices per root.
    /// 4. A component is complete iff edges == vertices*(vertices-1)/2.
    ///
    /// # Complexity
    /// - Time: O(n + e * α(n)) where e = number of edges
    /// - Space: O(n)
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank = vec![0usize; n];

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
            let (rx, ry) = (find(parent, x), find(parent, y));
            if rx == ry {
                return;
            }
            match rank[rx].cmp(&rank[ry]) {
                std::cmp::Ordering::Less => parent[rx] = ry,
                std::cmp::Ordering::Greater => parent[ry] = rx,
                std::cmp::Ordering::Equal => {
                    parent[ry] = rx;
                    rank[rx] += 1;
                }
            }
        }

        // Union all edges
        for edge in &edges {
            union(&mut parent, &mut rank, edge[0] as usize, edge[1] as usize);
        }

        // Count vertices and edges per component root
        let mut vertex_count = vec![0usize; n];
        let mut edge_count = vec![0usize; n];

        for i in 0..n {
            let root = find(&mut parent, i);
            vertex_count[root] += 1;
        }

        for edge in &edges {
            let root = find(&mut parent, edge[0] as usize);
            edge_count[root] += 1;
        }

        // A complete component with k vertices has k*(k-1)/2 edges
        (0..n)
            .filter(|&i| {
                parent[i] == i && edge_count[i] == vertex_count[i] * (vertex_count[i] - 1) / 2
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // All three components are complete: {0,1,2}, {3,4}, {5}
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
            ),
            3
        );
    }

    #[test]
    fn test_example_2() {
        // {0,1,2} is complete, {3,4,5} is not (missing edge 4-5)
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
            ),
            1
        );
    }

    #[test]
    fn test_no_edges() {
        // Each isolated vertex is a complete component (trivially)
        assert_eq!(Solution::count_complete_components(4, vec![]), 4);
    }

    #[test]
    fn test_single_vertex() {
        assert_eq!(Solution::count_complete_components(1, vec![]), 1);
    }

    #[test]
    fn test_complete_graph_k4() {
        // K4: 4 vertices, 6 edges — one complete component
        assert_eq!(
            Solution::count_complete_components(
                4,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3]
                ]
            ),
            1
        );
    }

    #[test]
    fn test_two_complete_components() {
        // {0,1,2} complete + {3,4,5} complete
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![4, 5]
                ]
            ),
            2
        );
    }

    #[test]
    fn test_single_edge() {
        // {0,1} is complete (K2), {2} is complete (isolated)
        assert_eq!(Solution::count_complete_components(3, vec![vec![0, 1]]), 2);
    }
}
