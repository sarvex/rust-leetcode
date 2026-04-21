impl Solution {
    /// DP assignment of sorted robots to sorted factories.
    ///
    /// # Intuition
    /// After sorting both robots and factories by position, an optimal
    /// assignment never crosses — if robot `a < b` both go to factories
    /// `f1 < f2`, then `a` goes to `f1` and `b` goes to `f2`. This lets
    /// us solve with a sequential DP over sorted positions.
    ///
    /// # Approach
    /// Sort robots and factories by position. Define `dp[i][j]` as the
    /// minimum total distance to assign the first `i` robots using the
    /// first `j` factories. For each factory `j`, try assigning `k` robots
    /// (from 0 up to its limit) ending at robot `i`, accumulating distances
    /// from the rightmost assigned robot backwards.
    ///
    /// # Complexity
    /// - Time: O(n × m × L) where n = robots, m = factories, L = max limit
    /// - Space: O(n × m) for the DP table
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable_by_key(|f| f[0]);

        let n = robot.len();
        let m = factory.len();

        let mut dp = vec![vec![i64::MAX; m + 1]; n + 1];
        dp[0][0] = 0;

        for j in 0..=m {
            dp[0][j] = 0;
        }

        for j in 1..=m {
            let pos = factory[j - 1][0] as i64;
            let limit = factory[j - 1][1] as usize;

            for i in 0..=n {
                dp[i][j] = dp[i][j - 1];

                let mut cost = 0i64;
                for k in 1..=limit.min(i) {
                    cost += (robot[i - k] as i64 - pos).abs();
                    if dp[i - k][j - 1] < i64::MAX {
                        dp[i][j] = dp[i][j].min(dp[i - k][j - 1] + cost);
                    }
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_robots_two_factories() {
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        );
    }

    #[test]
    fn two_robots_opposite_directions() {
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
            2
        );
    }

    #[test]
    fn single_robot_single_factory() {
        assert_eq!(
            Solution::minimum_total_distance(vec![5], vec![vec![10, 1]]),
            5
        );
    }

    #[test]
    fn robot_at_factory_position() {
        assert_eq!(
            Solution::minimum_total_distance(vec![3], vec![vec![3, 1]]),
            0
        );
    }

    #[test]
    fn all_robots_same_factory() {
        assert_eq!(
            Solution::minimum_total_distance(vec![1, 2, 3], vec![vec![0, 3]]),
            6
        );
    }

    #[test]
    fn large_coordinates() {
        assert_eq!(
            Solution::minimum_total_distance(vec![-1_000_000_000, 1_000_000_000], vec![vec![0, 2]]),
            2_000_000_000
        );
    }
}
