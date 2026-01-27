impl Solution {
    /// Minimum rectangles of width w to cover all points (height is unlimited).
    ///
    /// # Intuition
    /// Only the x-coordinates matter since rectangles have unlimited height.
    /// Sort points by x, then greedily place each rectangle starting at the
    /// leftmost uncovered point.
    ///
    /// # Approach
    /// 1. Sort points by their x-coordinate.
    /// 2. Iterate through sorted points; when a point exceeds the current
    ///    rectangle's right edge, start a new rectangle.
    /// 3. Return the rectangle count.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary (sort is in-place)
    pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
        points.sort_unstable_by_key(|p| p[0]);

        points
            .iter()
            .fold((0, -1), |(count, right_edge), p| match p[0] > right_edge {
                true => (count + 1, p[0] + w),
                false => (count, right_edge),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_rectangles_needed() {
        let points = vec![
            vec![2, 1],
            vec![1, 0],
            vec![1, 4],
            vec![1, 8],
            vec![3, 5],
            vec![4, 6],
        ];
        assert_eq!(Solution::min_rectangles_to_cover_points(points, 1), 2);
    }

    #[test]
    fn single_rectangle_covers_all() {
        let points = vec![vec![0, 0], vec![1, 1], vec![2, 2]];
        assert_eq!(Solution::min_rectangles_to_cover_points(points, 5), 1);
    }
}
