impl Solution {
    /// Greedy: the optimal pair always involves either the first or last house.
    ///
    /// # Intuition
    /// For any valid pair `(i, j)` with different colors, we can always extend
    /// one endpoint to index `0` or `n - 1` without shrinking the distance:
    /// - If `colors[0] != colors[j]`, the pair `(0, j)` is at least as far as `(i, j)`.
    /// - Otherwise `colors[0] == colors[j]`, so `colors[0] == colors[j] != colors[i]`,
    ///   meaning `(i, n - 1)` (when extended to the right endpoint analogously) works.
    ///
    /// So it suffices to scan once from the left looking for the farthest house
    /// whose color differs from `colors[0]`, and once from the right looking for
    /// the farthest house whose color differs from `colors[n - 1]`, then take the max.
    ///
    /// # Approach
    /// 1. Walk from the right end toward the left; the first index `j` with
    ///    `colors[j] != colors[0]` yields distance `j`.
    /// 2. Walk from the left end toward the right; the first index `i` with
    ///    `colors[i] != colors[n - 1]` yields distance `n - 1 - i`.
    /// 3. Return the larger of the two distances. The problem guarantees at
    ///    least one such pair exists, so at least one scan succeeds.
    ///
    /// # Complexity
    /// - Time: O(n) — at most two linear scans over `colors`.
    /// - Space: O(1) — only a handful of scalars.
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let first = colors[0];
        let last = colors[n - 1];

        let from_left = colors
            .iter()
            .rposition(|&c| c != first)
            .map(|j| j as i32)
            .unwrap_or(0);

        let from_right = colors
            .iter()
            .position(|&c| c != last)
            .map(|i| (n - 1 - i) as i32)
            .unwrap_or(0);

        from_left.max(from_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
    }

    #[test]
    fn test_example_three() {
        assert_eq!(Solution::max_distance(vec![0, 1]), 1);
    }

    #[test]
    fn test_two_different_endpoints() {
        // Endpoints already differ — answer is the full span n - 1.
        assert_eq!(Solution::max_distance(vec![5, 5, 5, 5, 9]), 4);
    }

    #[test]
    fn test_different_in_middle_only() {
        // Endpoints are equal (color 7); only index 2 differs.
        // Farther endpoint is index 6, so distance = 6 - 2 = 4.
        assert_eq!(Solution::max_distance(vec![7, 7, 2, 7, 7, 7, 7]), 4);
    }

    #[test]
    fn test_minimum_size_different() {
        assert_eq!(Solution::max_distance(vec![4, 9]), 1);
    }

    #[test]
    fn test_minimum_size_boundary_values() {
        assert_eq!(Solution::max_distance(vec![0, 100]), 1);
    }

    #[test]
    fn test_alternating_colors() {
        // Alternating pattern: endpoints differ, so answer is n - 1.
        assert_eq!(Solution::max_distance(vec![1, 2, 1, 2, 1, 2]), 5);
    }

    #[test]
    fn test_all_same_except_last() {
        assert_eq!(Solution::max_distance(vec![3, 3, 3, 3, 3, 3, 3, 8]), 7);
    }

    #[test]
    fn test_all_same_except_first() {
        assert_eq!(Solution::max_distance(vec![8, 3, 3, 3, 3, 3, 3, 3]), 7);
    }

    #[test]
    fn test_near_maximum_size() {
        // n = 100, only index 50 differs. Farther endpoint is index 99,
        // so expected distance is 99 - 50 = 49.
        let mut colors = vec![1; 100];
        colors[50] = 2;
        assert_eq!(Solution::max_distance(colors), 50);
    }

    #[test]
    fn test_boundary_color_values() {
        assert_eq!(Solution::max_distance(vec![100, 100, 0, 100, 100, 100]), 3);
    }
}
