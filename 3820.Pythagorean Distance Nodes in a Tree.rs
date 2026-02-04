use std::collections::VecDeque;

struct Solution;

impl Solution {
    /// Count nodes whose distances to x, y, z form a Pythagorean triplet (a² + b² = c²).
    ///
    /// # Intuition
    /// For each node we need distances to x, y, z. In a tree, single-source distances are
    /// computed by BFS. A node is special iff the three distances, when sorted, satisfy
    /// a² + b² = c².
    ///
    /// # Approach
    /// - Build adjacency list from edges.
    /// - Run BFS from x, y, and z to get distance arrays to each target.
    /// - For each node u, take (dx, dy, dz), sort to (a, b, c), and check a² + b² == c².
    /// - Count such nodes.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
        let n = n as usize;
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;

        let mut adj = vec![Vec::new(); n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let dist_x = Self::bfs_distances(n, &adj, x);
        let dist_y = Self::bfs_distances(n, &adj, y);
        let dist_z = Self::bfs_distances(n, &adj, z);

        (0..n)
            .filter(|&u| {
                let mut d = [dist_x[u], dist_y[u], dist_z[u]];
                d.sort_unstable();
                let (a, b, c) = (d[0] as i64, d[1] as i64, d[2] as i64);
                a * a + b * b == c * c
            })
            .count() as i32
    }

    fn bfs_distances(n: usize, adj: &[Vec<usize>], start: usize) -> Vec<i32> {
        let mut dist = vec![-1; n];
        dist[start] = 0;
        let mut queue = VecDeque::from([start]);
        while let Some(u) = queue.pop_front() {
            let d = dist[u];
            for &v in &adj[u] {
                if dist[v] == -1 {
                    dist[v] = d + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::special_nodes(
                4,
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                1,
                2,
                3
            ),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::special_nodes(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                0,
                3,
                2
            ),
            0
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::special_nodes(
                4,
                vec![vec![0, 1], vec![1, 2], vec![1, 3]],
                1,
                3,
                0
            ),
            1
        );
    }
}
