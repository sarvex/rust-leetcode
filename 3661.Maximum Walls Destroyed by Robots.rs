impl Solution {
    /// Maximum walls destroyed by robots via direction-choice DP with binary search.
    ///
    /// # Intuition
    /// Each robot fires left or right, blocked by adjacent robots. The key insight
    /// is that when robot `i` fires right, its effective range depends on what robot
    /// `i+1` does: if the next robot fires left, the boundary is tighter to avoid
    /// double-counting the overlap region.
    ///
    /// # Approach
    /// 1. Sort robots by position, sort walls
    /// 2. Bottom-up DP over robots left-to-right with state `j` indicating what
    ///    robot `i+1` fires (0=left, 1=right)
    /// 3. For each robot, compute wall counts via binary search:
    ///    - Left-firing range: `[max(pos-dist, prev_pos+1), pos]`
    ///    - Right-firing range: `[pos, min(pos+dist, right_cap_j)]` where cap
    ///      depends on j
    /// 4. Transition: `dp[j] = max(old[0] + CL, old[1] + CR_j)`
    ///
    /// # Complexity
    /// - Time: O(n log n + m log m + n log m)
    /// - Space: O(n + m)
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let mut walls = walls;
        walls.sort_unstable();

        if n == 0 || walls.is_empty() {
            return 0;
        }

        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_unstable_by_key(|&i| robots[i]);
        let arr: Vec<[i32; 2]> = idx.iter().map(|&i| [robots[i], distance[i]]).collect();

        let lb = |val: i32| walls.partition_point(|&w| w < val) as i32;

        // dp[j]: max walls from robots 0..=i, given robot i+1 fires direction j
        let mut dp = [0i32; 2];

        for i in 0..n {
            let (pos, dist) = (arr[i][0], arr[i][1]);

            // Robot i fires LEFT: range [left, pos]
            let left = if i > 0 {
                (pos - dist).max(arr[i - 1][0] + 1)
            } else {
                pos - dist
            };
            let cl = lb(pos + 1) - lb(left);

            // Robot i fires RIGHT: range [pos, right_j]
            let base_right = pos + dist;
            let (cr0, cr1) = if i + 1 < n {
                let right0 = base_right.min(arr[i + 1][0] - arr[i + 1][1] - 1);
                let right1 = base_right.min(arr[i + 1][0] - 1);
                (
                    (lb(right0 + 1) - lb(pos)).max(0),
                    (lb(right1 + 1) - lb(pos)).max(0),
                )
            } else {
                let cr = lb(base_right + 1) - lb(pos);
                (cr, cr)
            };

            dp = [(dp[0] + cl).max(dp[1] + cr0), (dp[0] + cl).max(dp[1] + cr1)];
        }

        dp[0].max(dp[1])
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
