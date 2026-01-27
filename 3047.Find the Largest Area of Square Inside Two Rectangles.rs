impl Solution {
    /// Largest square area within the intersection of two rectangles.
    ///
    /// # Intuition
    /// For any two rectangles to have an intersection, their overlapping region forms
    /// another rectangle. The largest square that fits in this intersection has a side
    /// length equal to the minimum of the intersection's width and height.
    ///
    /// # Approach
    /// 1. Iterate through all pairs of rectangles.
    /// 2. For each pair, compute the intersection rectangle by taking max of left/bottom
    ///    edges and min of right/top edges.
    /// 3. If the intersection is valid (positive width and height), calculate the
    ///    maximum square side as min(width, height).
    /// 4. Track the maximum side length found across all pairs.
    ///
    /// # Complexity
    /// - Time: O(n^2) where n is the number of rectangles
    /// - Space: O(1)
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();

        (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .filter_map(|(i, j)| {
                let width =
                    top_right[i][0].min(top_right[j][0]) - bottom_left[i][0].max(bottom_left[j][0]);
                let height =
                    top_right[i][1].min(top_right[j][1]) - bottom_left[i][1].max(bottom_left[j][1]);

                match width > 0 && height > 0 {
                    true => Some(width.min(height) as i64),
                    false => None,
                }
            })
            .max()
            .map(|side| side * side)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_rectangles_small_square() {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![3, 1]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![6, 6]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 1);
    }

    #[test]
    fn overlapping_rectangles_larger_square() {
        let bottom_left = vec![vec![1, 1], vec![1, 3], vec![1, 5]];
        let top_right = vec![vec![5, 5], vec![5, 7], vec![5, 9]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 4);
    }

    #[test]
    fn partial_overlap_returns_unit_square() {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![1, 2]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![3, 4]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 1);
    }

    #[test]
    fn no_overlap_returns_zero() {
        let bottom_left = vec![vec![1, 1], vec![3, 3], vec![3, 1]];
        let top_right = vec![vec![2, 2], vec![4, 4], vec![4, 2]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 0);
    }
}
