/// Sparse table for range minimum queries supporting O(1) LCA queries.
struct SparseTable<T: Copy, F: Fn(T, T) -> T> {
    table: Vec<Vec<T>>,
    operation: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    /// Builds sparse table for range queries using the given operation.
    fn new(elements: Vec<T>, operation: F) -> Self {
        let n = elements.len();
        let log_levels = n.ilog2() as usize;

        let mut table = vec![elements];

        for level in 1..=log_levels {
            let step = 1 << (level - 1);
            let range_size = 1 << level;
            table.push(Vec::with_capacity(n + 1 - range_size));
            for i in 0..=n - range_size {
                let left_value = table[level - 1][i];
                let right_value = table[level - 1][i + step];
                table[level].push(operation(left_value, right_value));
            }
        }

        Self { table, operation }
    }

    /// Returns the result of the operation over range [left, right] in O(1) time.
    fn query(&self, left: usize, right: usize) -> T {
        let range_size = right - left + 1;
        let log_level = range_size.ilog2() as usize;
        let step = 1 << log_level;

        let left_value = self.table[log_level][left];
        let right_value = self.table[log_level][right + 1 - step];
        (self.operation)(left_value, right_value)
    }
}

/// Context for Euler Tour traversal storing path and weight information.
struct EulerTourContext {
    /// Euler Tour path: (depth, node_id) pairs
    euler_path: Vec<(usize, usize)>,
    /// Prefix sum of edge weights along the Euler Tour
    weight_prefix_sum: Vec<i32>,
    /// First occurrence index of each node in the Euler Tour
    first_occurrence: Vec<usize>,
}

impl EulerTourContext {
    fn new(node_count: usize) -> Self {
        Self {
            euler_path: Vec::with_capacity(2 * node_count),
            weight_prefix_sum: Vec::with_capacity(2 * node_count),
            first_occurrence: vec![0; node_count],
        }
    }
}

impl Solution {
    /// Finds minimum weighted subtree connecting two sources to destination using Euler Tour + RMQ.
    ///
    /// # Intuition
    /// In a tree, the path between any two nodes is unique. The minimum weighted subtree
    /// that connects src1 and src2 to dest is the union of paths src1->dest and src2->dest.
    /// We use Euler Tour to convert the tree into a linear array, enabling O(1) LCA queries
    /// via Range Minimum Query (RMQ) on a sparse table.
    ///
    /// # Approach
    /// 1. Build adjacency list representation of the tree
    /// 2. Perform Euler Tour DFS to create linear representation:
    ///    - Store (depth, node) pairs for each visit
    ///    - Track edge weights with +w on descent, -w on ascent
    ///    - Build prefix sum array for O(1) path weight queries
    /// 3. Build sparse table for O(1) LCA queries via RMQ
    /// 4. For each query (src1, src2, dest):
    ///    - Find LCAs using RMQ: LCA(src1, src2), LCA(src1, dest), LCA(src2, dest)
    ///    - Find deepest LCA as meeting point
    ///    - Compute union weight using prefix sums
    ///
    /// # Complexity
    /// - Time: O(n log n + q) for preprocessing and O(1) per query
    /// - Space: O(n log n) for sparse table
    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let node_count = edges.len() + 1;
        let mut adjacency_list = vec![Vec::new(); node_count];

        for edge in edges {
            let node1 = edge[0] as usize;
            let node2 = edge[1] as usize;
            let edge_weight = edge[2];

            adjacency_list[node1].push((node2, edge_weight));
            adjacency_list[node2].push((node1, edge_weight));
        }

        let mut euler_context = EulerTourContext::new(node_count);
        euler_context.weight_prefix_sum.push(0);
        Self::build_euler_tour(0, 0, 0, &adjacency_list, &mut euler_context);

        // Build prefix sum array for path weights
        for i in 1..euler_context.weight_prefix_sum.len() {
            euler_context.weight_prefix_sum[i] += euler_context.weight_prefix_sum[i - 1];
        }

        // Build sparse table for O(1) LCA queries via RMQ
        let sparse_table = SparseTable::new(euler_context.euler_path.clone(), |a, b| a.min(b));

        queries
            .iter()
            .map(|query| {
                let source1 = query[0] as usize;
                let source2 = query[1] as usize;
                let destination = query[2] as usize;

                let source1_index = euler_context.first_occurrence[source1];
                let source2_index = euler_context.first_occurrence[source2];
                let destination_index = euler_context.first_occurrence[destination];

                let lca_sources = sparse_table.query(
                    source1_index.min(source2_index),
                    source1_index.max(source2_index),
                );
                let lca_source1_destination = sparse_table.query(
                    source1_index.min(destination_index),
                    source1_index.max(destination_index),
                );
                let lca_source2_destination = sparse_table.query(
                    source2_index.min(destination_index),
                    source2_index.max(destination_index),
                );

                // Find LCA of all three nodes
                let lca_sources_index = euler_context.first_occurrence[lca_sources.1];
                let lca_all_three = sparse_table.query(
                    lca_sources_index.min(destination_index),
                    lca_sources_index.max(destination_index),
                );

                // Determine the deepest meeting point among the three LCAs
                let meeting_point = if lca_sources.0 >= lca_source1_destination.0
                    && lca_sources.0 >= lca_source2_destination.0
                {
                    lca_sources.1
                } else if lca_source1_destination.0 >= lca_sources.0
                    && lca_source1_destination.0 >= lca_source2_destination.0
                {
                    lca_source1_destination.1
                } else {
                    lca_source2_destination.1
                };

                let meeting_point_index = euler_context.first_occurrence[meeting_point];
                let lca_all_index = euler_context.first_occurrence[lca_all_three.1];

                // Compute union weight using prefix sums
                // Formula: weight(meeting->dest) + weight(root->src1) + weight(root->src2) - 2*weight(root->lca_all)
                euler_context.weight_prefix_sum[destination_index]
                    - euler_context.weight_prefix_sum[meeting_point_index]
                    + euler_context.weight_prefix_sum[source1_index]
                    + euler_context.weight_prefix_sum[source2_index]
                    - 2 * euler_context.weight_prefix_sum[lca_all_index]
            })
            .collect()
    }

    /// Performs Euler Tour DFS to build linear representation of the tree.
    fn build_euler_tour(
        current_node: usize,
        parent_node: usize,
        current_depth: usize,
        adjacency_list: &[Vec<(usize, i32)>],
        euler_context: &mut EulerTourContext,
    ) {
        euler_context.first_occurrence[current_node] = euler_context.euler_path.len();
        euler_context.euler_path.push((current_depth, current_node));

        for &(neighbor, edge_weight) in &adjacency_list[current_node] {
            if neighbor == parent_node {
                continue;
            }

            // Add edge weight on descent
            euler_context.weight_prefix_sum.push(edge_weight);
            Self::build_euler_tour(
                neighbor,
                current_node,
                current_depth + 1,
                adjacency_list,
                euler_context,
            );

            // Subtract edge weight on ascent
            euler_context.weight_prefix_sum.push(-edge_weight);
            euler_context.euler_path.push((current_depth, current_node));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 5],
            vec![1, 4, 4],
            vec![2, 5, 6],
        ];
        let queries = vec![vec![2, 3, 4], vec![0, 2, 5]];
        assert_eq!(Solution::minimum_weight(edges, queries), vec![12, 11]);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![1, 0, 8], vec![0, 2, 7]];
        let queries = vec![vec![0, 1, 2]];
        assert_eq!(Solution::minimum_weight(edges, queries), vec![15]);
    }
}
