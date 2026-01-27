impl Solution {
    /// Chebyshev distance sum over consecutive point pairs.
    ///
    /// # Intuition
    /// Moving diagonally covers both x and y distance simultaneously, so the
    /// minimum time between two points equals the Chebyshev distance:
    /// `max(|dx|, |dy|)`. Summing over consecutive pairs gives the total.
    ///
    /// # Approach
    /// 1. Use windows of size 2 over the points array
    /// 2. For each pair, compute max of absolute coordinate differences
    /// 3. Sum all Chebyshev distances
    ///
    /// # Complexity
    /// - Time: O(n) single pass over points
    /// - Space: O(1)
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|w| (w[0][0] - w[1][0]).abs().max((w[0][1] - w[1][1]).abs()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_points() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
    }

    #[test]
    fn collinear_points() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }

    #[test]
    fn single_point() {
        assert_eq!(Solution::min_time_to_visit_all_points(vec![vec![0, 0]]), 0);
    }
}
