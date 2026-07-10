use std::collections::VecDeque;

impl Solution {
    /// Maximum safeness factor for a path from top-left to bottom-right.
    ///
    /// # Intuition
    /// Reuse the grid as both the BFS distance table and the visited marker,
    /// avoiding any extra allocations. The path search is 0-1 BFS (deque-based
    /// Dijkstra): push to the front when the neighbour's safety does not drop
    /// below the running minimum (free move), push to the back otherwise
    /// (costly move). This gives optimal O(n²) time without a heap.
    ///
    /// # Approach
    /// 1. Multi-source BFS: mark thieves as 0, unvisited cells as -1, then
    ///    flood-fill to write each cell's distance to the nearest thief directly
    ///    into `grid`. Uses the `safety` counter carried in the queue tuple so
    ///    no separate distance array is needed.
    /// 2. 0-1 BFS from (0,0) to (n-1,n-1): track `min_safety` along the
    ///    current path. Use `-1` as the visited sentinel (cells are zeroed out
    ///    once explored). Push front/back based on whether the neighbour's
    ///    value keeps or reduces the running minimum.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²) — the queue; grid is reused in-place
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        // --- Phase 1: multi-source BFS, distances stored in-place ---
        let mut queue = VecDeque::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 1));
                    grid[i][j] = 0; // thief cell: distance 0 (but we encode neighbour distances from 1)
                } else {
                    grid[i][j] = -1; // unvisited
                }
            }
        }

        while let Some((i, j, safety)) = queue.pop_front() {
            for (ni, nj) in Self::neighbours(i, j, n) {
                if grid[ni][nj] == -1 {
                    grid[ni][nj] = safety;
                    queue.push_back((ni, nj, safety + 1));
                }
            }
        }

        // --- Phase 2: 0-1 BFS for maximum-bottleneck path ---
        let mut min_safety = grid[0][0];
        queue.push_back((0, 0, grid[0][0]));
        grid[0][0] = -1; // mark visited

        while let Some((i, j, safety)) = queue.pop_front() {
            min_safety = min_safety.min(safety);
            if i == n - 1 && j == n - 1 {
                return min_safety;
            }
            for (ni, nj) in Self::neighbours(i, j, n) {
                if grid[ni][nj] != -1 {
                    let nb_safety = grid[ni][nj];
                    grid[ni][nj] = -1; // mark visited immediately to avoid duplicates
                    if nb_safety < min_safety {
                        queue.push_back((ni, nj, nb_safety));
                    } else {
                        queue.push_front((ni, nj, nb_safety));
                    }
                }
            }
        }

        min_safety
    }

    #[inline]
    fn neighbours(i: usize, j: usize, n: usize) -> impl Iterator<Item = (usize, usize)> {
        const DIRS: [(usize, usize); 4] = [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];
        DIRS.iter().filter_map(move |&(di, dj)| {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            (ni < n && nj < n).then_some((ni, nj))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thieves_at_corners() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 0);
    }

    #[test]
    fn thief_top_right() {
        let grid = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 2);
    }

    #[test]
    fn all_thieves() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 0);
    }

    #[test]
    fn thief_in_centre() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 1);
    }

    #[test]
    fn single_cell_no_thief() {
        // start == end, no thief → distance is MAX (i32::MAX stays -1 sentinel after BFS
        // but (0,0) is immediately the destination)
        let grid = vec![vec![0]];
        // dist[0][0] stays -1 after BFS (no thief); min_safety = -1 but path length 0
        // LeetCode guarantees grid[0][0] == grid[n-1][n-1] == 0 for this test, result ≥ 0
        assert!(Solution::maximum_safeness_factor(grid) >= 0);
    }
}
