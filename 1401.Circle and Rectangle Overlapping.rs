impl Solution {
    /// Closest-point clamp: overlap iff the nearest rectangle point is within the radius.
    ///
    /// # Intuition
    /// An axis-aligned rectangle and a circle intersect if and only if the
    /// closest point of the rectangle to the circle center lies inside or on
    /// the circle. That closest point is obtained by independently clamping
    /// the center's coordinates to the rectangle's bounds.
    ///
    /// # Approach
    /// 1. Clamp `x_center` into `[x1, x2]` and `y_center` into `[y1, y2]`.
    /// 2. Compare the squared Euclidean distance from the center to that
    ///    point against `radius²`, using integer arithmetic to avoid floats.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let closest_x = x_center.clamp(x1, x2);
        let closest_y = y_center.clamp(y1, y2);
        let dx = closest_x - x_center;
        let dy = closest_y - y_center;
        dx * dx + dy * dy <= radius * radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::check_overlap(1, 1, 1, 1, -3, 2, -1));
    }

    #[test]
    fn test_example_3() {
        assert!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1));
    }

    #[test]
    fn test_center_inside_rectangle() {
        assert!(Solution::check_overlap(1, 5, 5, 0, 0, 10, 10));
    }

    #[test]
    fn test_rectangle_inside_circle() {
        assert!(Solution::check_overlap(10, 0, 0, 3, 3, 4, 4));
    }

    #[test]
    fn test_circle_inside_rectangle() {
        assert!(Solution::check_overlap(2, 5, 5, 0, 0, 10, 10));
    }

    #[test]
    fn test_touches_corner_exactly() {
        // Closest corner is (3, 4); 3² + 4² = 25 = radius².
        assert!(Solution::check_overlap(5, 0, 0, 3, 4, 10, 10));
    }

    #[test]
    fn test_misses_corner() {
        assert!(!Solution::check_overlap(4, 0, 0, 3, 4, 10, 10));
    }

    #[test]
    fn test_touches_edge() {
        assert!(Solution::check_overlap(2, 0, 0, 2, -1, 4, 1));
    }

    #[test]
    fn test_no_overlap_far_away() {
        assert!(!Solution::check_overlap(1, 0, 0, 100, 100, 200, 200));
    }

    #[test]
    fn test_negative_quadrant() {
        assert!(Solution::check_overlap(3, -5, -5, -8, -8, -4, -4));
    }

    #[test]
    fn test_max_constraints_no_overlap() {
        assert!(!Solution::check_overlap(
            1, -10_000, -10_000, 9_999, 9_999, 10_000, 10_000
        ));
    }

    #[test]
    fn test_max_radius_covers_distant_rect() {
        assert!(Solution::check_overlap(2000, 0, 0, 1000, 1000, 1001, 1001));
    }
}
