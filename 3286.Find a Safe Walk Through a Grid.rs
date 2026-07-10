use std::collections::VecDeque;

impl Solution {
    /// 0-1 BFS with early exit and health-based pruning.
    ///
    /// # Intuition
    /// Edge weights are binary (0 for safe, 1 for unsafe), so 0-1 BFS gives
    /// optimal O(m*n) complexity. Two key pruning opportunities: exit immediately
    /// when the destination is first dequeued (guaranteed optimal), and skip any
    /// neighbor whose cumulative cost already exceeds the health budget.
    ///
    /// # Approach
    /// Maintain `dis[r][c]` as minimum health consumed to reach each cell.
    /// Safe neighbors (cost 0) go to the front of the deque; unsafe (cost 1) go
    /// to the back. Prune branches where `dis[cx][cy] + grid[nx][ny] >= health`
    /// since they cannot reach the destination with health ≥ 1.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n)
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dis = vec![vec![i32::MAX; n]; m];
        dis[0][0] = grid[0][0];

        let mut deque = VecDeque::with_capacity(m * n);
        deque.push_front((0usize, 0usize));

        const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((cx, cy)) = deque.pop_front() {
            // First dequeue of destination is guaranteed to be the minimum cost
            if cx == m - 1 && cy == n - 1 {
                return true;
            }

            for (dx, dy) in DIRS {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;
                if nx < 0 || ny < 0 || nx >= m as i32 || ny >= n as i32 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                let cost = dis[cx][cy] + grid[nx][ny];

                // Prune: path already violates health requirement
                if cost >= health {
                    continue;
                }

                if cost < dis[nx][ny] {
                    dis[nx][ny] = cost;
                    if grid[nx][ny] == 0 {
                        deque.push_front((nx, ny));
                    } else {
                        deque.push_back((nx, ny));
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        assert!(Solution::find_safe_walk(grid, 1));
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![0, 1, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 1],
            vec![0, 0, 1, 0, 1, 0],
        ];
        assert!(!Solution::find_safe_walk(grid, 3));
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert!(Solution::find_safe_walk(grid, 5));
    }

    #[test]
    fn test_all_safe() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert!(Solution::find_safe_walk(grid, 1));
    }

    #[test]
    fn test_single_unsafe_not_enough_health() {
        let grid = vec![vec![1]];
        assert!(!Solution::find_safe_walk(grid, 1));
    }

    #[test]
    fn test_single_unsafe_enough_health() {
        let grid = vec![vec![1]];
        assert!(Solution::find_safe_walk(grid, 2));
    }

    #[test]
    fn test_start_is_destination_safe() {
        let grid = vec![vec![0]];
        assert!(Solution::find_safe_walk(grid, 1));
    }
}
