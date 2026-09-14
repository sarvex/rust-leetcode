impl Solution {
    /// Per-axis interval intersection test.
    ///
    /// # Intuition
    /// An axis-aligned rectangle is the Cartesian product of an x-interval and a
    /// y-interval, so the intersection of two rectangles is the product of the two
    /// interval intersections. That product has positive area exactly when both
    /// factors have positive length, which reduces the geometry question to two
    /// independent 1-D overlap checks. Intervals `[a1, a2]` and `[b1, b2]` overlap
    /// with positive length iff `a1 < b2 && b1 < a2`; the strict `<` is what
    /// rejects rectangles that merely touch along an edge or at a corner.
    ///
    /// # Approach
    /// The layout `[x1, y1, x2, y2]` puts each axis's low bound at index `axis` and
    /// its high bound at `axis + 2`, so one comparison serves both axes: run it for
    /// `axis` in `0..2` and require both to hold. No arithmetic is performed on the
    /// coordinates, so the `±10^9` range cannot overflow `i32`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Panics if either rectangle has fewer than four coordinates.
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        (0..2).all(|axis| rec1[axis] < rec2[axis + 2] && rec2[axis] < rec1[axis + 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_diagonal_overlap() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![1, 1, 3, 3]
        ));
    }

    #[test]
    fn example_two_shared_edge_is_not_overlap() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1]
        ));
    }

    #[test]
    fn example_three_disjoint() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3]
        ));
    }

    #[test]
    fn shared_corner_is_not_overlap() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 1, 2, 2]
        ));
    }

    #[test]
    fn shared_horizontal_edge_is_not_overlap() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 2, 1],
            vec![0, 1, 2, 2]
        ));
    }

    #[test]
    fn containment_counts_as_overlap() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 10, 10],
            vec![3, 4, 5, 6]
        ));
        assert!(Solution::is_rectangle_overlap(
            vec![3, 4, 5, 6],
            vec![0, 0, 10, 10]
        ));
    }

    #[test]
    fn identical_rectangles_overlap() {
        assert!(Solution::is_rectangle_overlap(
            vec![-4, -3, 7, 5],
            vec![-4, -3, 7, 5]
        ));
    }

    #[test]
    fn overlap_on_x_axis_only_is_rejected() {
        // Columns line up but rows are far apart: one axis alone is not enough.
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 5, 1],
            vec![1, 8, 4, 9]
        ));
    }

    #[test]
    fn overlap_on_y_axis_only_is_rejected() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 5],
            vec![8, 1, 9, 4]
        ));
    }

    #[test]
    fn negative_coordinates_overlap() {
        assert!(Solution::is_rectangle_overlap(
            vec![-5, -5, -1, -1],
            vec![-3, -3, 4, 4]
        ));
    }

    #[test]
    fn sliver_overlap_of_unit_width() {
        // Intersection is 1 x 1: the smallest positive area possible here.
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 3, 3],
            vec![2, 2, 6, 6]
        ));
    }

    #[test]
    fn cross_shape_overlaps_without_containment() {
        // A tall thin rectangle crossing a wide flat one; neither contains a corner
        // of the other, yet both projections intersect.
        assert!(Solution::is_rectangle_overlap(
            vec![0, 2, 10, 4],
            vec![3, 0, 5, 9]
        ));
    }

    #[test]
    fn boundary_extremes_span_full_range() {
        assert!(Solution::is_rectangle_overlap(
            vec![-1_000_000_000, -1_000_000_000, 0, 0],
            vec![-1, -1, 1_000_000_000, 1_000_000_000]
        ));
    }

    #[test]
    fn boundary_extremes_touch_at_origin() {
        // Far-apart magnitudes meeting exactly at (0, 0): touching, not overlapping.
        assert!(!Solution::is_rectangle_overlap(
            vec![-1_000_000_000, -1_000_000_000, 0, 0],
            vec![0, 0, 1_000_000_000, 1_000_000_000]
        ));
    }

    #[test]
    fn symmetric_in_arguments() {
        let cases = [
            (vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
            (vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
            (vec![7, 8, 9, 9], vec![-1, -2, 3, 4]),
        ];
        assert!(cases.iter().all(|(first, second)| {
            Solution::is_rectangle_overlap(first.clone(), second.clone())
                == Solution::is_rectangle_overlap(second.clone(), first.clone())
        }));
    }
}
