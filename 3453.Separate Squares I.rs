impl Solution {
    /// Sweep line to find horizontal line splitting square areas equally.
    ///
    /// # Intuition
    /// Sweep a horizontal line upward through all critical y-coordinates
    /// (bottom and top edges of squares). Track the total width of squares
    /// intersecting the line. When the cumulative area below reaches half
    /// the total area, interpolate to find the exact y-coordinate.
    ///
    /// # Approach
    /// 1. Create events at each square's bottom edge (+width) and top edge (-width).
    /// 2. Sort events by y-coordinate and process from bottom to top.
    /// 3. For each event, accumulate `covered_width * height_delta`. When cumulative
    ///    area would exceed half the total, linearly interpolate the answer.
    ///
    /// # Complexity
    /// - Time: O(n log n) — sorting events dominates
    /// - Space: O(n) — events vector
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        // Calculate total area and create sweep line events
        let (total_area, mut events) = squares.iter().fold(
            (0_i64, Vec::with_capacity(squares.len() * 2)),
            |(area, mut events), sq| {
                let y = sq[1];
                let length = sq[2];
                events.push((y, length, 1)); // Bottom edge: start contributing width
                events.push((y + length, length, -1)); // Top edge: stop contributing width
                (area + length as i64 * length as i64, events)
            },
        );

        // Sort events by y-coordinate
        events.sort_unstable_by_key(|&(y, _, _)| y);

        let half_area = total_area as f64 * 0.5;
        let mut covered_width = 0.0_f64;
        let mut curr_area = 0.0_f64;
        let mut prev_y = 0.0_f64;

        for (y, length, delta) in events {
            let y = y as f64;
            let height_delta = y - prev_y;
            let area_delta = covered_width * height_delta;

            // Check if we cross the half-area threshold in this segment
            if curr_area + area_delta >= half_area {
                // Linear interpolation to find exact y-coordinate
                return prev_y + (half_area - curr_area) / covered_width;
            }

            // Update state for next iteration
            covered_width += (delta * length) as f64;
            curr_area += area_delta;
            prev_y = y;
        }

        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests two non-overlapping squares with simple geometry
    #[test]
    fn test_example_1() {
        let squares = vec![vec![0, 0, 1], vec![2, 2, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    /// Tests overlapping squares requiring fractional y-coordinate
    #[test]
    fn test_example_2() {
        let squares = vec![vec![0, 0, 2], vec![1, 1, 1]];
        let result = Solution::separate_squares(squares);
        let expected = 7.0 / 6.0;
        assert!((result - expected).abs() < 1e-5);
    }

    /// Tests single square (baseline case)
    #[test]
    fn test_single_square() {
        let squares = vec![vec![0, 0, 2]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    /// Tests two non-overlapping squares with gap between them
    #[test]
    fn test_non_overlapping_squares() {
        let squares = vec![vec![0, 0, 2], vec![5, 5, 2]];
        let result = Solution::separate_squares(squares);
        assert!((result - 2.0).abs() < 1e-5);
    }

    /// Tests vertically stacked squares
    #[test]
    fn test_vertically_aligned_squares() {
        let squares = vec![vec![0, 0, 1], vec![0, 1, 1], vec![0, 2, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.5).abs() < 1e-5);
    }

    /// Tests edge case with large coordinates (10^9 scale)
    #[test]
    fn test_large_coordinates() {
        let squares = vec![vec![0, 0, 1000000000], vec![0, 1000000000, 1000000000]];
        let result = Solution::separate_squares(squares);
        let expected = 1e9;
        assert!((result - expected).abs() / expected < 1e-5);
    }

    /// Tests completely overlapping squares (area counted multiple times)
    #[test]
    fn test_overlapping_squares_same_position() {
        let squares = vec![vec![0, 0, 2], vec![0, 0, 2]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    /// Tests edge case with many small squares
    #[test]
    fn test_many_small_squares() {
        let squares = (0..10).map(|i| vec![0, i, 1]).collect::<Vec<_>>();
        let result = Solution::separate_squares(squares);
        assert!((result - 5.0).abs() < 1e-5);
    }

    /// Tests partially overlapping squares with complex geometry
    #[test]
    fn test_partial_overlap() {
        let squares = vec![vec![0, 0, 3], vec![0, 1, 2]];
        let result = Solution::separate_squares(squares);
        let expected = 1.7;
        assert!((result - expected).abs() < 1e-5);
    }

    /// Tests boundary condition with minimum size squares
    #[test]
    fn test_minimum_size_squares() {
        let squares = vec![vec![0, 0, 1], vec![0, 1, 1]];
        let result = Solution::separate_squares(squares);
        assert!((result - 1.0).abs() < 1e-5);
    }

    /// Performance test with maximum constraints
    #[test]
    fn test_performance_large_input() {
        let squares: Vec<Vec<i32>> = (0..50000)
            .map(|i| vec![i as i32 % 1000, i as i32 * 100, 100])
            .collect();
        let result = Solution::separate_squares(squares);
        assert!(result > 0.0);
    }
}
