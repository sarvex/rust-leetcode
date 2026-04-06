use std::collections::HashSet;

impl Solution {
    /// Simulates robot movement on a grid, tracking maximum squared distance from origin.
    ///
    /// # Intuition
    /// The robot follows a fixed set of directions (N, E, S, W) and turns modify the
    /// current heading. By encoding obstacles in a HashSet with a single `i64` key,
    /// we achieve O(1) collision checks per step. Since each move command is at most
    /// 9 steps, the total work is bounded by 9 × |commands|.
    ///
    /// # Approach
    /// 1. Encode all obstacle coordinates into a `HashSet<i64>` using bit-shifted keys.
    /// 2. Maintain a direction index into `[(0,1), (1,0), (0,-1), (-1,0)]` for N, E, S, W.
    /// 3. For turn commands, adjust the direction index modulo 4.
    /// 4. For move commands, advance one unit at a time, stopping before any obstacle.
    /// 5. After each move command, update the maximum squared Euclidean distance.
    ///
    /// # Complexity
    /// - Time: O(n × k + m) where n = commands.len(), k ≤ 9, m = obstacles.len()
    /// - Space: O(m) for the obstacle set
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacle_set: HashSet<i64> = obstacles.iter().map(|o| encode(o[0], o[1])).collect();

        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut x = 0i32;
        let mut y = 0i32;
        let mut dir = 0usize;
        let mut result = 0i32;

        for cmd in &commands {
            match *cmd {
                -2 => dir = (dir + 3) % 4,
                -1 => dir = (dir + 1) % 4,
                steps => {
                    let (dx, dy) = DIRS[dir];
                    for _ in 0..steps {
                        let nx = x + dx;
                        let ny = y + dy;
                        if obstacle_set.contains(&encode(nx, ny)) {
                            break;
                        }
                        x = nx;
                        y = ny;
                    }
                    result = result.max(x * x + y * y);
                }
            }
        }

        result
    }
}

/// Encodes two `i32` coordinates into a single `i64` key for O(1) hashing.
fn encode(x: i32, y: i32) -> i64 {
    ((x as i64) << 32) | (y as u32 as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_obstacles() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
    }

    #[test]
    fn test_blocked_by_obstacle() {
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
            65
        );
    }

    #[test]
    fn test_obstacle_at_origin() {
        assert_eq!(
            Solution::robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]),
            36
        );
    }

    #[test]
    fn test_only_turns() {
        assert_eq!(Solution::robot_sim(vec![-2, -1, -2, -1], vec![]), 0);
    }

    #[test]
    fn test_single_step() {
        assert_eq!(Solution::robot_sim(vec![1], vec![]), 1);
    }

    #[test]
    fn test_full_circle() {
        assert_eq!(
            Solution::robot_sim(vec![3, -1, 3, -1, 3, -1, 3], vec![]),
            18
        );
    }

    #[test]
    fn test_immediate_obstacle() {
        assert_eq!(Solution::robot_sim(vec![5], vec![vec![0, 1]]), 0);
    }
}
