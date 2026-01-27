impl Solution {
    /// Finds the nearest valid point sharing x or y coordinate.
    ///
    /// # Intuition
    /// A point is valid if it shares either the x or y coordinate. Among valid
    /// points, return the index with the smallest Manhattan distance, preferring
    /// the smallest index on ties.
    ///
    /// # Approach
    /// 1. Iterate with index, filtering points that share x or y.
    /// 2. Compute Manhattan distance for valid points.
    /// 3. Track the minimum distance and its index.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .enumerate()
            .filter(|(_, p)| p[0] == x || p[1] == y)
            .map(|(i, p)| (i, (p[0] - x).abs() + (p[1] - y).abs()))
            .min_by_key(|&(_, dist)| dist)
            .map_or(-1, |(i, _)| i as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        assert_eq!(Solution::nearest_valid_point(3, 4, points), 2);
    }

    #[test]
    fn test_no_valid() {
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
    }
}
