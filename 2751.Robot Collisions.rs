impl Solution {
    /// Stack-based collision simulation in position order.
    ///
    /// # Intuition
    /// Collisions only happen between a right-moving robot and the next left-moving robot to its
    /// right. Sorting by position lets us resolve collisions by only looking back at the nearest
    /// unresolved right-moving robots.
    ///
    /// # Approach
    /// Sort robots by position and keep a stack of indices for right-moving robots. When a
    /// left-moving robot arrives, repeatedly collide it with the stack top:
    /// - Lower health robot is removed.
    /// - Survivor loses one health.
    /// - Equal health removes both.
    /// Update the original health array in place and collect remaining healths in input order.
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting, O(n) for collisions.
    /// - Space: O(n)
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        if n == 0 {
            return Vec::new();
        }

        let dir_bytes = directions.as_bytes();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&idx| positions[idx]);

        let mut healths = healths;
        let mut right_stack: Vec<usize> = Vec::with_capacity(n);

        for &idx in &order {
            if dir_bytes[idx] == b'R' {
                right_stack.push(idx);
                continue;
            }

            let mut current_health = healths[idx];
            while current_health > 0 {
                let Some(&right_idx) = right_stack.last() else {
                    break;
                };
                let right_health = healths[right_idx];

                if right_health < current_health {
                    right_stack.pop();
                    healths[right_idx] = 0;
                    current_health -= 1;
                    continue;
                }

                if right_health == current_health {
                    right_stack.pop();
                    healths[right_idx] = 0;
                    current_health = 0;
                    break;
                }

                healths[right_idx] = right_health - 1;
                current_health = 0;
                break;
            }

            healths[idx] = current_health;
        }

        healths.into_iter().filter(|&health| health > 0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let positions = vec![5, 4, 3, 2, 1];
        let healths = vec![2, 17, 9, 15, 10];
        let directions = "RRRRR".to_string();
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![2, 17, 9, 15, 10]
        );
    }

    #[test]
    fn example_two() {
        let positions = vec![3, 5, 2, 6];
        let healths = vec![10, 10, 15, 12];
        let directions = "RLRL".to_string();
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![14]
        );
    }

    #[test]
    fn example_three() {
        let positions = vec![1, 2, 5, 6];
        let healths = vec![10, 10, 11, 11];
        let directions = "RLRL".to_string();
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn chain_collisions() {
        let positions = vec![1, 3, 5, 7];
        let healths = vec![4, 2, 3, 1];
        let directions = "RRLL".to_string();
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![2]
        );
    }

    #[test]
    fn moving_away_no_collision() {
        let positions = vec![1, 2];
        let healths = vec![5, 6];
        let directions = "LR".to_string();
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![5, 6]
        );
    }
}
