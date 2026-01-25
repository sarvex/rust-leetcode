impl Solution {
    /// Finds the weighted median node for each query on a tree.
    ///
    /// # Intuition
    /// The weighted median on a path is the first node where cumulative edge weight
    /// from the source reaches at least half the total path weight. We leverage LCA
    /// (Lowest Common Ancestor) with binary lifting to efficiently navigate paths.
    ///
    /// # Approach
    /// 1. Build adjacency list and compute parent/depth/distance using DFS
    /// 2. Precompute sparse table for O(log n) LCA queries
    /// 3. For each query (u, v):
    ///    - Find LCA and calculate total path weight
    ///    - Threshold = (total + 1) / 2 for "greater than or equal to half"
    ///    - If cumulative from u to LCA >= threshold, median is on u-side
    ///    - Otherwise, median is on v-side of LCA
    ///    - Use binary lifting to pinpoint the exact median node
    ///
    /// # Complexity
    /// - Time: O((n + q) * log n) for preprocessing and queries
    /// - Space: O(n * log n) for LCA sparse table
    pub fn find_median(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let log = 20;

        let mut adj = vec![vec![]; n];
        edges.iter().for_each(|e| {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
            adj[u].push((v, w));
            adj[v].push((u, w));
        });

        let mut parent = vec![vec![0usize; n]; log];
        let mut depth = vec![0usize; n];
        let mut dist = vec![0i64; n];

        let mut stack = vec![(0usize, 0usize)];
        let mut visited = vec![false; n];
        visited[0] = true;

        while let Some((u, p)) = stack.pop() {
            parent[0][u] = p;
            for &(v, w) in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    depth[v] = depth[u] + 1;
                    dist[v] = dist[u] + w;
                    stack.push((v, u));
                }
            }
        }

        (1..log).for_each(|k| {
            (0..n).for_each(|v| {
                parent[k][v] = parent[k - 1][parent[k - 1][v]];
            });
        });

        let lca = |mut u: usize, mut v: usize| -> usize {
            if depth[u] < depth[v] {
                std::mem::swap(&mut u, &mut v);
            }
            let diff = depth[u] - depth[v];
            (0..log)
                .filter(|&k| (diff >> k) & 1 == 1)
                .for_each(|k| u = parent[k][u]);

            if u == v {
                return u;
            }

            (0..log).rev().for_each(|k| {
                if parent[k][u] != parent[k][v] {
                    u = parent[k][u];
                    v = parent[k][v];
                }
            });
            parent[0][u]
        };

        queries
            .iter()
            .map(|q| {
                let (u, v) = (q[0] as usize, q[1] as usize);
                let l = lca(u, v);
                let total = dist[u] + dist[v] - 2 * dist[l];
                let threshold = (total + 1) / 2;
                let u_to_lca = dist[u] - dist[l];

                if u_to_lca >= threshold {
                    let target = dist[u] - threshold;
                    let mut cur = u;
                    (0..log).rev().for_each(|k| {
                        let anc = parent[k][cur];
                        if depth[anc] >= depth[l] && dist[anc] > target {
                            cur = anc;
                        }
                    });
                    if dist[cur] <= target {
                        cur as i32
                    } else {
                        parent[0][cur] as i32
                    }
                } else {
                    let target = dist[l] + (threshold - u_to_lca);
                    let mut cur = v;
                    (0..log).rev().for_each(|k| {
                        let anc = parent[k][cur];
                        if depth[anc] > depth[l] && dist[anc] >= target {
                            cur = anc;
                        }
                    });
                    cur as i32
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 2;
        let edges = vec![vec![0, 1, 7]];
        let queries = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(Solution::find_median(n, edges, queries), vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![0, 1, 2], vec![2, 0, 4]];
        let queries = vec![vec![0, 1], vec![2, 0], vec![1, 2]];
        assert_eq!(Solution::find_median(n, edges, queries), vec![1, 0, 2]);
    }

    #[test]
    fn test_example_3() {
        let n = 5;
        let edges = vec![vec![0, 1, 2], vec![0, 2, 5], vec![1, 3, 1], vec![2, 4, 3]];
        let queries = vec![vec![3, 4], vec![1, 2]];
        assert_eq!(Solution::find_median(n, edges, queries), vec![2, 2]);
    }

    #[test]
    fn test_same_node_query() {
        let n = 2;
        let edges = vec![vec![0, 1, 5]];
        let queries = vec![vec![0, 0]];
        assert_eq!(Solution::find_median(n, edges, queries), vec![0]);
    }

    #[test]
    fn test_linear_chain() {
        let n = 4;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1]];
        let queries = vec![vec![0, 3], vec![3, 0]];
        assert_eq!(Solution::find_median(n, edges, queries), vec![2, 1]);
    }

    #[test]
    fn test_unbalanced_weights() {
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![0, 2, 100]];
        let queries = vec![vec![1, 2]];
        assert_eq!(Solution::find_median(n, edges, queries), vec![2]);
    }
}
