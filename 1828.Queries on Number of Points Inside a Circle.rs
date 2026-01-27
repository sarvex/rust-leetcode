impl Solution {
    /// Counts points inside each query circle using brute-force distance checks.
    ///
    /// # Intuition
    /// For each circle, count points whose squared distance from the center
    /// does not exceed the squared radius, avoiding floating-point arithmetic.
    ///
    /// # Approach
    /// 1. For each query (cx, cy, r), compute r squared.
    /// 2. Count points where (px - cx)^2 + (py - cy)^2 <= r^2.
    ///
    /// # Complexity
    /// - Time: O(q * p) where q = queries, p = points
    /// - Space: O(q)
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .iter()
            .map(|q| {
                let (cx, cy, r2) = (q[0], q[1], q[2] * q[2]);
                points
                    .iter()
                    .filter(|p| (p[0] - cx).pow(2) + (p[1] - cy).pow(2) <= r2)
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_queries() {
        assert_eq!(
            Solution::count_points(
                vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
                vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
            ),
            vec![3, 2, 2]
        );
    }

    #[test]
    fn test_no_points_in_circle() {
        assert_eq!(
            Solution::count_points(vec![vec![1, 1]], vec![vec![10, 10, 1]]),
            vec![0]
        );
    }
}
