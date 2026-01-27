impl Solution {
    /// Offline Union-Find with sorted queries for edge-limited paths.
    ///
    /// # Intuition
    /// Process queries in increasing limit order. For each query, union all
    /// edges with weight below the limit. Two nodes are connected by a valid
    /// path if they share the same Union-Find root at query time.
    ///
    /// # Approach
    /// 1. Sort queries by limit, sort edges by weight
    /// 2. Process queries in order: union edges below current limit
    /// 3. Check connectivity for each query
    /// 4. Place results back in original query order
    ///
    /// # Complexity
    /// - Time: O((n + m + q) · α(n)) where m = edges, q = queries
    /// - Space: O(n + q)
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut parent: Vec<usize> = (0..n as usize).collect();
        let mut result = vec![false; queries.len()];

        let mut query_order: Vec<usize> = (0..queries.len()).collect();
        query_order.sort_unstable_by_key(|&i| queries[i][2]);

        let mut edges = edge_list;
        edges.sort_unstable_by_key(|e| e[2]);

        let mut edge_idx = 0;
        for &qi in &query_order {
            let limit = queries[qi][2];
            while edge_idx < edges.len() && edges[edge_idx][2] < limit {
                Self::union(
                    edges[edge_idx][0] as usize,
                    edges[edge_idx][1] as usize,
                    &mut parent,
                );
                edge_idx += 1;
            }
            result[qi] = Self::find(queries[qi][0] as usize, &mut parent)
                == Self::find(queries[qi][1] as usize, &mut parent);
        }

        result
    }

    fn find(x: usize, parent: &mut Vec<usize>) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent[x], parent);
        }
        parent[x]
    }

    fn union(a: usize, b: usize, parent: &mut Vec<usize>) {
        let pa = Self::find(a, parent);
        let pb = Self::find(b, parent);
        parent[pa] = pb;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_queries() {
        assert_eq!(
            Solution::distance_limited_paths_exist(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
                vec![vec![0, 1, 2], vec![0, 2, 5]],
            ),
            vec![false, true]
        );
    }

    #[test]
    fn disconnected() {
        assert_eq!(
            Solution::distance_limited_paths_exist(
                5,
                vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]],
                vec![vec![0, 4, 14], vec![1, 4, 13]],
            ),
            vec![true, false]
        );
    }
}
