use std::collections::BTreeSet;

impl Solution {
    /// Kth smallest path XOR sum using small-to-large set merging.
    ///
    /// # Intuition
    /// The path XOR sum from root to each node can be computed with a simple DFS/BFS traversal.
    /// For subtree queries, we need to efficiently collect and query distinct XOR values.
    /// Small-to-large merging ensures optimal complexity when combining children's XOR sets.
    ///
    /// # Approach
    /// 1. Build adjacency list from parent array representing children for each node
    /// 2. Compute path XOR sums - each node's XOR is parent's XOR ^ node's value
    /// 3. Pre-sort queries by (node, k) to avoid per-node allocations during traversal
    /// 4. Use iterative post-order DFS with small-to-large set merging:
    ///    - Process children first, then merge their XOR sets into parent
    ///    - Always merge smaller set into larger for O(n log n) total insertions
    ///    - Answer queries using binary search on pre-sorted query array
    ///
    /// # Complexity
    /// - Time: O(n log²n + q log q) where n is number of nodes, q is number of queries
    /// - Space: O(n + q) for storing sets and query structures
    pub fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = par.len();

        let children = Self::build_children(&par, n);
        let path_xor = Self::compute_path_xors(&children, &vals, n);

        let mut indexed_queries: Vec<(usize, usize, usize)> = queries
            .iter()
            .enumerate()
            .map(|(i, q)| (q[0] as usize, q[1] as usize, i))
            .collect();
        indexed_queries.sort_unstable();

        Self::process_queries(&children, &path_xor, &indexed_queries, n, queries.len())
    }

    #[inline]
    fn build_children(par: &[i32], n: usize) -> Vec<Vec<usize>> {
        let mut children = vec![vec![]; n];
        for (i, &p) in par.iter().enumerate().skip(1) {
            children[p as usize].push(i);
        }
        children
    }

    #[inline]
    fn compute_path_xors(children: &[Vec<usize>], vals: &[i32], n: usize) -> Vec<i32> {
        let mut path_xor = vec![0; n];
        path_xor[0] = vals[0];
        let mut stack = vec![0usize];

        while let Some(u) = stack.pop() {
            for &v in &children[u] {
                path_xor[v] = path_xor[u] ^ vals[v];
                stack.push(v);
            }
        }

        path_xor
    }

    fn process_queries(
        children: &[Vec<usize>],
        path_xor: &[i32],
        indexed_queries: &[(usize, usize, usize)],
        n: usize,
        query_count: usize,
    ) -> Vec<i32> {
        let mut results = vec![-1; query_count];
        let mut sets: Vec<Option<BTreeSet<i32>>> = vec![None; n];
        let mut stack: Vec<(usize, bool)> = vec![(0, false)];

        while let Some((u, processed)) = stack.pop() {
            if processed {
                let set = Self::merge_subtree_sets(u, children, path_xor, &mut sets);
                Self::answer_node_queries(u, &set, indexed_queries, &mut results);
                sets[u] = Some(set);
            } else {
                stack.push((u, true));
                for &v in &children[u] {
                    stack.push((v, false));
                }
            }
        }

        results
    }

    #[inline]
    fn merge_subtree_sets(
        u: usize,
        children: &[Vec<usize>],
        path_xor: &[i32],
        sets: &mut [Option<BTreeSet<i32>>],
    ) -> BTreeSet<i32> {
        let mut set = BTreeSet::new();
        set.insert(path_xor[u]);

        for &v in &children[u] {
            let child_set = sets[v].take().expect("Child set should exist");
            set = Self::small_to_large_merge(set, child_set);
        }

        set
    }

    #[inline]
    fn small_to_large_merge(set_a: BTreeSet<i32>, set_b: BTreeSet<i32>) -> BTreeSet<i32> {
        let (mut larger, smaller) = if set_b.len() > set_a.len() {
            (set_b, set_a)
        } else {
            (set_a, set_b)
        };

        for x in smaller {
            larger.insert(x);
        }

        larger
    }

    #[inline]
    fn answer_node_queries(
        u: usize,
        set: &BTreeSet<i32>,
        indexed_queries: &[(usize, usize, usize)],
        results: &mut [i32],
    ) {
        let start = indexed_queries.partition_point(|&(node, _, _)| node < u);
        let end = indexed_queries.partition_point(|&(node, _, _)| node <= u);

        if start == end {
            return;
        }

        let set_len = set.len();
        let queries_slice = &indexed_queries[start..end];

        // Fast path: if all queries have k=1, use first() which is O(1)
        if queries_slice.iter().all(|&(_, k, _)| k == 1) {
            if let Some(&first) = set.first() {
                for &(_, _, idx) in queries_slice {
                    results[idx] = first;
                }
            }
            return;
        }

        let mut iter = set.iter();
        let mut pos = 0_usize;
        let mut last_val = 0_i32;

        for &(_, k, idx) in queries_slice {
            if k > set_len {
                continue;
            }
            if k > pos {
                last_val = *iter.nth(k - 1 - pos).unwrap();
                pos = k;
            }
            results[idx] = last_val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let par = vec![-1, 0, 0];
        let vals = vec![1, 1, 1];
        let queries = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        assert_eq!(Solution::kth_smallest(par, vals, queries), vec![0, 1, -1]);
    }

    #[test]
    fn test_example_2() {
        let par = vec![-1, 0, 1];
        let vals = vec![5, 2, 7];
        let queries = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 1]];
        assert_eq!(
            Solution::kth_smallest(par, vals, queries),
            vec![0, 7, -1, 0]
        );
    }

    #[test]
    fn test_single_node() {
        let par = vec![-1];
        let vals = vec![42];
        let queries = vec![vec![0, 1], vec![0, 2]];
        assert_eq!(Solution::kth_smallest(par, vals, queries), vec![42, -1]);
    }

    #[test]
    fn test_linear_tree() {
        let par = vec![-1, 0, 1, 2];
        let vals = vec![1, 2, 4, 8];
        let queries = vec![vec![0, 1], vec![0, 4], vec![3, 1]];
        let result = Solution::kth_smallest(par, vals, queries);
        assert_eq!(result[2], 1 ^ 2 ^ 4 ^ 8);
    }

    #[test]
    fn test_star_tree() {
        let par = vec![-1, 0, 0, 0, 0];
        let vals = vec![0, 1, 2, 3, 4];
        let queries = vec![vec![0, 1], vec![0, 6], vec![1, 1]];
        let result = Solution::kth_smallest(par, vals, queries);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], -1);
        assert_eq!(result[2], 1);
    }

    #[test]
    fn test_duplicate_queries() {
        let par = vec![-1, 0];
        let vals = vec![15425, 65425];
        let queries = vec![vec![1, 1], vec![1, 1]];
        let result = Solution::kth_smallest(par, vals, queries);
        let expected = 15425 ^ 65425;
        assert_eq!(result[0], expected);
        assert_eq!(result[1], expected);
    }
}
