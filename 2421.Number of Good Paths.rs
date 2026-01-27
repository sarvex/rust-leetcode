struct UnionFind {
    parent: Vec<usize>,
    nodes_with_max_value: Vec<usize>,
    max_value: Vec<i32>,
    good_path_count: usize,
}

impl UnionFind {
    fn new(values: &[i32]) -> Self {
        let n = values.len();
        UnionFind {
            parent: (0..n).collect(),
            nodes_with_max_value: vec![1; n],
            max_value: values.to_vec(),
            good_path_count: 0,
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }
        self.parent[node]
    }

    fn union(&mut self, node_a: usize, node_b: usize) {
        let (root_a, root_b) = (self.find(node_a), self.find(node_b));
        if root_a == root_b {
            return;
        }

        match self.max_value[root_a].cmp(&self.max_value[root_b]) {
            std::cmp::Ordering::Less => {
                self.parent[root_a] = root_b;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_b] = root_a;
            }
            std::cmp::Ordering::Equal => {
                self.good_path_count +=
                    self.nodes_with_max_value[root_a] * self.nodes_with_max_value[root_b];
                self.parent[root_a] = root_b;
                self.nodes_with_max_value[root_b] += self.nodes_with_max_value[root_a];
            }
        }
    }
}

impl Solution {
    /// Counts good paths in a tree using Union-Find sorted by edge max value.
    ///
    /// # Intuition
    /// Process edges in order of max endpoint value. Union-by-max-value ensures
    /// roots always hold the maximum value, simplifying merge logic.
    ///
    /// # Approach
    /// 1. Sort edges by max(vals[u], vals[v])
    /// 2. Union components using max value as rank
    /// 3. When merging same-max components, add count1 * count2 pairs
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting, O(n * alpha(n)) for Union-Find
    /// - Space: O(n) for Union-Find arrays
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(&vals);
        let mut edges = edges;
        edges.sort_unstable_by_key(|e| vals[e[0] as usize].max(vals[e[1] as usize]));

        for edge in &edges {
            uf.union(edge[0] as usize, edge[1] as usize);
        }

        (vals.len() + uf.good_path_count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let vals = vec![1, 3, 2, 1, 3];
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 6);
    }

    #[test]
    fn test_example_2() {
        let vals = vec![1, 1, 2, 2, 3];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 7);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::number_of_good_paths(vec![1], vec![]), 1);
    }

    #[test]
    fn test_single_edge_same_values() {
        assert_eq!(
            Solution::number_of_good_paths(vec![2, 2], vec![vec![0, 1]]),
            3
        );
    }

    #[test]
    fn test_all_same_values() {
        let vals = vec![1, 1, 1];
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 6);
    }

    #[test]
    fn test_linear_increasing() {
        let vals = vec![1, 2, 3];
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 3);
    }

    #[test]
    fn test_star_topology() {
        let vals = vec![3, 1, 1, 1];
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 4);
    }

    #[test]
    fn test_all_same_four_nodes() {
        let vals = vec![5, 5, 5, 5];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 10);
    }
}
