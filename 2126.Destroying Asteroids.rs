impl Solution {
    /// Determines if all asteroids can be destroyed by absorbing them greedily.
    ///
    /// # Intuition
    /// By sorting asteroids in ascending order, we always attempt the smallest
    /// first. If the planet's mass is at least as large as an asteroid, it
    /// absorbs it and grows. If any asteroid exceeds the current mass, we fail.
    ///
    /// # Approach
    /// 1. Sort asteroids in ascending order using `sort_unstable`.
    /// 2. Use `i64` to avoid overflow when accumulating mass.
    /// 3. Iterate through sorted asteroids, returning false if mass is
    ///    insufficient; otherwise absorb and continue.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) (in-place sort)
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        asteroids.sort_unstable();
        asteroids
            .iter()
            .try_fold(mass as i64, |acc, &asteroid| {
                if acc < asteroid as i64 {
                    None
                } else {
                    Some(acc + asteroid as i64)
                }
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_destroy_all() {
        assert!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
    }

    #[test]
    fn test_cannot_destroy() {
        assert!(!Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]));
    }

    #[test]
    fn test_single_asteroid() {
        assert!(Solution::asteroids_destroyed(10, vec![5]));
    }

    #[test]
    fn test_mass_equals_asteroid() {
        assert!(Solution::asteroids_destroyed(1, vec![1]));
    }
}
