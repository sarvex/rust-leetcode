impl Solution {
    /// DP to find maximum score and count of paths on a board from 'S' to 'E'.
    ///
    /// # Intuition
    /// Movement is restricted to up, left, and diagonal up-left, so processing
    /// cells in reverse row/column order guarantees all three successors
    /// (i+1,j), (i,j+1), (i+1,j+1) are resolved before (i,j).
    ///
    /// # Approach
    /// Use two flat arrays `score` and `cnt` of length n² for cache-friendly
    /// access. `i32::MIN` marks unreachable cells. Work directly on board bytes
    /// to avoid a second allocation. For each non-obstacle, non-start cell,
    /// inspect the three successors, keep the best score and accumulate counts
    /// on ties. The final answer is at index 0 (top-left, 'E').
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const MOD: i32 = 1_000_000_007;
        const NEG_INF: i32 = i32::MIN;

        let n = board.len();
        let bytes: Vec<&[u8]> = board.iter().map(|s| s.as_bytes()).collect();

        // Flat DP arrays: index = i * n + j
        let sz = n * n;
        let mut score = vec![NEG_INF; sz];
        let mut cnt = vec![0i32; sz];

        // Seed 'S' at bottom-right
        score[sz - 1] = 0;
        cnt[sz - 1] = 1;

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                let idx = i * n + j;

                if i == n - 1 && j == n - 1 {
                    continue; // already seeded
                }
                if bytes[i][j] == b'X' {
                    continue; // obstacle
                }

                let mut best = NEG_INF;
                let mut paths = 0i32;

                // Check three successors: down, right, down-right
                for (ni, nj) in [(i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                    if ni < n && nj < n {
                        let s = score[ni * n + nj];
                        if s == NEG_INF {
                            continue;
                        }
                        if s > best {
                            best = s;
                            paths = cnt[ni * n + nj];
                        } else if s == best {
                            paths += cnt[ni * n + nj];
                            if paths >= MOD {
                                paths -= MOD;
                            }
                        }
                    }
                }

                if best == NEG_INF {
                    continue; // no reachable successor
                }

                let cell_val = match bytes[i][j] {
                    b'E' => 0,
                    d => (d - b'0') as i32,
                };

                score[idx] = best + cell_val;
                cnt[idx] = paths;
            }
        }

        if score[0] == NEG_INF {
            vec![0, 0]
        } else {
            vec![score[0] % MOD, cnt[0]]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let board = vec!["E23".to_string(), "2X2".to_string(), "12S".to_string()];
        assert_eq!(Solution::paths_with_max_score(board), vec![7, 1]);
    }

    #[test]
    fn test_example2() {
        let board = vec!["E12".to_string(), "1X1".to_string(), "21S".to_string()];
        assert_eq!(Solution::paths_with_max_score(board), vec![4, 2]);
    }

    #[test]
    fn test_no_path() {
        let board = vec!["E11".to_string(), "XXX".to_string(), "11S".to_string()];
        assert_eq!(Solution::paths_with_max_score(board), vec![0, 0]);
    }

    #[test]
    fn test_blocked_2x2() {
        let board = vec!["EX".to_string(), "XS".to_string()];
        assert_eq!(Solution::paths_with_max_score(board), vec![0, 0]);
    }

    #[test]
    fn test_open_2x2() {
        // Three paths from S(1,1): diagonal (score 0), via (0,1) (score 1), via (1,0) (score 1)
        // Max = 1, two paths that each collect one '1' cell
        let board = vec!["E1".to_string(), "1S".to_string()];
        assert_eq!(Solution::paths_with_max_score(board), vec![2, 2]);
    }

    #[test]
    fn test_all_ones_3x3() {
        // Optimal routes collect 4 intermediate cells of value 1 → score 4
        let board = vec!["E11".to_string(), "111".to_string(), "11S".to_string()];
        let result = Solution::paths_with_max_score(board);
        assert_eq!(result[0], 4);
    }
}
