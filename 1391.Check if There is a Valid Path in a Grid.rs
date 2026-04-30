impl Solution {
    /// BFS/DFS over cells using precomputed direction tables.
    ///
    /// # Intuition
    /// We only care whether `(0, 0)` reaches `(m-1, n-1)`. Instead of unioning
    /// every cell pair in the grid (DSU), walk outward from the start and stop
    /// the instant we touch the target. Each street type exposes exactly two
    /// outgoing directions, so from any cell we know in O(1) which two
    /// neighbors to probe — no bitmask arithmetic inside the hot loop beyond a
    /// single `&` to verify the neighbor faces back.
    ///
    /// # Approach
    /// 1. Encode direction indices N=0, E=1, S=2, W=3 with row/col deltas and
    ///    the "facing-back" direction index `OPP[d]`.
    /// 2. For each street `s ∈ 1..=6`, store its two outgoing direction indices
    ///    in `STREET_DIRS[s]`.
    /// 3. Also store `OPENS[s]` as a 4-bit mask so `OPENS[s] & (1 << d)` tells
    ///    us whether street `s` opens direction `d` — one bit test, no branch.
    /// 4. DFS from `(0, 0)` with a stack (less overhead than `VecDeque` and
    ///    branching factor is ≤ 2). For the current cell, walk both outgoing
    ///    directions; step into a neighbor only if its mask contains the
    ///    opposite direction. Short-circuit on reaching the target.
    ///
    /// # Complexity
    /// - Time: O(m * n) worst case; early-exits on target hit, and only
    ///   touches cells reachable from the origin.
    /// - Space: O(m * n) for the flat `visited` array.
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 && n == 1 {
            return true;
        }

        // Direction index layout: 0 = N, 1 = E, 2 = S, 3 = W.
        // `DELTA[d]` = (dr, dc) and `OPP[d]` = opposite direction index.
        const DELTA: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        const OPP: [usize; 4] = [2, 3, 0, 1];

        // For each street 1..=6, the two directions it opens.
        // Street 1: E+W, 2: N+S, 3: W+S, 4: E+S, 5: W+N, 6: E+N.
        const STREET_DIRS: [[usize; 2]; 7] = [
            [0, 0], // unused (street 0)
            [1, 3], // 1: E, W
            [0, 2], // 2: N, S
            [3, 2], // 3: W, S
            [1, 2], // 4: E, S
            [3, 0], // 5: W, N
            [1, 0], // 6: E, N
        ];

        // Bit `d` set iff street opens direction `d`.
        const OPENS: [u8; 7] = [
            0,
            (1 << 1) | (1 << 3), // 1: E + W
            (1 << 0) | (1 << 2), // 2: N + S
            (1 << 3) | (1 << 2), // 3: W + S
            (1 << 1) | (1 << 2), // 4: E + S
            (1 << 3) | (1 << 0), // 5: W + N
            (1 << 1) | (1 << 0), // 6: E + N
        ];

        let target = (m - 1) * n + (n - 1);
        let mut visited = vec![false; m * n];
        visited[0] = true;

        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(m * n);
        stack.push((0, 0));

        while let Some((r, c)) = stack.pop() {
            let dirs = STREET_DIRS[grid[r][c] as usize];
            for &d in &dirs {
                let (dr, dc) = DELTA[d];
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nc < 0 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if nr >= m || nc >= n {
                    continue;
                }
                let idx = nr * n + nc;
                if visited[idx] {
                    continue;
                }
                // Neighbor must open the opposite direction to connect back.
                if OPENS[grid[nr][nc] as usize] & (1 << OPP[d]) == 0 {
                    continue;
                }
                if idx == target {
                    return true;
                }
                visited[idx] = true;
                stack.push((nr, nc));
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
        let grid = vec![vec![2, 4, 3], vec![6, 5, 2]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![1, 2, 1], vec![1, 2, 1]];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![1, 1, 2]];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_single_cell() {
        // Any single cell trivially contains start == end.
        for v in 1..=6 {
            assert!(Solution::has_valid_path(vec![vec![v]]));
        }
    }

    #[test]
    fn test_single_row_horizontal() {
        let grid = vec![vec![1, 1, 1, 1]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_single_column_vertical() {
        let grid = vec![vec![2], vec![2], vec![2]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_l_shape_down_then_right() {
        // (0,0)=4 opens E+S -> go to (1,0)=6 opens N+E -> (1,1)=1 opens W+E.
        let grid = vec![vec![4, 1], vec![6, 1]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_disconnected_corner() {
        // Start cell opens only to the right/down but neighbors don't face back.
        let grid = vec![vec![4, 2], vec![1, 1]];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_large_horizontal() {
        let grid = vec![vec![1; 300]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_large_vertical() {
        let grid = (0..300).map(|_| vec![2]).collect();
        assert!(Solution::has_valid_path(grid));
    }
}
