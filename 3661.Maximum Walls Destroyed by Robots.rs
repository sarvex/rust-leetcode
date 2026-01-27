impl Solution {
    /// Maximum walls destroyed by robots with direction-choice DP
    ///
    /// # Intuition
    /// Each robot can fire left or right, with bullets blocked by adjacent robots.
    /// Walls between adjacent robots can be covered by at most two robot-direction
    /// pairs, creating a local dependency structure suitable for dynamic programming.
    ///
    /// # Approach
    /// 1. Sort robots by position and precompute firing ranges considering blocking
    /// 2. For each interval between adjacent robots, categorize walls into:
    ///    - A: only reachable by left robot firing right
    ///    - B: only reachable by right robot firing left
    ///    - C: reachable by both options
    /// 3. Use DP where dp[i][dir] = max walls from robots 0..i with robot i firing dir
    /// 4. Handle walls at robot positions (always destroyed) and outside boundaries
    ///
    /// # Complexity
    /// - Time: O((n + m) log(n + m)) for sorting, O(n log m) for DP with binary search
    /// - Space: O(n + m) for sorted arrays and DP state
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let n = robots.len();

        let mut robot_data: Vec<(i64, i64)> = robots
            .iter()
            .zip(distance.iter())
            .map(|(&p, &d)| (p as i64, d as i64))
            .collect();
        robot_data.sort_unstable_by_key(|&(p, _)| p);

        let mut walls: Vec<i64> = walls.into_iter().map(|w| w as i64).collect();
        walls.sort_unstable();

        let robot_positions: HashSet<i64> = robot_data.iter().map(|&(p, _)| p).collect();

        let walls_at_robots = walls.iter().filter(|w| robot_positions.contains(w)).count() as i32;

        let count_walls = |lo: i64, hi: i64| -> i32 {
            match lo > hi {
                true => 0,
                false => {
                    let left = walls.partition_point(|&w| w < lo);
                    let right = walls.partition_point(|&w| w <= hi);
                    (right - left) as i32
                }
            }
        };

        if n == 1 {
            let (p, d) = robot_data[0];
            let left_outside = count_walls(p - d, p - 1);
            let right_outside = count_walls(p + 1, p + d);
            return walls_at_robots + left_outside.max(right_outside);
        }

        let left_outside = count_walls(robot_data[0].0 - robot_data[0].1, robot_data[0].0 - 1);
        let right_outside = count_walls(
            robot_data[n - 1].0 + 1,
            robot_data[n - 1].0 + robot_data[n - 1].1,
        );

        let mut dp = vec![[0i32; 2]; n];
        dp[0][0] = left_outside;
        dp[0][1] = 0;

        (1..n).for_each(|i| {
            let (p1, d1) = robot_data[i - 1];
            let (p2, d2) = robot_data[i];

            let r1_right = (p1 + d1).min(p2 - 1);
            let l2_left = (p1 + 1).max(p2 - d2);

            let a = match l2_left > p1 + 1 {
                true => count_walls(p1 + 1, r1_right.min(l2_left - 1)),
                false => 0,
            };

            let b = match r1_right < p2 - 1 {
                true => count_walls(l2_left.max(r1_right + 1), p2 - 1),
                false => 0,
            };

            let c_lo = (p1 + 1).max(l2_left);
            let c_hi = r1_right.min(p2 - 1);
            let c = count_walls(c_lo, c_hi);

            dp[i][0] = (dp[i - 1][0] + b + c).max(dp[i - 1][1] + a + b + c);
            dp[i][1] = dp[i - 1][0].max(dp[i - 1][1] + a + c);
        });

        walls_at_robots + dp[n - 1][0].max(dp[n - 1][1] + right_outside)
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
