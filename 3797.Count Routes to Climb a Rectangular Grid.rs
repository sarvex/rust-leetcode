use std::cmp::min;

const MOD: u32 = 1_000_000_007;

impl Solution {
    /// Optimized DP with prefix sums for counting grid climbing routes
    ///
    /// # Intuition
    /// Use two DP states: dp0 (arrived from below) and dp1 (arrived via horizontal move).
    /// Prefix sums enable O(1) range queries for transitions within distance d.
    ///
    /// # Approach
    /// Precompute left/right bounds for horizontal and vertical moves. Process rows bottom-up,
    /// using prefix sums for efficient range sum computation. Reuse buffers to minimize allocations.
    ///
    /// # Complexity
    /// - Time: O(n * m)
    /// - Space: O(m)
    pub fn number_of_routes(grid: Vec<String>, d: i32) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let d = d as usize;

        let grid: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();

        let d_sq = d * d;
        let du = match d_sq >= 1 {
            true => Self::isqrt(d_sq - 1),
            false => 0,
        };

        let rd = min(d, m - 1);
        let rdu = min(du, m - 1);

        let ld: Vec<usize> = (0..m).map(|j| j.saturating_sub(rd)).collect();
        let rd1: Vec<usize> = (0..m).map(|j| min(m, j + rd + 1)).collect();
        let lu: Vec<usize> = (0..m).map(|j| j.saturating_sub(rdu)).collect();
        let ru1: Vec<usize> = (0..m).map(|j| min(m, j + rdu + 1)).collect();

        let mut dp0 = vec![0u32; m];
        let mut dp1 = vec![0u32; m];
        let mut new0 = vec![0u32; m];
        let mut pref = vec![0u32; m + 1];

        let bottom = grid[n - 1];
        for j in 0..m {
            if bottom[j] == b'.' {
                dp0[j] = 1;
            }
        }

        Self::build_prefix(&dp0, &mut pref);
        for j in 0..m {
            if bottom[j] == b'.' {
                let sum = Self::range_sum(&pref, ld[j], rd1[j]);
                dp1[j] = Self::sub_mod(sum, dp0[j]);
            }
        }

        for row in (0..n - 1).rev() {
            let cur_row = grid[row];

            Self::build_prefix_combined(&dp0, &dp1, &mut pref);
            for j in 0..m {
                new0[j] = if cur_row[j] == b'.' {
                    Self::range_sum(&pref, lu[j], ru1[j])
                } else {
                    0
                };
            }

            std::mem::swap(&mut dp0, &mut new0);

            Self::build_prefix(&dp0, &mut pref);
            for j in 0..m {
                dp1[j] = if cur_row[j] == b'.' {
                    let sum = Self::range_sum(&pref, ld[j], rd1[j]);
                    Self::sub_mod(sum, dp0[j])
                } else {
                    0
                };
            }
        }

        let top = grid[0];
        let mut ans = 0u32;
        for j in 0..m {
            if top[j] == b'.' {
                ans = Self::add_mod(ans, Self::add_mod(dp0[j], dp1[j]));
            }
        }
        ans as i32
    }

    #[inline(always)]
    fn isqrt(x: usize) -> usize {
        let mut r = (x as f64).sqrt() as usize;
        while (r + 1) * (r + 1) <= x {
            r += 1;
        }
        while r > 0 && r * r > x {
            r -= 1;
        }
        r
    }

    #[inline(always)]
    fn add_mod(a: u32, b: u32) -> u32 {
        let s = a + b;
        if s >= MOD {
            s - MOD
        } else {
            s
        }
    }

    #[inline(always)]
    fn sub_mod(a: u32, b: u32) -> u32 {
        if a >= b {
            a - b
        } else {
            a + MOD - b
        }
    }

    #[inline(always)]
    fn build_prefix(arr: &[u32], pref: &mut [u32]) {
        pref[0] = 0;
        for (j, &val) in arr.iter().enumerate() {
            pref[j + 1] = Self::add_mod(pref[j], val);
        }
    }

    #[inline(always)]
    fn build_prefix_combined(a: &[u32], b: &[u32], pref: &mut [u32]) {
        pref[0] = 0;
        for j in 0..a.len() {
            let mut sum = a[j] + b[j];
            if sum >= MOD {
                sum -= MOD;
            }
            pref[j + 1] = Self::add_mod(pref[j], sum);
        }
    }

    #[inline(always)]
    fn range_sum(pref: &[u32], l: usize, r1: usize) -> u32 {
        Self::sub_mod(pref[r1], pref[l])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_grid_distance_one() {
        let grid = vec!["..".to_string(), "#.".to_string()];
        assert_eq!(Solution::number_of_routes(grid, 1), 2);
    }

    #[test]
    fn test_small_grid_distance_two() {
        let grid = vec!["..".to_string(), "#.".to_string()];
        assert_eq!(Solution::number_of_routes(grid, 2), 4);
    }

    #[test]
    fn test_single_blocked_cell() {
        let grid = vec!["#".to_string()];
        assert_eq!(Solution::number_of_routes(grid, 750), 0);
    }

    #[test]
    fn test_single_row_two_columns() {
        let grid = vec!["..".to_string()];
        assert_eq!(Solution::number_of_routes(grid, 1), 4);
    }

    #[test]
    fn test_all_blocked() {
        let grid = vec!["##".to_string(), "##".to_string()];
        assert_eq!(Solution::number_of_routes(grid, 2), 0);
    }

    #[test]
    fn test_single_open_cell() {
        let grid = vec![".".to_string()];
        assert_eq!(Solution::number_of_routes(grid, 1), 1);
    }
}
