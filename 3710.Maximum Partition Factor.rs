impl Solution {
    /// Maximum partition factor through optimal point splitting
    ///
    /// # Intuition
    /// The partition factor is the minimum Manhattan distance among pairs within the same group.
    /// To maximize this, pairs with small distances should be in different groups. This is a
    /// graph coloring problem where points with distance below a threshold must be separated.
    ///
    /// # Approach
    /// 1. Binary search on the partition factor threshold
    /// 2. For each threshold, build a constraint graph where edges connect points with distance < threshold
    /// 3. Check if the graph is bipartite (2-colorable) using DFS
    /// 4. The maximum threshold for which the graph remains bipartite is the answer
    ///
    /// # Complexity
    /// - Time: O(n² log n) for binary search with bipartite check
    /// - Space: O(n²) for distance matrix
    pub fn max_partition_factor(points: Vec<Vec<i32>>) -> i64 {
        let n = points.len();
        if n == 2 {
            return 0;
        }

        // Precompute all pairwise Manhattan distances
        let mut dist_matrix = vec![vec![0i64; n]; n];
        let mut distances: Vec<i64> = Vec::new();

        (0..n).for_each(|i| {
            (i + 1..n).for_each(|j| {
                let d = (points[i][0] as i64 - points[j][0] as i64).abs()
                    + (points[i][1] as i64 - points[j][1] as i64).abs();
                dist_matrix[i][j] = d;
                dist_matrix[j][i] = d;
                distances.push(d);
            });
        });

        distances.sort_unstable();
        distances.dedup();
        distances.push(distances.last().copied().unwrap_or(0) + 1);

        // Check if we can partition with all intra-group distances >= threshold
        let can_partition = |threshold: i64| -> bool {
            let mut color = vec![-1i32; n];

            (0..n).all(|start| {
                if color[start] != -1 {
                    return true;
                }

                let mut stack = vec![start];
                color[start] = 0;

                while let Some(u) = stack.pop() {
                    for v in 0..n {
                        if v != u && dist_matrix[u][v] < threshold {
                            match color[v] {
                                -1 => {
                                    color[v] = 1 - color[u];
                                    stack.push(v);
                                }
                                c if c == color[u] => return false,
                                _ => {}
                            }
                        }
                    }
                }
                true
            })
        };

        // Binary search for maximum threshold where bipartite split exists
        let (mut lo, mut hi) = (0, distances.len() - 1);

        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if can_partition(distances[mid]) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        distances[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_points() {
        // Points form a square, optimal split: diagonals in separate groups
        let points = vec![vec![0, 0], vec![0, 2], vec![2, 0], vec![2, 2]];
        assert_eq!(Solution::max_partition_factor(points), 4);
    }

    #[test]
    fn test_singleton_group() {
        // Optimal: one singleton group, pair in other group
        let points = vec![vec![0, 0], vec![0, 1], vec![10, 0]];
        assert_eq!(Solution::max_partition_factor(points), 11);
    }

    #[test]
    fn test_two_points() {
        // Special case: n=2, partition factor is 0 by definition
        let points = vec![vec![0, 0], vec![5, 5]];
        assert_eq!(Solution::max_partition_factor(points), 0);
    }

    #[test]
    fn test_collinear_points() {
        // Points on a line
        let points = vec![vec![0, 0], vec![1, 0], vec![3, 0], vec![6, 0]];
        assert_eq!(Solution::max_partition_factor(points), 3);
    }

    #[test]
    fn test_identical_points() {
        // All points at same location
        let points = vec![vec![0, 0], vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::max_partition_factor(points), 0);
    }

    #[test]
    fn test_negative_coordinates() {
        // Points with negative coordinates
        let points = vec![vec![-5, -5], vec![5, 5], vec![-5, 5]];
        assert_eq!(Solution::max_partition_factor(points), 20);
    }
}
