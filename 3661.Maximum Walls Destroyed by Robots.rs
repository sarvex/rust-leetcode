impl Solution {
    /// Maximum Walls Destroyed by Robots
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

        // Sort robots by position, keeping distances associated
        let mut robot_data: Vec<(i64, i64)> = robots
            .iter()
            .zip(distance.iter())
            .map(|(&p, &d)| (p as i64, d as i64))
            .collect();
        robot_data.sort_unstable_by_key(|&(p, _)| p);

        // Sort walls
        let mut walls: Vec<i64> = walls.into_iter().map(|w| w as i64).collect();
        walls.sort_unstable();

        // Create set of robot positions for quick lookup
        let robot_positions: HashSet<i64> = robot_data.iter().map(|&(p, _)| p).collect();

        // Count walls at robot positions (always destroyed regardless of direction)
        let walls_at_robots = walls
            .iter()
            .filter(|&&w| robot_positions.contains(&w))
            .count() as i32;

        // Helper to count walls in range [lo, hi] using binary search
        let count_walls = |lo: i64, hi: i64| -> i32 {
            if lo > hi {
                return 0;
            }
            let left = walls.partition_point(|&w| w < lo);
            let right = walls.partition_point(|&w| w <= hi);
            (right - left) as i32
        };

        if n == 1 {
            let (p, d) = robot_data[0];
            let left_outside = count_walls(p - d, p - 1);
            let right_outside = count_walls(p + 1, p + d);
            return walls_at_robots + left_outside.max(right_outside);
        }

        // Walls left of leftmost robot (only reachable by robot 0 firing left)
        let left_outside = count_walls(robot_data[0].0 - robot_data[0].1, robot_data[0].0 - 1);

        // Walls right of rightmost robot (only reachable by robot n-1 firing right)
        let right_outside = count_walls(
            robot_data[n - 1].0 + 1,
            robot_data[n - 1].0 + robot_data[n - 1].1,
        );

        // DP: dp[i][0] = robot i fires left, dp[i][1] = robot i fires right
        let mut dp = vec![[0i32; 2]; n];
        dp[0][0] = left_outside;
        dp[0][1] = 0;

        for i in 1..n {
            let (p1, d1) = robot_data[i - 1];
            let (p2, d2) = robot_data[i];

            // Robot i-1 fires right: covers [p1 + 1, min(p1 + d1, p2 - 1)]
            let r1_right = (p1 + d1).min(p2 - 1);

            // Robot i fires left: covers [max(p1 + 1, p2 - d2), p2 - 1]
            let l2_left = (p1 + 1).max(p2 - d2);

            // A = walls only by robot i-1 firing right (not by robot i firing left)
            let a = if l2_left > p1 + 1 {
                count_walls(p1 + 1, r1_right.min(l2_left - 1))
            } else {
                0
            };

            // B = walls only by robot i firing left (not by robot i-1 firing right)
            let b = if r1_right < p2 - 1 {
                count_walls(l2_left.max(r1_right + 1), p2 - 1)
            } else {
                0
            };

            // C = walls reachable by both options
            let c_lo = (p1 + 1).max(l2_left);
            let c_hi = r1_right.min(p2 - 1);
            let c = count_walls(c_lo, c_hi);

            // Transition for robot i firing left:
            // From i-1 left: gain B + C (only i covers interval)
            // From i-1 right: gain A + B + C (all walls in interval)
            dp[i][0] = (dp[i - 1][0] + b + c).max(dp[i - 1][1] + a + b + c);

            // Transition for robot i firing right:
            // From i-1 left: gain 0 (neither covers this interval)
            // From i-1 right: gain A + C (only i-1 covers)
            dp[i][1] = dp[i - 1][0].max(dp[i - 1][1] + a + c);
        }

        walls_at_robots + dp[n - 1][0].max(dp[n - 1][1] + right_outside)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_robot_fires_left() {
        // Robot at 4 with distance 3 can reach walls in [1, 7]
        // Wall at 1 is reachable by firing left, wall at 10 is not
        assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1);
    }

    #[test]
    fn test_two_robots_optimal_directions() {
        // Robot 0 at 2 (dist 1): left [1,2], right [2,3]
        // Robot 1 at 10 (dist 5): left [5,10], right [10,15]
        // Wall at 2 is at robot position (always counted)
        // Walls at 5, 7 are in robot 1's left range
        assert_eq!(
            Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]),
            3
        );
    }

    #[test]
    fn test_blocking_prevents_destruction() {
        // Robot at 1 (dist 100) blocked by robot at 2 when firing right
        // Wall at 10 is unreachable by either robot
        assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
    }

    #[test]
    fn test_wall_at_robot_position() {
        // Wall at same position as robot is always destroyed
        assert_eq!(Solution::max_walls(vec![5], vec![1], vec![5]), 1);
    }

    #[test]
    fn test_multiple_walls_in_range() {
        // Robot at 10 with distance 5: left covers [5,9], right covers [11,15]
        // Firing left: walls at 5, 7, 8 = 3 walls
        // Firing right: walls at 12, 15 = 2 walls
        // Maximum is 3 (firing left)
        assert_eq!(
            Solution::max_walls(vec![10], vec![5], vec![5, 7, 8, 12, 15]),
            3
        );
    }

    #[test]
    fn test_overlapping_ranges() {
        // Two robots with overlapping potential ranges
        // Robot 0 at 5 (dist 3): left [2,5], right [5,8]
        // Robot 1 at 10 (dist 4): left [6,10], right [10,14]
        // Walls at 3, 7, 12
        // Optimal: robot 0 left (gets 3), robot 1 left (gets 7), no one gets 12
        // Or: robot 0 right (gets 7), robot 1 right (gets 12), no one gets 3
        assert_eq!(
            Solution::max_walls(vec![5, 10], vec![3, 4], vec![3, 7, 12]),
            2
        );
    }

    #[test]
    fn test_three_robots() {
        // Robots at 5, 10, 15 with distance 3 each
        // Wall 2: only by robot 0 left
        // Wall 18: only by robot 2 right
        // Walls 7, 12: need robot 1 to cover, but it can only fire one direction
        // Optimal: robot 0 left (wall 2), robot 1 left (wall 7), robot 2 right (wall 18) = 3
        assert_eq!(
            Solution::max_walls(vec![5, 10, 15], vec![3, 3, 3], vec![2, 7, 12, 18]),
            3
        );
    }

    #[test]
    fn test_no_walls_in_range() {
        // All walls are out of range
        assert_eq!(
            Solution::max_walls(vec![50], vec![5], vec![1, 2, 100, 200]),
            0
        );
    }
}
