
impl Solution {
    /// Checks if three points form a valid boomerang (non-collinear).
    ///
    /// # Intuition
    /// Three points are collinear if the cross product of vectors between
    /// them is zero. Non-zero cross product means a valid boomerang.
    ///
    /// # Approach
    /// Compute `(x1-x2)*(y2-y3) - (x2-x3)*(y1-y2)`. Non-zero means
    /// the points are not collinear.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let (x1, y1) = (points[0][0], points[0][1]);
        let (x2, y2) = (points[1][0], points[1][1]);
        let (x3, y3) = (points[2][0], points[2][1]);
        (x1 - x2) * (y2 - y3) != (x2 - x3) * (y1 - y2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 3],
            vec![3, 2]
        ]));
    }

    #[test]
    fn test_collinear() {
        assert!(!Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3]
        ]));
    }
}
