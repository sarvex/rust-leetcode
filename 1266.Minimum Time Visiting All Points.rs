impl Solution {
    /// Calculate the minimum time to visit all points in order.
    ///
    /// # intuition
    /// To move between two points (x1, y1) and (x2, y2), the minimum time is the maximum of the
    /// absolute differences between their coordinates, because we can move diagonally. This is
    /// known as the Chebyshev distance.
    ///
    /// # approach
    /// Use an iterator over windows of size 2 to process consecutive points. For each pair of
    /// points, calculate the Chebyshev distance and sum them up.
    ///
    /// # complexity
    /// - Time complexity: O(n) where n is the number of points.
    /// - Space complexity: O(1) as we iterate in place.
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|w| (w[0][0] - w[1][0]).abs().max((w[0][1] - w[1][1]).abs()))
            .sum()
    }
}
