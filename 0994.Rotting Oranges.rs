use std::collections::VecDeque;

impl Solution {
    /// Finds minimum minutes for all oranges to rot using multi-source BFS.
    ///
    /// # Intuition
    /// All initially rotten oranges spread simultaneously. BFS level-by-level
    /// simulates the time progression of rotting.
    ///
    /// # Approach
    /// Enqueue all rotten oranges and count fresh ones. BFS: each level is
    /// one minute. When a fresh orange is reached, decrement the count.
    /// Return minutes when count reaches 0, or -1 if fresh oranges remain.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n) for the queue
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::new();
        let mut fresh = 0;

        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    1 => fresh += 1,
                    2 => queue.push_back((i, j)),
                    _ => {}
                }
            }
        }

        let mut minutes = 0;
        while !queue.is_empty() && fresh > 0 {
            minutes += 1;
            for _ in 0..queue.len() {
                let (r, c) = queue.pop_front().unwrap();
                for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
                    let nr = r.wrapping_add(dr);
                    let nc = c.wrapping_add(dc);
                    if nr < m && nc < n && grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }

        if fresh > 0 { -1 } else { minutes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
    }

    #[test]
    fn test_impossible() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
    }

    #[test]
    fn test_no_fresh() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }
}
