impl Solution {
    /// Simulates asteroid collisions using a stack.
    ///
    /// # Intuition
    /// Collisions only happen when a right-moving asteroid meets a left-moving
    /// one. Use a stack to track surviving asteroids, resolving collisions as
    /// new left-moving asteroids are encountered.
    ///
    /// # Approach
    /// Iterate through asteroids. Push right-moving ones directly. For
    /// left-moving ones, pop smaller right-moving asteroids from the stack.
    /// If sizes are equal, both are destroyed. If the stack is empty or the
    /// top is also left-moving, push the current asteroid.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the stack
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::with_capacity(asteroids.len());
        for &asteroid in &asteroids {
            if asteroid > 0 {
                stack.push(asteroid);
            } else {
                while stack.last().is_some_and(|&top| top > 0 && top < -asteroid) {
                    stack.pop();
                }
                if stack.last().is_some_and(|&top| top == -asteroid) {
                    stack.pop();
                } else if !stack.last().is_some_and(|&top| top > 0) {
                    stack.push(asteroid);
                }
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_wins() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn test_mutual_destruction() {
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    }

    #[test]
    fn test_left_survives() {
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }

    #[test]
    fn test_no_collision() {
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
    }

    #[test]
    fn test_chain_destruction() {
        assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, -4]), vec![-4]);
    }
}
