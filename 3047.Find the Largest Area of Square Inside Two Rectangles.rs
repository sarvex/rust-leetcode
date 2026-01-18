impl Solution {
    /// Find the Largest Area of Square Inside Two Rectangles
    ///
    /// # Intuition
    /// For any two rectangles to have an intersection, their overlapping region forms
    /// another rectangle. The largest square that fits in this intersection has a side
    /// length equal to the minimum of the intersection's width and height.
    ///
    /// # Approach
    /// 1. Iterate through all pairs of rectangles
    /// 2. For each pair, compute the intersection rectangle by taking:
    ///    - max of left edges for the intersection's left edge
    ///    - max of bottom edges for the intersection's bottom edge
    ///    - min of right edges for the intersection's right edge
    ///    - min of top edges for the intersection's top edge
    /// 3. If the intersection is valid (positive width and height), calculate the
    ///    maximum square side as min(width, height)
    /// 4. Track the maximum side length found across all pairs
    ///
    /// # Complexity
    /// - Time: O(n²) where n is the number of rectangles
    /// - Space: O(1) auxiliary space
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();

        (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .filter_map(|(i, j)| {
                let inter_left = bottom_left[i][0].max(bottom_left[j][0]);
                let inter_bottom = bottom_left[i][1].max(bottom_left[j][1]);
                let inter_right = top_right[i][0].min(top_right[j][0]);
                let inter_top = top_right[i][1].min(top_right[j][1]);

                let width = inter_right - inter_left;
                let height = inter_top - inter_bottom;

                if width > 0 && height > 0 {
                    Some(width.min(height) as i64)
                } else {
                    None
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
    fn example_1() {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![3, 1]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![6, 6]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 1);
    }

    #[test]
    fn example_2() {
        let bottom_left = vec![vec![1, 1], vec![1, 3], vec![1, 5]];
        let top_right = vec![vec![5, 5], vec![5, 7], vec![5, 9]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 4);
    }

    #[test]
    fn example_3() {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![1, 2]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![3, 4]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 1);
    }

    #[test]
    fn example_4() {
        let bottom_left = vec![vec![1, 1], vec![3, 3], vec![3, 1]];
        let top_right = vec![vec![2, 2], vec![4, 4], vec![4, 2]];
        assert_eq!(Solution::largest_square_area(bottom_left, top_right), 0);
    }
}
