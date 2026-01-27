use std::collections::VecDeque;

impl Solution {
    /// Maximum safeness factor for a path from top-left to bottom-right.
    ///
    /// # Intuition
    /// First compute each cell's distance to the nearest thief via multi-source
    /// BFS. Then binary search on the safeness value: for a candidate value `v`,
    /// check via DFS whether a path exists using only cells with distance > v.
    ///
    /// # Approach
    /// 1. Multi-source BFS from all thief cells to compute distance grid.
    /// 2. Binary search on answer `v` in `[0, max_distance)`.
    /// 3. For each candidate, DFS from (0,0) to (n-1,n-1) through cells with
    ///    distance > v.
    ///
    /// # Complexity
    /// - Time: O(n² log n)
    /// - Space: O(n²)
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dist = vec![vec![-1i32; n]; n];
        let mut queue = VecDeque::new();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j));
                }
            }
        }

        let mut level = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (i, j) = queue.pop_front().unwrap();
                if dist[i][j] != -1 {
                    continue;
                }
                dist[i][j] = level;
                for (di, dj) in [(0, 1), (0, !0), (1, 0), (!0, 0)] {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if ni < n && nj < n {
                        queue.push_back((ni, nj));
                    }
                }
            }
            level += 1;
        }

        let (mut lo, mut hi) = (0, level);
        while lo < hi {
            let mid = (lo + hi) >> 1;
            let mut vis = vec![vec![false; n]; n];
            match Self::dfs(0, 0, mid, &dist, &mut vis) {
                true => lo = mid + 1,
                false => hi = mid,
            }
        }
        hi
    }

    fn dfs(i: usize, j: usize, threshold: i32, dist: &[Vec<i32>], vis: &mut [Vec<bool>]) -> bool {
        let n = dist.len();
        if vis[i][j] || dist[i][j] <= threshold {
            return false;
        }
        vis[i][j] = true;
        if i == n - 1 && j == n - 1 {
            return true;
        }
        [(0, 1), (0, !0), (1, 0), (!0, 0)].iter().any(|&(di, dj)| {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            ni < n && nj < n && Self::dfs(ni, nj, threshold, dist, vis)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_grid() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 0);
    }

    #[test]
    fn safe_path_exists() {
        let grid = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 2);
    }

    #[test]
    fn all_thieves() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::maximum_safeness_factor(grid), 0);
    }
}
