struct SparseTable<T: Copy, F: Fn(T, T) -> T> {
    table: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    fn new(values: Vec<T>, op: F) -> Self {
        let n = values.len();
        let max_pow = n.ilog2() as usize;
        let mut table = Vec::with_capacity(max_pow + 1);
        table.push(values);

        for k in 1..=max_pow {
            let size = n + 1 - (1 << k);
            let mut level = Vec::with_capacity(size);
            let step = 1 << (k - 1);
            for i in 0..size {
                let combined = op(table[k - 1][i], table[k - 1][i + step]);
                level.push(combined);
            }
            table.push(level);
        }

        Self { table, op }
    }

    #[inline]
    fn query(&self, l: usize, r: usize) -> T {
        let k = (r - l + 1).ilog2() as usize;
        (self.op)(self.table[k][l], self.table[k][r + 1 - (1 << k)])
    }
}

fn build_euler_tour(
    node: usize,
    parent: usize,
    depth: usize,
    graph: &[Vec<(usize, u8)>],
    euler: &mut Vec<(usize, usize)>,
    first: &mut [usize],
    counter: &mut [[u16; 26]],
) {
    first[node] = euler.len();
    euler.push((depth, node));
    for &(next, weight) in &graph[node] {
        if next == parent {
            continue;
        }
        counter[next] = counter[node];
        counter[next][weight as usize] += 1;
        build_euler_tour(next, node, depth + 1, graph, euler, first, counter);
        euler.push((depth, node));
    }
}

impl Solution {
    /// Answers minimum operations per query using Euler tour RMQ for LCA.
    ///
    /// # Intuition
    /// Along any path, keeping the most frequent weight and changing the rest minimizes operations.
    ///
    /// # Approach
    /// - Build a weighted adjacency list.
    /// - Run an Euler tour from root `0` to record depths and first visits, while tracking
    ///   prefix counts of edge weights along the root path.
    /// - Build a sparse table over the Euler tour to answer LCA queries in O(1).
    /// - For each query, compute weight counts on the path via prefix differences; answer is
    ///   `path_len - max_count`.
    ///
    /// # Complexity
    /// - Time: O(n log n + 26m)
    /// - Space: O(n log n + 26n)
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let node_count = n as usize;
        if node_count == 0 {
            return Vec::new();
        }

        let mut degrees = vec![0_usize; node_count];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            degrees[u] += 1;
            degrees[v] += 1;
        }
        let mut graph: Vec<Vec<(usize, u8)>> = degrees
            .into_iter()
            .map(|degree| Vec::with_capacity(degree))
            .collect();
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = (edge[2] - 1) as u8;
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let mut counter = vec![[0_u16; 26]; node_count];
        let mut first = vec![0_usize; node_count];
        let mut euler = Vec::with_capacity(2 * node_count - 1);
        build_euler_tour(
            0,
            usize::MAX,
            0,
            &graph,
            &mut euler,
            &mut first,
            &mut counter,
        );

        let rmq = SparseTable::new(euler, |a, b| if a.0 <= b.0 { a } else { b });

        let mut answers = Vec::with_capacity(queries.len());
        for query in queries {
            let a = query[0] as usize;
            let b = query[1] as usize;
            if a == b {
                answers.push(0);
                continue;
            }
            let left = first[a].min(first[b]);
            let right = first[a].max(first[b]);
            let lca = rmq.query(left, right).1;

            let mut total = 0_i32;
            let mut max_count = 0_i32;
            for weight in 0..26 {
                let count = i32::from(counter[a][weight]) + i32::from(counter[b][weight])
                    - 2 * i32::from(counter[lca][weight]);
                total += count;
                if count > max_count {
                    max_count = count;
                }
            }
            answers.push(total - max_count);
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let n = 7;
        let edges = vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 2],
            vec![5, 6, 2],
        ];
        let queries = vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]];
        assert_eq!(
            Solution::min_operations_queries(n, edges, queries),
            vec![0, 0, 1, 3]
        );
    }

    #[test]
    fn test_example_2() {
        let n = 8;
        let edges = vec![
            vec![1, 2, 6],
            vec![1, 3, 4],
            vec![2, 4, 6],
            vec![2, 5, 3],
            vec![3, 6, 6],
            vec![3, 0, 8],
            vec![7, 0, 2],
        ];
        let queries = vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]];
        assert_eq!(
            Solution::min_operations_queries(n, edges, queries),
            vec![1, 2, 2, 3]
        );
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges: Vec<Vec<i32>> = Vec::new();
        let queries = vec![vec![0, 0]];
        assert_eq!(Solution::min_operations_queries(n, edges, queries), vec![0]);
    }
}
