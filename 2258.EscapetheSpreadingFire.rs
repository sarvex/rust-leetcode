use std::collections::VecDeque;

impl Solution {
    /// Escape the Spreading Fire
    ///
    /// # Intuition
    /// The maximum wait time has a monotonic property: if we can escape after waiting
    /// `t` minutes, we can also escape after waiting any time less than `t`. This
    /// suggests binary search on the wait time.
    ///
    /// # Approach
    /// 1. Multi-source BFS from all fire cells to compute fire arrival time at each cell
    /// 2. Binary search on the maximum wait time
    /// 3. For each candidate wait time, BFS from (0,0) to check if we can reach safehouse
    ///    - For intermediate cells: must arrive strictly before fire
    ///    - For safehouse: can arrive at same time as fire (edge case from problem)
    ///
    /// # Complexity
    /// - Time: O(m * n * log(m * n)) - binary search with BFS verification
    /// - Space: O(m * n) - for fire times and visited arrays
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        const INF: i32 = i32::MAX;

        // Multi-source BFS to compute fire arrival times
        let mut fire_time = vec![vec![INF; n]; m];
        let mut queue = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    fire_time[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for &(dx, dy) in &DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 0 && fire_time[nx][ny] == INF {
                        fire_time[nx][ny] = fire_time[x][y] + 1;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        // Check if escape is possible with given wait time
        let can_escape = |wait: i32| -> bool {
            let mut visited = vec![vec![false; n]; m];
            let mut queue = VecDeque::new();

            // Check if starting position is already on fire
            if fire_time[0][0] != INF && fire_time[0][0] <= wait {
                return false;
            }

            visited[0][0] = true;
            queue.push_back((0usize, 0usize, wait));

            while let Some((x, y, t)) = queue.pop_front() {
                if x == m - 1 && y == n - 1 {
                    return true;
                }

                for &(dx, dy) in &DIRS {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if !visited[nx][ny] && grid[nx][ny] == 0 {
                            let arrival = t + 1;
                            // Safehouse: can arrive at same time as fire
                            // Other cells: must arrive strictly before fire
                            let reachable = if nx == m - 1 && ny == n - 1 {
                                fire_time[nx][ny] == INF || arrival <= fire_time[nx][ny]
                            } else {
                                fire_time[nx][ny] == INF || arrival < fire_time[nx][ny]
                            };

                            if reachable {
                                visited[nx][ny] = true;
                                queue.push_back((nx, ny, arrival));
                            }
                        }
                    }
                }
            }
            false
        };

        // Edge case: impossible to reach safehouse
        if !can_escape(0) {
            return -1;
        }

        // Edge case: fire never blocks the path
        let max_possible = (m * n) as i32;
        if can_escape(max_possible) {
            return 1_000_000_000;
        }

        // Binary search for maximum wait time
        let (mut lo, mut hi) = (0, max_possible);
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if can_escape(mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![0, 2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 2, 1, 0],
            vec![0, 2, 0, 0, 1, 2, 0],
            vec![0, 0, 2, 2, 2, 0, 2],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::maximum_minutes(grid), 3);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![0, 0, 0, 0], vec![0, 1, 2, 0], vec![0, 2, 0, 0]];
        assert_eq!(Solution::maximum_minutes(grid), -1);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![0, 0, 0], vec![2, 2, 0], vec![1, 2, 0]];
        assert_eq!(Solution::maximum_minutes(grid), 1_000_000_000);
    }

    #[test]
    fn test_no_wait_needed() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::maximum_minutes(grid), 1_000_000_000);
    }

    #[test]
    fn test_fire_at_start() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::maximum_minutes(grid), -1);
    }
}
