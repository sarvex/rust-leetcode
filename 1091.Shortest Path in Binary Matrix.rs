use std::collections::VecDeque;

impl Solution {
    /// Finds shortest clear path in a binary matrix using BFS.
    ///
    /// # Intuition
    /// BFS from top-left to bottom-right guarantees the shortest path in
    /// an unweighted grid. Move in all 8 directions.
    ///
    /// # Approach
    /// Start BFS from (0,0) if clear. Mark cells as visited by setting to 1.
    /// Expand in 8 directions. Return path length when reaching (n-1, n-1).
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n^2) for the queue
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        grid[0][0] = 1;
        let mut steps = 1;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (r, c) = queue.pop_front().unwrap();
                if r == n - 1 && c == n - 1 {
                    return steps;
                }
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                            let (nr, nc) = (nr as usize, nc as usize);
                            if grid[nr][nc] == 0 {
                                grid[nr][nc] = 1;
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
    }

    #[test]
    fn test_three_by_three() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0],
            ]),
            4
        );
    }

    #[test]
    fn test_blocked() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![1, 0], vec![0, 0]]),
            -1
        );
    }
}
