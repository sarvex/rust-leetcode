impl Solution {
    /// Finds the k closest points to the origin by sorting on distance.
    ///
    /// # Intuition
    /// Sort points by squared distance (avoids sqrt) and take the first k.
    ///
    /// # Approach
    /// Sort by `x^2 + y^2` using `sort_unstable_by_key`, then truncate.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(log n) for sorting
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points.truncate(k as usize);
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        assert_eq!(result, vec![vec![-2, 2]]);
    }

    #[test]
    fn test_two_closest() {
        let mut result = Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        result.sort();
        let mut expected = vec![vec![-2, 4], vec![3, 3]];
        expected.sort();
        assert_eq!(result, expected);
    }
}
