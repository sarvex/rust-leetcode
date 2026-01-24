impl Solution {
    /// Minimum Cost Path with Teleportations using optimized Dijkstra
    ///
    /// # Intuition
    /// Since Dijkstra processes states in cost order, when we first unlock a threshold,
    /// that's the minimum cost. Track max expanded threshold per teleport count to avoid
    /// re-expanding cells.
    ///
    /// # Approach
    /// 1. Group cells by value for efficient expansion
    /// 2. Track max_expanded[t] = highest threshold expanded for t teleports
    /// 3. When unlocking threshold v, only expand values in (max_expanded[t+1], v]
    /// 4. Each value level expanded at most once per teleport count
    ///
    /// # Complexity
    /// - Time: O(mnk * log(mnk))
    /// - Space: O(mnk)
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let (m, n) = (grid.len(), grid[0].len());
        let k = k as usize;
        let max_val = grid
            .iter()
            .flat_map(|r| r.iter())
            .copied()
            .max()
            .unwrap_or(0) as usize;

        // Group cells by their value
        let mut cells_by_val: Vec<Vec<(usize, usize)>> = vec![vec![]; max_val + 1];
        for i in 0..m {
            for j in 0..n {
                cells_by_val[grid[i][j] as usize].push((i, j));
            }
        }

        // dist[t][i][j] = min cost to reach (i,j) with t teleports
        let mut dist = vec![vec![vec![i32::MAX; n]; m]; k + 1];
        // max_expanded[t] = highest threshold that has been expanded for t teleports
        let mut max_expanded: Vec<i32> = vec![-1; k + 1];

        dist[0][0][0] = 0;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0i32, 0usize, 0usize, 0usize)));

        while let Some(Reverse((cost, i, j, t))) = pq.pop() {
            if cost > dist[t][i][j] {
                continue;
            }

            if i == m - 1 && j == n - 1 {
                return cost;
            }

            // Normal moves: down and right
            if i + 1 < m {
                let new_cost = cost + grid[i + 1][j];
                if new_cost < dist[t][i + 1][j] {
                    dist[t][i + 1][j] = new_cost;
                    pq.push(Reverse((new_cost, i + 1, j, t)));
                }
            }
            if j + 1 < n {
                let new_cost = cost + grid[i][j + 1];
                if new_cost < dist[t][i][j + 1] {
                    dist[t][i][j + 1] = new_cost;
                    pq.push(Reverse((new_cost, i, j + 1, t)));
                }
            }

            // Teleportation: unlock threshold = current cell's value
            if t < k {
                let threshold = grid[i][j];
                let prev = max_expanded[t + 1];

                if threshold > prev {
                    // Only expand values in range (prev, threshold] - each value expanded once
                    let start = (prev + 1).max(0) as usize;
                    let end = threshold as usize;
                    for v in start..=end {
                        for &(ni, nj) in &cells_by_val[v] {
                            if cost < dist[t + 1][ni][nj] {
                                dist[t + 1][ni][nj] = cost;
                                pq.push(Reverse((cost, ni, nj, t + 1)));
                            }
                        }
                    }
                    max_expanded[t + 1] = threshold;
                }
            }
        }

        (0..=k)
            .filter_map(|t| {
                let d = dist[t][m - 1][n - 1];
                if d == i32::MAX {
                    None
                } else {
                    Some(d)
                }
            })
            .min()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]];
        assert_eq!(Solution::min_cost(grid, 2), 7);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::min_cost(grid, 1), 9);
    }

    #[test]
    fn no_teleports() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::min_cost(grid, 0), 6);
    }

    #[test]
    fn teleport_to_destination() {
        let grid = vec![vec![5, 3, 3], vec![2, 5, 4], vec![4, 3, 1]];
        assert_eq!(Solution::min_cost(grid, 1), 0);
    }

    #[test]
    fn large_k() {
        let grid = vec![vec![1, 100], vec![100, 1]];
        assert_eq!(Solution::min_cost(grid, 10), 0);
    }
}
