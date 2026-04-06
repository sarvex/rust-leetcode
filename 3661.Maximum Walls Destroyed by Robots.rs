impl Solution {
    /// Maximum walls destroyed by robots via merged-array linear DP.
    ///
    /// # Intuition
    /// Merge robots (pos, dist) and walls (pos, MAX) with two sentinel robots
    /// into one sorted array. Each interval between adjacent robots contains only
    /// walls. Count left-reachable (`lc`) and right-reachable (`rc`) walls per
    /// interval, then DP over firing directions with `min(c, lc+rc)` to prevent
    /// double-counting overlap.
    ///
    /// # Approach
    /// 1. Merge robots and walls into `(position, distance)` pairs, walls marked
    ///    with `i32::MAX` distance. Add sentinels `(0,0)` and `(MAX,0)`
    /// 2. Sort, then `retain` to remove walls coinciding with robots (always destroyed)
    /// 3. Linear scan: for each robot→robot interval, count `lc` (left robot's
    ///    rightward reach), `rc` (right robot's leftward reach), `c` (total walls)
    /// 4. DP: `dp_l` = best if current robot fires left, `dp_r` = fires right
    ///
    /// # Complexity
    /// - Time: O((n + m) log(n + m)) for sort, O(n + m) for linear scan
    /// - Space: O(n + m)
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let m = walls.len();
        let mut ss = Vec::with_capacity(n + m + 2);
        for (r, d) in robots.into_iter().zip(distance) {
            ss.push((r, d));
        }
        ss.push((0, 0));
        ss.push((i32::MAX, 0));
        for w in walls {
            ss.push((w, i32::MAX));
        }
        ss.sort_unstable();

        let mut res0 = 0;
        let mut last = 0;
        ss.retain(|&(x, d)| {
            if d == i32::MAX && last == x {
                res0 += 1;
                return false;
            }
            last = x;
            true
        });

        let mut i = 0;
        let mut dp_l = 0;
        let mut dp_r = 0;
        loop {
            let (lx, ld) = ss[i];
            if lx == i32::MAX {
                break;
            }
            let lxx = lx + ld;
            let mut lc = 0;
            let mut rx = 0;
            let mut rd = 0;
            let mut j = i + 1;
            loop {
                (rx, rd) = ss[j];
                if rd < i32::MAX {
                    break;
                }
                if rx <= lxx {
                    lc += 1;
                }
                j += 1;
            }
            let c = (j - i - 1) as i32;
            let mut rc = 0;
            let rxx = rx - rd;
            for k in (i + 1..j).rev() {
                if ss[k].0 >= rxx {
                    rc += 1;
                } else {
                    break;
                }
            }
            let dp_l_1 = (dp_l + rc).max(dp_r + c.min(lc + rc));
            let dp_r_1 = dp_l.max(dp_r + lc);
            dp_l = dp_l_1;
            dp_r = dp_r_1;
            i = j;
        }

        dp_l.max(dp_r) + res0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_robot_fires_toward_more_walls() {
        assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1);
    }

    #[test]
    fn two_robots_optimal_direction_covers_all() {
        assert_eq!(
            Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]),
            3
        );
    }

    #[test]
    fn blocking_prevents_distant_wall_destruction() {
        assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
    }

    #[test]
    fn wall_at_robot_position_always_destroyed() {
        assert_eq!(Solution::max_walls(vec![5], vec![1], vec![5]), 1);
    }

    #[test]
    fn multiple_walls_selects_best_direction() {
        assert_eq!(
            Solution::max_walls(vec![10], vec![5], vec![5, 7, 8, 12, 15]),
            3
        );
    }

    #[test]
    fn overlapping_ranges_dp_selects_optimal() {
        assert_eq!(
            Solution::max_walls(vec![5, 10], vec![3, 4], vec![3, 7, 12]),
            2
        );
    }

    #[test]
    fn three_robots_coordinate_directions() {
        assert_eq!(
            Solution::max_walls(vec![5, 10, 15], vec![3, 3, 3], vec![2, 7, 12, 18]),
            3
        );
    }

    #[test]
    fn no_walls_in_any_range() {
        assert_eq!(
            Solution::max_walls(vec![50], vec![5], vec![1, 2, 100, 200]),
            0
        );
    }
}
