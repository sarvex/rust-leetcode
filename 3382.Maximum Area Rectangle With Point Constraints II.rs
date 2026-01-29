use std::collections::{HashMap, HashSet};

impl Solution {
    /// Finds the maximum area rectangle formed by four points with no interior points.
    ///
    /// # Intuition
    /// For a valid rectangle, we need four corner points and no other points strictly
    /// inside or on the edges. Group points by x-coordinate, then for each pair of
    /// x-values, find matching y-coordinate pairs that form potential rectangles.
    ///
    /// # Approach
    /// 1. Group points by x-coordinate into sorted lists of y-values.
    /// 2. For each pair of points sharing same x, record (y1, y2) -> list of x-values.
    /// 3. For y-pairs appearing at multiple x-values, check if they form valid rectangles.
    /// 4. Validate no interior points using the point set and coordinate maps.
    /// 5. Track maximum area among all valid rectangles.
    ///
    /// # Complexity
    /// - Time: O(n^2 log n) for pair enumeration and validation
    /// - Space: O(n^2) for storing y-pair to x-value mappings
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        let n = x_coord.len();
        if n < 4 {
            return -1;
        }

        let points: HashSet<(i32, i32)> = x_coord
            .iter()
            .zip(y_coord.iter())
            .map(|(&x, &y)| (x, y))
            .collect();

        // Group y-coordinates by x-coordinate
        let mut x_to_ys: HashMap<i32, Vec<i32>> = HashMap::new();
        for (&x, &y) in x_coord.iter().zip(y_coord.iter()) {
            x_to_ys.entry(x).or_default().push(y);
        }

        // Sort y-values for each x
        for ys in x_to_ys.values_mut() {
            ys.sort_unstable();
        }

        // Map (y1, y2) pairs to list of x-values where both y1 and y2 exist
        let mut y_pair_to_xs: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        for (&x, ys) in &x_to_ys {
            let m = ys.len();
            for i in 0..m {
                for j in i + 1..m {
                    y_pair_to_xs.entry((ys[i], ys[j])).or_default().push(x);
                }
            }
        }

        // Sort x-values for each y-pair
        for xs in y_pair_to_xs.values_mut() {
            xs.sort_unstable();
        }

        let mut max_area: i64 = -1;

        // Check each y-pair for valid rectangles
        for ((y1, y2), xs) in &y_pair_to_xs {
            if xs.len() < 2 {
                continue;
            }

            // Check consecutive x-values for valid rectangles
            for window in xs.windows(2) {
                let (x1, x2) = (window[0], window[1]);

                // Check for points on horizontal edges (excluding corners)
                let has_horizontal_interior = x_to_ys
                    .iter()
                    .any(|(&x, ys)| x > x1 && x < x2 && ys.iter().any(|&y| y == *y1 || y == *y2));

                if has_horizontal_interior {
                    continue;
                }

                // Check for points on vertical edges (excluding corners)
                let y1_ys = x_to_ys.get(&x1).unwrap();
                let y2_ys = x_to_ys.get(&x2).unwrap();

                let has_left_vertical = y1_ys.iter().any(|&y| y > *y1 && y < *y2);
                let has_right_vertical = y2_ys.iter().any(|&y| y > *y1 && y < *y2);

                if has_left_vertical || has_right_vertical {
                    continue;
                }

                // Check for interior points
                let has_interior = points
                    .iter()
                    .any(|&(px, py)| px > x1 && px < x2 && py > *y1 && py < *y2);

                if has_interior {
                    continue;
                }

                let area = (x2 - x1) as i64 * (*y2 - *y1) as i64;
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3], vec![1, 3, 1, 3]),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3, 2], vec![1, 3, 1, 3, 2]),
            -1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3, 1, 3], vec![1, 3, 1, 3, 2, 2]),
            2
        );
    }

    #[test]
    fn test_insufficient_points() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 2, 3], vec![1, 2, 3]),
            -1
        );
    }

    #[test]
    fn test_no_valid_rectangle() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            -1
        );
    }

    #[test]
    fn test_multiple_rectangles() {
        assert_eq!(
            Solution::max_rectangle_area(vec![0, 0, 1, 1, 2, 2], vec![0, 1, 0, 1, 0, 1]),
            1
        );
    }

    #[test]
    fn test_large_rectangle() {
        assert_eq!(
            Solution::max_rectangle_area(vec![0, 0, 10, 10], vec![0, 20, 0, 20]),
            200
        );
    }

    #[test]
    fn test_point_on_horizontal_edge() {
        assert_eq!(
            Solution::max_rectangle_area(
                vec![
                    32, 100, 32, 100, 85, 12, 59, 84, 3, 68, 31, 75, 87, 83, 30, 22, 85, 71, 92, 69
                ],
                vec![0, 11, 11, 0, 42, 83, 80, 0, 18, 43, 18, 25, 12, 62, 94, 27, 76, 19, 14, 33]
            ),
            -1
        );
    }
}
